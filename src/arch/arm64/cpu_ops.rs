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
use core::arch::asm;

/// Returns the current state of the interrupt flags (DAIF register) and disables IRQs.
#[inline(always)]
pub fn local_irq_save() -> usize {
    let flags: u64;
    unsafe {
        asm!(
            "mrs {0}, daif",     // Read DAIF into flags
            "msr daifset, #2",   // Disable IRQs (set the I bit)
            out(reg) flags,
            options(nomem, nostack)
        );
    }
    flags as _
}

/// Restores the interrupt flags to a previously saved state.
#[inline(always)]
pub fn local_irq_restore(flags: usize) {
    unsafe {
        asm!(
            "msr daif, {0}",    // Write flags back to DAIF
            in(reg) flags as u64,
            options(nomem, nostack)
        );
    }
}
