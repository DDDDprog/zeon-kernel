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
use crate::sched::syscall_ctx::ProcessCtx;
use libkernel::{
    error::{KernelError, Result},
    fs::OpenFlags,
};

use super::{Fd, FdFlags, FileDescriptorEntry};

pub fn dup_fd(ctx: &ProcessCtx, fd: Fd, min_fd: Option<Fd>) -> Result<Fd> {
    let task = ctx.shared();
    let mut files = task.fd_table.lock_save_irq();

    let file = files.get(fd).ok_or(KernelError::BadFd)?;

    let new_fd = match min_fd {
        Some(min_fd) => files.insert_above(min_fd, file.clone())?,
        None => files.insert(file.clone())?,
    };

    Ok(new_fd)
}

pub fn sys_dup(ctx: &ProcessCtx, fd: Fd) -> Result<usize> {
    let new_fd = dup_fd(ctx, fd, None)?;

    Ok(new_fd.as_raw() as _)
}

pub fn sys_dup3(ctx: &ProcessCtx, oldfd: Fd, newfd: Fd, flags: u32) -> Result<usize> {
    if oldfd == newfd {
        return Err(KernelError::InvalidValue);
    }

    let flags = OpenFlags::from_bits_retain(flags);

    if !flags.difference(OpenFlags::O_CLOEXEC).is_empty() {
        // We only permit the O_CLOEXEC flag for dup3.
        return Err(KernelError::InvalidValue);
    }

    let task = ctx.shared();
    let mut files = task.fd_table.lock_save_irq();

    let old_file = files.get(oldfd).ok_or(KernelError::BadFd)?;

    files.insert_at(
        newfd,
        FileDescriptorEntry {
            file: old_file.clone(),
            flags: if flags.contains(OpenFlags::O_CLOEXEC) {
                FdFlags::CLOEXEC
            } else {
                FdFlags::empty()
            },
        },
    );

    Ok(newfd.as_raw() as _)
}
