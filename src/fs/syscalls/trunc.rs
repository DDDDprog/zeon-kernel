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

use crate::{
    fs::VFS, memory::uaccess::cstr::UserCStr, process::fd_table::Fd, sched::syscall_ctx::ProcessCtx,
};
use libkernel::{
    error::{KernelError, Result},
    fs::{OpenFlags, attr::FilePermissions, path::Path},
    memory::address::TUA,
};

pub async fn sys_truncate(ctx: &ProcessCtx, path: TUA<c_char>, new_size: usize) -> Result<usize> {
    let mut buf = [0; 1024];

    let task = ctx.shared().clone();
    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);

    let root = task.root.lock_save_irq().0.clone();
    let file = VFS
        .open(
            path,
            OpenFlags::O_WRONLY,
            root,
            FilePermissions::empty(),
            &task,
        )
        .await?;

    let (ops, ctx) = &mut *file.lock().await;

    ops.truncate(ctx, new_size).await.map(|_| 0)
}

pub async fn sys_ftruncate(ctx: &ProcessCtx, fd: Fd, new_size: usize) -> Result<usize> {
    let fd = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let (ops, ctx) = &mut *fd.lock().await;

    ops.truncate(ctx, new_size).await.map(|_| 0)
}
