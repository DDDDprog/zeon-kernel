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
use core::arch::global_asm;
use libkernel::{
    KernAddressSpace, VirtualMemory,
    error::Result,
    memory::{
        address::VA,
        permissions::PtePermissions,
        region::{PhysMemoryRegion, VirtMemoryRegion},
    },
};
use log::info;

use crate::{arch::ArchImpl, ksym_pa};

global_asm!(include_str!("vdso.s"));

pub const VDSO_BASE: VA = VA::from_value(0xffff_8100_0000_0000);

unsafe extern "C" {
    static __vdso_start: u8;
    static __vdso_end: u8;
}

pub fn vdso_init() -> Result<()> {
    let start = ksym_pa!(__vdso_start);
    let end = ksym_pa!(__vdso_end);
    let region = PhysMemoryRegion::from_start_end_address(start, end);

    let mappable_region = region.to_mappable_region();

    let mut kspc = ArchImpl::kern_address_space().lock_save_irq();

    let vregion = VirtMemoryRegion::new(VDSO_BASE, mappable_region.region().size());

    kspc.map_normal(mappable_region.region(), vregion, PtePermissions::rx(true))?;

    info!(
        "VDSO mapped to: 0x{:x} (0x{:x} bytes)",
        vregion.start_address().value(),
        vregion.size()
    );

    Ok(())
}
