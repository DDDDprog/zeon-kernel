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
use libkernel::error::{KernelError, Result};

use crate::{fs::VFS, process::fd_table::Fd, sched::syscall_ctx::ProcessCtx};

pub async fn sys_sync(_ctx: &ProcessCtx) -> Result<usize> {
    VFS.sync_all().await?;
    Ok(0)
}

pub async fn sys_syncfs(ctx: &ProcessCtx, fd: Fd) -> Result<usize> {
    let task = ctx.shared().clone();

    let inode = task
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?
        .inode()
        .ok_or(KernelError::BadFd)?;

    VFS.sync(inode).await?;
    Ok(0)
}

pub async fn sys_fsync(ctx: &ProcessCtx, fd: Fd) -> Result<usize> {
    let task = ctx.shared().clone();

    let inode = task
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?
        .inode()
        .ok_or(KernelError::BadFd)?;
    inode.sync().await?;

    Ok(0)
}

pub async fn sys_fdatasync(ctx: &ProcessCtx, fd: Fd) -> Result<usize> {
    let task = ctx.shared().clone();

    let inode = task
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?
        .inode()
        .ok_or(KernelError::BadFd)?;
    inode.datasync().await?;

    Ok(0)
}
