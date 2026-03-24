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
use super::{MMIO_BASE, tlb::AllEl1TlbInvalidator};
use crate::sync::{OnceLock, SpinLock};
use libkernel::{
    KernAddressSpace,
    arch::arm64::memory::{
        pg_descriptors::{MemoryType, PaMapper},
        pg_tables::{L0Table, MapAttributes, MappingContext, PgTableArray, map_range},
        pg_walk::get_pte,
    },
    error::Result,
    memory::{
        address::{PA, TPA, VA},
        permissions::PtePermissions,
        region::{PhysMemoryRegion, VirtMemoryRegion},
    },
};
use page_allocator::PageTableAllocator;
use page_mapper::PageOffsetPgTableMapper;

pub mod page_allocator;
pub mod page_mapper;
pub mod smalloc_page_allocator;

pub static KERN_ADDR_SPC: OnceLock<SpinLock<Arm64KernelAddressSpace>> = OnceLock::new();

pub struct Arm64KernelAddressSpace {
    kernel_l0: TPA<PgTableArray<L0Table>>,
    mmio_ptr: VA,
}

impl Arm64KernelAddressSpace {
    fn do_map(&self, map_attrs: MapAttributes) -> Result<()> {
        let mut ctx = MappingContext {
            allocator: &mut PageTableAllocator::new(),
            mapper: &mut PageOffsetPgTableMapper {},
            invalidator: &AllEl1TlbInvalidator::new(),
        };

        map_range(self.kernel_l0, map_attrs, &mut ctx)
    }

    pub fn translate(&self, va: VA) -> Option<PA> {
        let pg_offset = va.page_offset();

        let pte = get_pte(self.kernel_l0, va, &mut PageOffsetPgTableMapper {})
            .ok()
            .flatten()?;

        let pa = pte.mapped_address()?;

        Some(pa.add_bytes(pg_offset))
    }

    pub fn table_pa(&self) -> PA {
        self.kernel_l0.to_untyped()
    }
}

unsafe impl Send for Arm64KernelAddressSpace {}

impl KernAddressSpace for Arm64KernelAddressSpace {
    fn map_normal(
        &mut self,
        phys_range: PhysMemoryRegion,
        virt_range: VirtMemoryRegion,
        perms: PtePermissions,
    ) -> Result<()> {
        self.do_map(MapAttributes {
            phys: phys_range,
            virt: virt_range,
            mem_type: MemoryType::Normal,
            perms,
        })
    }

    fn map_mmio(&mut self, phys_range: PhysMemoryRegion) -> Result<VA> {
        let phys_mappable_region = phys_range.to_mappable_region();
        let base_va = self.mmio_ptr;

        let virt_range = VirtMemoryRegion::new(base_va, phys_mappable_region.region().size());

        self.do_map(MapAttributes {
            phys: phys_mappable_region.region(),
            virt: virt_range,
            mem_type: MemoryType::Device,
            perms: PtePermissions::rw(false),
        })?;

        self.mmio_ptr =
            VA::from_value(self.mmio_ptr.value() + phys_mappable_region.region().size());

        Ok(VA::from_value(
            base_va.value() + phys_mappable_region.offset(),
        ))
    }
}

pub fn setup_kern_addr_space(pa: TPA<PgTableArray<L0Table>>) -> Result<()> {
    let addr_space = SpinLock::new(Arm64KernelAddressSpace {
        kernel_l0: pa,
        mmio_ptr: MMIO_BASE,
    });

    KERN_ADDR_SPC
        .set(addr_space)
        .map_err(|_| libkernel::error::KernelError::InUse)
}
