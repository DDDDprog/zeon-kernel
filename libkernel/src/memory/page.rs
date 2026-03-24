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
use super::{
    PAGE_SHIFT,
    address::AddressTranslator,
    allocators::phys::{PageAllocGetter, PageAllocation},
    region::PhysMemoryRegion,
};
use crate::{
    CpuOps,
    error::Result,
    memory::{
        PAGE_SIZE,
        address::{PA, VA},
    },
};
use alloc::slice;
use core::fmt::Display;
use core::marker::PhantomData;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PageFrame {
    n: usize,
}

impl Display for PageFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.n.fmt(f)
    }
}

impl PageFrame {
    pub fn from_pfn(n: usize) -> Self {
        Self { n }
    }

    pub fn pa(&self) -> PA {
        PA::from_value(self.n << PAGE_SHIFT)
    }

    pub fn as_phys_range(&self) -> PhysMemoryRegion {
        PhysMemoryRegion::new(self.pa(), PAGE_SIZE)
    }

    pub fn value(&self) -> usize {
        self.n
    }

    pub(crate) fn buddy(self, order: usize) -> Self {
        Self {
            n: self.n ^ (1 << order),
        }
    }

    #[must_use]
    pub fn add_pages(self, n: usize) -> Self {
        Self { n: self.n + n }
    }
}

/// A convenience wrapper for dealing with single-page allocations.
pub struct ClaimedPage<A: CpuOps, G: PageAllocGetter<A>, T: AddressTranslator<()>>(
    PageAllocation<'static, A>,
    PhantomData<G>,
    PhantomData<T>,
);

impl<A: CpuOps, G: PageAllocGetter<A>, T: AddressTranslator<()>> Display for ClaimedPage<A, G, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0.region().start_address().to_pfn())
    }
}

impl<A: CpuOps, G: PageAllocGetter<A>, T: AddressTranslator<()>> ClaimedPage<A, G, T> {
    /// Allocates a single physical page. The contents of the page are
    /// undefined.
    fn alloc() -> Result<Self> {
        let frame = G::global_page_alloc().alloc_frames(0)?;
        Ok(Self(frame, PhantomData, PhantomData))
    }

    /// Allocates a single physical page and zeroes its contents.
    pub fn alloc_zeroed() -> Result<Self> {
        let mut page = Self::alloc()?;
        page.as_slice_mut().fill(0);
        Ok(page)
    }

    /// Takes ownership of the page at pfn.
    ///
    /// # Safety
    ///
    /// Ensure that the calling context does indeed own this page. Otherwise,
    /// the page may be freed when it's owned by another context.
    pub unsafe fn from_pfn(pfn: PageFrame) -> Self {
        Self(
            unsafe { G::global_page_alloc().alloc_from_region(pfn.as_phys_range()) },
            PhantomData,
            PhantomData,
        )
    }

    #[inline(always)]
    pub fn pa(&self) -> PA {
        self.0.region().start_address()
    }

    /// Returns the kernel virtual address where this page is mapped.
    #[inline(always)]
    pub fn va(&self) -> VA {
        self.pa().to_va::<T>()
    }

    /// Returns a raw pointer to the page's content.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        self.va().as_ptr() as *const _
    }

    /// Returns a mutable raw pointer to the page's content.
    #[inline(always)]
    pub fn as_ptr_mut(&self) -> *mut u8 {
        self.va().as_ptr_mut() as *mut _
    }

    /// Returns a slice representing the page's content.
    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        // This is safe because:
        // 1. We have a reference `&self`, guaranteeing safe access.
        // 2. The pointer is valid and aligned.
        // 3. The lifetime of the slice is tied to `&self` by the compiler.
        unsafe { slice::from_raw_parts(self.as_ptr(), PAGE_SIZE) }
    }

    /// Returns a mutable slice representing the page's content.
    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        // This is safe because:
        // 1. We have a mutable reference `&mut self`, guaranteeing exclusive access.
        // 2. The pointer is valid and aligned.
        // 3. The lifetime of the slice is tied to `&mut self` by the compiler.
        unsafe { slice::from_raw_parts_mut(self.as_ptr_mut(), PAGE_SIZE) }
    }

    pub fn leak(self) -> PageFrame {
        self.0.leak().start_address().to_pfn()
    }
}
