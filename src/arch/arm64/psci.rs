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
use core::arch::naked_asm;
use libkernel::memory::address::PA;

pub struct PSCIEntry {
    pub method: PSCIMethod,
    pub cpu_on_id: Option<u32>,
}

pub enum PSCIMethod {
    Hvc,
    Smc,
}

const CPU_ON_ID: u32 = 0xc400_0003;

// Re-export the low-level PSCI helpers so other modules (e.g. `arch::arm64::mod`)
// can invoke them without repeating the `use` dance.

pub fn boot_secondary_psci(entry: PSCIEntry, core_id: usize, entry_fn: PA, ctx: PA) {
    let method_id = entry.cpu_on_id.unwrap_or(CPU_ON_ID);

    match entry.method {
        PSCIMethod::Hvc => unsafe {
            do_psci_hyp_call(
                method_id,
                core_id as _,
                entry_fn.value() as _,
                ctx.value() as _,
            )
        },
        PSCIMethod::Smc => unsafe {
            do_psci_smc_call(
                method_id,
                core_id as _,
                entry_fn.value() as _,
                ctx.value() as _,
            )
        },
    };
}

#[unsafe(naked)]
pub unsafe extern "C" fn do_psci_hyp_call(id: u32, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    naked_asm!("hvc #0", "ret")
}

#[unsafe(naked)]
pub unsafe extern "C" fn do_psci_smc_call(id: u32, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    naked_asm!("smc #0", "ret")
}
