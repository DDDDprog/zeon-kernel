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
use super::park_cpu;
use aarch64_cpu::asm;
use aarch64_cpu::registers::{
    CurrentEL, ELR_EL2, ELR_EL3, HCR_EL2, Readable, SCR_EL3, SP_EL1, SPSR_EL2, SPSR_EL3, Writeable,
};
use core::arch::asm;

/// First rust entry point, called from `boot.s`. This function takes us down to
/// EL1. Also called by secondaries during secondary boot.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn transition_to_el1(stack_addr: u64) {
    let ret_address = {
        let mut addr: u64;
        unsafe {
            asm!("mov {0}, lr", out(reg) addr);
        }
        addr
    };

    match CurrentEL.read_as_enum(CurrentEL::EL) {
        Some(CurrentEL::EL::Value::EL0) => park_cpu(),
        Some(CurrentEL::EL::Value::EL1) => return,
        Some(CurrentEL::EL::Value::EL2) => {
            SPSR_EL2.write(
                SPSR_EL2::M::EL1h
                    + SPSR_EL2::I::Masked
                    + SPSR_EL2::F::Masked
                    + SPSR_EL2::D::Masked
                    + SPSR_EL2::A::Masked,
            );
            HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
            ELR_EL2.set(ret_address);
        }
        Some(CurrentEL::EL::Value::EL3) => {
            SPSR_EL3.write(
                SPSR_EL3::M::EL1h
                    + SPSR_EL3::I::Masked
                    + SPSR_EL3::F::Masked
                    + SPSR_EL3::D::Masked
                    + SPSR_EL3::A::Masked,
            );
            SCR_EL3.write(SCR_EL3::RW::NextELIsAarch64);
            ELR_EL3.set(ret_address);
        }
        None => park_cpu(),
    }

    SP_EL1.set(stack_addr);

    asm::eret();
}
