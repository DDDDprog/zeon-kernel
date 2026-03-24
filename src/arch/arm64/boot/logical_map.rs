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
use crate::memory::{INITAL_ALLOCATOR, PageOffsetTranslator};

use super::super::memory::{
    fixmap::{FIXMAPS, Fixmap},
    mmu::smalloc_page_allocator::SmallocPageAlloc,
    tlb::AllEl1TlbInvalidator,
};
use libkernel::{
    arch::arm64::memory::{
        pg_descriptors::MemoryType,
        pg_tables::{
            L0Table, MapAttributes, MappingContext, PageTableMapper, PgTable, PgTableArray,
            map_range,
        },
    },
    error::Result,
    memory::{
        address::{TPA, TVA},
        permissions::PtePermissions,
    },
};

pub struct FixmapMapper<'a> {
    pub fixmaps: &'a mut Fixmap,
}

impl PageTableMapper for FixmapMapper<'_> {
    unsafe fn with_page_table<T: PgTable, R>(
        &mut self,
        pa: TPA<PgTableArray<T>>,
        f: impl FnOnce(TVA<PgTableArray<T>>) -> R,
    ) -> Result<R> {
        let guard = self.fixmaps.temp_remap_page_table(pa)?;

        // SAFETY: The guard will live for the lifetime of the closure.
        Ok(f(unsafe { guard.get_va() }))
    }
}

pub fn setup_logical_map(pgtbl_base: TPA<PgTableArray<L0Table>>) -> Result<()> {
    let mut fixmaps = FIXMAPS.lock_save_irq();
    let mut alloc = INITAL_ALLOCATOR.lock_save_irq();
    let alloc = alloc.as_mut().unwrap();
    let mem_list = alloc.get_memory_list();
    let mut mapper = FixmapMapper {
        fixmaps: &mut fixmaps,
    };
    let mut pg_alloc = SmallocPageAlloc::new(alloc);

    let mut ctx = MappingContext {
        allocator: &mut pg_alloc,
        mapper: &mut mapper,
        invalidator: &AllEl1TlbInvalidator::new(),
    };

    for mem_region in mem_list.iter() {
        let map_attrs = MapAttributes {
            phys: mem_region,
            virt: mem_region.map_via::<PageOffsetTranslator>(),
            mem_type: MemoryType::Normal,
            perms: PtePermissions::rw(false),
        };

        map_range(pgtbl_base, map_attrs, &mut ctx)?;
    }

    Ok(())
}
