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
use crate::{
    arch::{ArchImpl, arm64::exceptions::ExceptionState},
    memory::{PageOffsetTranslator, page::ClaimedPage},
    process::owned::OwnedTask,
};
use core::arch::global_asm;
use libkernel::{
    UserAddressSpace, VirtualMemory,
    memory::{
        address::VA,
        permissions::PtePermissions,
        proc_vm::vmarea::{VMAPermissions, VMArea, VMAreaKind},
        region::VirtMemoryRegion,
    },
};

global_asm!(include_str!("idle.s"));

pub fn create_idle_task() -> OwnedTask {
    let code_page = ClaimedPage::alloc_zeroed().unwrap().leak();
    let code_addr = VA::from_value(0xd00d0000);

    unsafe extern "C" {
        static __idle_start: u8;
        static __idle_end: u8;
    }

    let idle_start_ptr = unsafe { &__idle_start } as *const u8;
    let idle_end_ptr = unsafe { &__idle_end } as *const u8;
    let code_sz = idle_end_ptr.addr() - idle_start_ptr.addr();

    unsafe {
        idle_start_ptr.copy_to(
            code_page
                .pa()
                .to_va::<PageOffsetTranslator>()
                .cast::<u8>()
                .as_ptr_mut(),
            code_sz,
        );
    };

    let mut addr_space = <ArchImpl as VirtualMemory>::ProcessAddressSpace::new().unwrap();

    addr_space
        .map_page(code_page, code_addr, PtePermissions::rx(true))
        .unwrap();

    let ctx = ExceptionState {
        x: [0; 31],
        elr_el1: code_addr.value() as _,
        spsr_el1: 0,
        sp_el0: 0,
        tpid_el0: 0,
    };

    let code_map = VMArea::new(
        VirtMemoryRegion::new(code_addr, code_sz),
        VMAreaKind::Anon,
        VMAPermissions::rx(),
    );

    OwnedTask::create_idle_task(addr_space, ctx, code_map)
}
