/*
 *  ____  ____      _    ____ ___ _____ ____ 
 * |  _ \|  _ \    / \  / ___|_   _/ ___| 
 * | |_) | |_) |  / _ \ \___ \ | | \___ \ 
 * |  __/|  _ <  / ___ \ ___) || |  ___) |
 * |_|   |_| \_\/_/   \____/ |_| |____/ 
 *
 * Zeon Operating System - www.zeon.io
 * 
 * This file is part of Zeon.
 * 
 * Zeon is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Zeon is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Zeon.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Copyright (c) 2015-2024 Zeon Team - All rights reserved
 */
use core::ffi::c_char;

use libkernel::{
    error::Result,
    fs::path::Path,
    memory::address::TUA,
    proc::{
        caps::CapabilitiesFlags,
        ids::{Gid, Uid},
    },
};

use crate::{
    fs::syscalls::at::{AtFlags, resolve_at_start_node, resolve_path_flags},
    memory::uaccess::cstr::UserCStr,
    process::fd_table::Fd,
    sched::syscall_ctx::ProcessCtx,
};

pub async fn sys_fchownat(
    ctx: &ProcessCtx,
    dirfd: Fd,
    path: TUA<c_char>,
    owner: i32,
    group: i32,
    flags: i32,
) -> Result<usize> {
    let mut buf = [0; 1024];

    let task = ctx.shared().clone();
    let flags = AtFlags::from_bits_retain(flags);
    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let start_node = resolve_at_start_node(ctx, dirfd, path, flags).await?;

    let node = resolve_path_flags(dirfd, path, start_node, &task, flags).await?;
    let mut attr = node.getattr().await?;

    {
        let creds = task.creds.lock_save_irq();
        if owner != -1 {
            creds.caps().check_capable(CapabilitiesFlags::CAP_CHOWN)?;
            attr.uid = Uid::new(owner as _);
        }
        if group != -1 {
            let gid = Gid::new(group as _);
            // doesn't seem like there's real groups so this is as good as it gets
            if creds.uid() != attr.uid || creds.gid() != gid {
                creds.caps().check_capable(CapabilitiesFlags::CAP_CHOWN)?;
            }
            attr.gid = gid;
        }
    }
    node.setattr(attr).await?;

    Ok(0)
}
