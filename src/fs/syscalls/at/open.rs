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
use crate::{
    fs::{VFS, syscalls::at::AtFlags},
    memory::uaccess::cstr::UserCStr,
    process::fd_table::Fd,
    sched::syscall_ctx::ProcessCtx,
};
use core::ffi::c_char;
use libkernel::{
    error::Result,
    fs::{OpenFlags, attr::FilePermissions, path::Path},
    memory::address::TUA,
};

use super::resolve_at_start_node;

pub async fn sys_openat(
    ctx: &ProcessCtx,
    dirfd: Fd,
    path: TUA<c_char>,
    flags: u32,
    mode: u16,
) -> Result<usize> {
    let mut buf = [0; 1024];

    let task = ctx.shared().clone();
    let flags = OpenFlags::from_bits_truncate(flags);
    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let start_node = resolve_at_start_node(ctx, dirfd, path, AtFlags::empty()).await?;
    let mode = FilePermissions::from_bits_retain(mode);

    let file = VFS.open(path, flags, start_node, mode, &task).await?;

    let fd = task.fd_table.lock_save_irq().insert(file)?;

    Ok(fd.as_raw() as _)
}
