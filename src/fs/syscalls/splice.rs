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
use crate::{kernel::kpipe::KPipe, process::fd_table::Fd, sched::syscall_ctx::ProcessCtx};
use alloc::sync::Arc;
use libkernel::{
    error::{KernelError, Result},
    memory::address::TUA,
};

pub async fn sys_sendfile(
    ctx: &ProcessCtx,
    out_fd: Fd,
    in_fd: Fd,
    _offset: TUA<u64>,
    mut count: usize,
) -> Result<usize> {
    let (reader, writer) = {
        let task = ctx.shared();
        let fds = task.fd_table.lock_save_irq();

        let reader = fds.get(in_fd).ok_or(KernelError::BadFd)?;
        let writer = fds.get(out_fd).ok_or(KernelError::BadFd)?;

        (reader, writer)
    };

    if Arc::ptr_eq(&reader, &writer) {
        return Err(KernelError::InvalidValue);
    }

    let kbuf = KPipe::new()?;

    let (reader_ops, reader_ctx) = &mut *reader.lock().await;
    let (writer_ops, writer_ctx) = &mut *writer.lock().await;

    let mut total_written = 0;

    while count > 0 {
        let read = reader_ops.splice_into(reader_ctx, &kbuf, count).await?;

        if read == 0 {
            return Ok(total_written);
        }

        let mut to_write = read;

        while to_write > 0 {
            let written = writer_ops.splice_from(writer_ctx, &kbuf, to_write).await?;
            to_write -= written;
            total_written += written;
        }

        count -= read;
    }

    Ok(total_written)
}
