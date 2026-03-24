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
use alloc::sync::Arc;
use bitflags::bitflags;
use libkernel::error::{KernelError, Result};

async fn close(ctx: &ProcessCtx, fd: Fd) -> Result<()> {
    let file = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .remove(fd)
        .ok_or(KernelError::BadFd)?;

    if let Some(file) = Arc::into_inner(file) {
        let (ops, ctx) = &mut *file.lock().await;
        ops.release(ctx).await?;
    }
    Ok(())
}

pub async fn sys_close(ctx: &ProcessCtx, fd: Fd) -> Result<usize> {
    close(ctx, fd).await?;
    Ok(0)
}

bitflags! {
    pub struct CloseRangeFlags: i32 {
        const CLOSE_RANGE_UNSHARE = 1 << 1;
        const CLOSE_RANGE_CLOEXEC = 1 << 2;
    }
}

pub async fn sys_close_range(ctx: &ProcessCtx, first: Fd, last: Fd, flags: i32) -> Result<usize> {
    let flags = CloseRangeFlags::from_bits_truncate(flags);
    if flags.contains(CloseRangeFlags::CLOSE_RANGE_UNSHARE) {
        todo!("Implement CLOSE_RANGE_UNSHARE");
    }
    if flags.contains(CloseRangeFlags::CLOSE_RANGE_CLOEXEC) {
        todo!("Implement CLOSE_RANGE_CLOEXEC");
    }

    for i in first.as_raw()..=last.as_raw() {
        close(ctx, Fd(i)).await?;
    }
    Ok(0)
}
