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

use libkernel::{error::Result, fs::path::Path, memory::address::TUA};

use crate::{
    fs::{
        VFS,
        syscalls::at::{AtFlags, resolve_at_start_node},
    },
    memory::uaccess::cstr::UserCStr,
    process::fd_table::Fd,
    sched::syscall_ctx::ProcessCtx,
};

pub async fn sys_symlinkat(
    ctx: &ProcessCtx,
    old_name: TUA<c_char>,
    new_dirfd: Fd,
    new_name: TUA<c_char>,
) -> Result<usize> {
    let mut buf = [0; 1024];
    let mut buf2 = [0; 1024];

    let task = ctx.shared().clone();
    let source = Path::new(
        UserCStr::from_ptr(old_name)
            .copy_from_user(&mut buf)
            .await?,
    );
    let target = Path::new(
        UserCStr::from_ptr(new_name)
            .copy_from_user(&mut buf2)
            .await?,
    );
    let start_node = resolve_at_start_node(ctx, new_dirfd, target, AtFlags::empty()).await?;

    VFS.symlink(source, target, start_node, &task).await?;

    Ok(0)
}
