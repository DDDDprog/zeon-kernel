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
use core::convert::Infallible;

use libkernel::memory::address::VA;

use crate::sched::syscall_ctx::ProcessCtx;

/// Handles the `brk` system call.
///
/// This function emulates the behavior of the Linux `brk` syscall.
///
/// # Arguments
/// * `addr`: The virtual address for the new program break.
///
/// # Returns
/// A `Result` containing the new program break address as a `usize`. Note that
/// according to the `brk(2)` man page, the syscall itself doesn't "fail" in the
/// traditional sense. It always returns a memory address, hence the Infallible
/// error type.
/// - If `addr` is 0, it returns the current break.
/// - On a successful resize, it returns the new break.
/// - On a failed resize, it returns the current, unchanged break.
pub async fn sys_brk(ctx: &ProcessCtx, addr: VA) -> Result<usize, Infallible> {
    let mut vm = ctx.shared().vm.lock_save_irq();

    // The query case `brk(0)` is special and is handled separately from modifications.
    if addr.is_null() {
        let current_brk_val = vm.current_brk().value();
        return Ok(current_brk_val);
    }

    // For non-null addresses, attempt to resize the break.
    let resize_result = vm.resize_brk(addr);

    match resize_result {
        // Success: The break was resized. The function returns the new address.
        Ok(new_brk) => Ok(new_brk.value()),
        // Failure: The resize was invalid (e.g., collision, shrink below start).
        // The contract is to return the current, unchanged break address.
        Err(_) => {
            let current_brk_val = vm.current_brk().value();
            Ok(current_brk_val)
        }
    }
}
