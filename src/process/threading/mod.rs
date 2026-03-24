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
use core::ffi::c_long;
use core::mem::size_of;

use crate::sched::syscall_ctx::ProcessCtx;
use libkernel::{
    error::{KernelError, Result},
    memory::address::TUA,
};

pub mod futex;

pub fn sys_set_tid_address(ctx: &mut ProcessCtx, tidptr: TUA<u32>) -> Result<usize> {
    let task = ctx.task_mut();

    task.child_tid_ptr = Some(tidptr);

    Ok(task.tid.value() as _)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RobustList {
    next: TUA<RobustList>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RobustListHead {
    list: RobustList,
    futex_offset: c_long,
    list_op_pending: RobustList,
}

pub async fn sys_set_robust_list(
    ctx: &mut ProcessCtx,
    head: TUA<RobustListHead>,
    len: usize,
) -> Result<usize> {
    if core::hint::unlikely(len != size_of::<RobustListHead>()) {
        return Err(KernelError::InvalidValue);
    }

    let task = ctx.task_mut();
    task.robust_list.replace(head);

    Ok(0)
}
