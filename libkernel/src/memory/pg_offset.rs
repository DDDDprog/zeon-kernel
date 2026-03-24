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
use super::address::{AddressTranslator, TPA, TVA};
use crate::VirtualMemory;
use core::marker::PhantomData;

pub struct PageOffsetTranslator<VM: VirtualMemory> {
    _phantom: PhantomData<VM>,
}

unsafe impl<VM: VirtualMemory> Send for PageOffsetTranslator<VM> {}
unsafe impl<VM: VirtualMemory> Sync for PageOffsetTranslator<VM> {}

impl<T, VM: VirtualMemory> AddressTranslator<T> for PageOffsetTranslator<VM> {
    fn virt_to_phys(va: TVA<T>) -> TPA<T> {
        let mut v = va.value();

        v -= VM::PAGE_OFFSET;

        TPA::from_value(v)
    }

    fn phys_to_virt(pa: TPA<T>) -> TVA<T> {
        let mut v = pa.value();

        v += VM::PAGE_OFFSET;

        TVA::from_value(v)
    }
}
