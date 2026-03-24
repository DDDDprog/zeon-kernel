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
use core::marker::PhantomData;

use crate::memory::page::ClaimedPage;
use libkernel::{
    arch::arm64::memory::pg_tables::{PageAllocator, PgTable, PgTableArray},
    error::Result,
    memory::address::TPA,
};

pub struct PageTableAllocator<'a> {
    data: PhantomData<&'a u8>,
}

impl PageTableAllocator<'_> {
    pub fn new() -> Self {
        Self { data: PhantomData }
    }
}

impl PageAllocator for PageTableAllocator<'_> {
    fn allocate_page_table<T: PgTable>(&mut self) -> Result<TPA<PgTableArray<T>>> {
        let pg = ClaimedPage::alloc_zeroed()?;

        Ok(pg.leak().pa().cast())
    }
}
