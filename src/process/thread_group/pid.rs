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

use crate::sched::syscall_ctx::ProcessCtx;
use core::convert::Infallible;

use super::Pgid;
use crate::process::{Tid, find_task_by_tid};

/// Userspace `pid_t` type.
pub type PidT = i32;

pub fn sys_getpid(ctx: &ProcessCtx) -> core::result::Result<usize, Infallible> {
    Ok(ctx.shared().process.tgid.value() as _)
}

pub fn sys_getppid(ctx: &ProcessCtx) -> core::result::Result<usize, Infallible> {
    Ok(ctx
        .shared()
        .process
        .parent
        .lock_save_irq()
        .as_ref()
        .and_then(|x| x.upgrade())
        .map(|x| x.tgid.value())
        .unwrap_or(0) as _)
}

pub fn sys_getpgid(ctx: &ProcessCtx, pid: PidT) -> Result<usize> {
    let pgid = if pid == 0 {
        *ctx.shared().process.pgid.lock_save_irq()
    } else if let Some(task) = find_task_by_tid(Tid::from_pid_t(pid)) {
        *task.process.pgid.lock_save_irq()
    } else {
        return Err(KernelError::NoProcess);
    };

    Ok(pgid.value() as _)
}

pub fn sys_setpgid(ctx: &ProcessCtx, pid: PidT, pgid: Pgid) -> Result<usize> {
    if pid == 0 {
        *ctx.shared().process.pgid.lock_save_irq() = pgid;
    } else if let Some(task) = find_task_by_tid(Tid::from_pid_t(pid)) {
        *task.process.pgid.lock_save_irq() = pgid;
    } else {
        return Err(KernelError::NoProcess);
    };

    Ok(0)
}
