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

use libkernel::memory::address::{PA, VA};

pub mod address_space;
pub mod fault;
pub mod fixmap;
pub mod heap;
pub mod mmu;
pub mod tlb;
pub mod uaccess;

pub const PAGE_OFFSET: usize = 0xffff_0000_0000_0000;
pub const IMAGE_BASE: VA = VA::from_value(0xffff_8000_0000_0000);
pub const FIXMAP_BASE: VA = VA::from_value(0xffff_9000_0000_0000);
pub const MMIO_BASE: VA = VA::from_value(0xffff_d000_0000_0000);
pub const EXCEPTION_BASE: VA = VA::from_value(0xffff_e000_0000_0000);

const BOGUS_START: PA = PA::from_value(usize::MAX);
static mut KIMAGE_START: PA = BOGUS_START;

#[macro_export]
macro_rules! ksym_pa {
    ($sym:expr) => {{
        let v = libkernel::memory::address::VA::from_value(core::ptr::addr_of!($sym) as usize);
        $crate::arch::arm64::memory::translate_kernel_va(v)
    }};
}

#[macro_export]
macro_rules! kfunc_pa {
    ($sym:expr) => {{
        let v = libkernel::memory::address::VA::from_value($sym as usize);
        $crate::arch::arm64::memory::translate_kernel_va(v)
    }};
}

pub fn set_kimage_start(pa: PA) {
    unsafe {
        if KIMAGE_START != BOGUS_START {
            panic!("Attempted to change RAM_START, once set");
        }

        KIMAGE_START = pa;
    }
}

pub fn get_kimage_start() -> PA {
    unsafe {
        if KIMAGE_START == BOGUS_START {
            panic!("attempted to access RAM_START before being set");
        }

        KIMAGE_START
    }
}

pub fn translate_kernel_va(addr: VA) -> PA {
    let mut v = addr.value();

    v -= IMAGE_BASE.value();

    PA::from_value(v + get_kimage_start().value())
}

pub fn flush_to_ram<T>(x: *const T) {
    let mut stride: usize = 0;

    // Calc  cache line stride.
    unsafe { asm!("mrs {0}, ctr_el0", out(reg) stride, options(nostack, nomem)) };
    stride = (1 << ((stride >> 16) & 0xf)) * 4;

    let end = unsafe { x.byte_add(size_of::<T>()) } as usize;
    let mut addr = (x as usize) & !(stride - 1); // align down

    while addr < end {
        // Clear the cache line for the given VA.
        unsafe {
            asm!("dc cvac, {0}", in(reg) addr, options(nostack));

            addr += stride;
        }
    }

    // Ensure the cache maintaince op has finished.
    unsafe { asm!("dsb ish", options(nostack)) };
}
