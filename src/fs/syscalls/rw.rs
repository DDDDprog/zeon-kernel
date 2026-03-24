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
use crate::{process::fd_table::Fd, sched::syscall_ctx::ProcessCtx};
use libkernel::{
    error::{KernelError, Result},
    memory::address::UA,
};

pub async fn sys_write(ctx: &ProcessCtx, fd: Fd, user_buf: UA, count: usize) -> Result<usize> {
    let file = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let (ops, ctx) = &mut *file.lock().await;

    ops.write(ctx, user_buf, count).await
}

pub async fn sys_read(ctx: &ProcessCtx, fd: Fd, user_buf: UA, count: usize) -> Result<usize> {
    let file = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let (ops, ctx) = &mut *file.lock().await;

    ops.read(ctx, user_buf, count).await
}

pub async fn sys_pwrite64(
    ctx: &ProcessCtx,
    fd: Fd,
    user_buf: UA,
    count: usize,
    offset: u64,
) -> Result<usize> {
    let file = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let (ops, _ctx) = &mut *file.lock().await;

    ops.writeat(user_buf, count, offset).await
}

pub async fn sys_pread64(
    ctx: &ProcessCtx,
    fd: Fd,
    user_buf: UA,
    count: usize,
    offset: u64,
) -> Result<usize> {
    let file = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let (ops, _ctx) = &mut *file.lock().await;

    ops.readat(user_buf, count, offset).await
}
