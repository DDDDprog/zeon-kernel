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

use libkernel::arch::arm64::memory::tlb::TLBInvalidator;

pub struct AllEl1TlbInvalidator;

impl AllEl1TlbInvalidator {
    pub fn new() -> Self {
        Self
    }
}

impl Drop for AllEl1TlbInvalidator {
    fn drop(&mut self) {
        unsafe {
            asm!(
                // Data Synchronization Barrier, Inner Shareable, write to
                // read/write.
                "dsb ishst",
                // Invalidate TLB by VA, for EL1, Inner Shareable
                "tlbi vmalle1is",
                // Data Synchronization Barrier, Inner Shareable.
                "dsb ish",
                // Instruction Synchronization Barrier.
                "isb",
                options(nostack, preserves_flags)
            );
        }
    }
}

impl TLBInvalidator for AllEl1TlbInvalidator {}

pub struct AllEl0TlbInvalidator;

impl AllEl0TlbInvalidator {
    pub fn new() -> Self {
        Self
    }
}

impl Drop for AllEl0TlbInvalidator {
    fn drop(&mut self) {
        unsafe {
            asm!(
                // Data Synchronization Barrier, Inner Shareable, write to
                // read/write.
                "dsb ishst",
                // Invalidate TLB by VA, for EL1, Inner Shareable
                "tlbi vmalle1is",
                // Data Synchronization Barrier, Inner Shareable.
                "dsb ish",
                // Instruction Synchronization Barrier.
                "isb",
                options(nostack, preserves_flags)
            );
        }
    }
}

impl TLBInvalidator for AllEl0TlbInvalidator {}
