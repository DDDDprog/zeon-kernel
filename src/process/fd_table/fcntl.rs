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
use super::Fd;
use crate::process::fd_table::dup::dup_fd;
use crate::{process::fd_table::FdFlags, sched::syscall_ctx::ProcessCtx};
use bitflags::Flags;
use libkernel::error::{KernelError, Result};
use libkernel::fs::OpenFlags;

const F_DUPFD: u32 = 0; // Duplicate file descriptor.
const F_GETFD: u32 = 1; // Get file descriptor flags.
const F_SETFD: u32 = 2; // Set file descriptor flags.
const F_GETFL: u32 = 3; // Get file status flags.
const F_SETFL: u32 = 4; // Set file status flags.

pub async fn sys_fcntl(ctx: &ProcessCtx, fd: Fd, op: u32, arg: usize) -> Result<usize> {
    let task = ctx.shared();

    match op {
        F_DUPFD => dup_fd(ctx, fd, Some(Fd(arg as i32))).map(|new_fd| new_fd.as_raw() as _),
        F_GETFD => {
            let fds = task.fd_table.lock_save_irq();
            let fd = fds
                .entries
                .get(fd.as_raw() as usize)
                .and_then(|entry| entry.as_ref())
                .ok_or(KernelError::BadFd)?;
            Ok(fd.flags.bits() as _)
        }
        F_SETFD => {
            let mut fds = task.fd_table.lock_save_irq();
            let fd = fds
                .entries
                .get_mut(fd.as_raw() as usize)
                .and_then(|entry| entry.as_mut())
                .ok_or(KernelError::BadFd)?;

            let new_flags = FdFlags::from_bits_retain(arg as _);
            if new_flags.contains_unknown_bits() {
                return Err(KernelError::InvalidValue);
            }
            fd.flags = new_flags;
            Ok(0)
        }
        F_GETFL => {
            let open_fd = {
                let mut fds = task.fd_table.lock_save_irq();
                let fd = fds
                    .entries
                    .get_mut(fd.as_raw() as usize)
                    .and_then(|entry| entry.as_mut())
                    .ok_or(KernelError::BadFd)?;

                fd.file.clone()
            };

            Ok(open_fd.flags().await.bits() as _)
        }
        F_SETFL => {
            let fl = OpenFlags::from_bits_retain(arg as _);
            if fl.contains_unknown_bits() {
                return Err(KernelError::InvalidValue);
            }
            let open_fd = {
                let mut fds = task.fd_table.lock_save_irq();
                let fd = fds
                    .entries
                    .get_mut(fd.as_raw() as usize)
                    .and_then(|entry| entry.as_mut())
                    .ok_or(KernelError::BadFd)?;

                fd.file.clone()
            };
            // TODO: Ignore sync/dsync when implemented
            open_fd.set_flags(fl).await;
            Ok(0)
        }
        _ => Err(KernelError::InvalidValue),
    }
}
