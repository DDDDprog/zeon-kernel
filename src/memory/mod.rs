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
    arch::ArchImpl,
    sync::{OnceLock, SpinLock},
};
use libkernel::memory::{
    allocators::{
        phys::FrameAllocator,
        smalloc::{RegionList, Smalloc},
    },
    region::PhysMemoryRegion,
};

pub mod brk;
pub mod fault;
pub mod mincore;
pub mod mmap;
pub mod page;
pub mod process_vm;
pub mod uaccess;

pub type PageOffsetTranslator = libkernel::memory::pg_offset::PageOffsetTranslator<ArchImpl>;

// Initial memory allocator. Used for initial memory setup.
const STATIC_REGION_COUNT: usize = 128;

static INIT_MEM_REGIONS: [PhysMemoryRegion; STATIC_REGION_COUNT] =
    [PhysMemoryRegion::empty(); STATIC_REGION_COUNT];
static INIT_RES_REGIONS: [PhysMemoryRegion; STATIC_REGION_COUNT] =
    [PhysMemoryRegion::empty(); STATIC_REGION_COUNT];

pub static INITAL_ALLOCATOR: SpinLock<Option<Smalloc<PageOffsetTranslator>>> =
    SpinLock::new(Some(Smalloc::new(
        RegionList::new(STATIC_REGION_COUNT, INIT_MEM_REGIONS.as_ptr().cast_mut()),
        RegionList::new(STATIC_REGION_COUNT, INIT_RES_REGIONS.as_ptr().cast_mut()),
    )));

// Main page allocator, setup by consuming smalloc.
pub static PAGE_ALLOC: OnceLock<FrameAllocator<ArchImpl>> = OnceLock::new();
