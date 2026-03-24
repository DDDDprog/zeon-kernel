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
use crate::memory::PageOffsetTranslator;
use libkernel::{
    arch::arm64::memory::pg_tables::{PageAllocator, PgTable, PgTableArray},
    error::Result,
    memory::{PAGE_SIZE, address::TPA, allocators::smalloc::Smalloc},
};

pub struct SmallocPageAlloc<'a> {
    smalloc: &'a mut Smalloc<PageOffsetTranslator>,
}

impl<'a> SmallocPageAlloc<'a> {
    pub fn new(smalloc: &'a mut Smalloc<PageOffsetTranslator>) -> Self {
        Self { smalloc }
    }
}

impl PageAllocator for SmallocPageAlloc<'_> {
    fn allocate_page_table<T: PgTable>(&mut self) -> Result<TPA<PgTableArray<T>>> {
        Ok(TPA::from_value(
            self.smalloc.alloc(PAGE_SIZE, PAGE_SIZE)?.value(),
        ))
    }
}
