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
use crate::sched::syscall_ctx::ProcessCtx;
use libkernel::UserAddressSpace;
use libkernel::error::{KernelError, Result};
use libkernel::memory::address::{TUA, VA};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum FutexKey {
    Private { pid: u32, addr: usize },
    Shared { frame: usize, offset: usize },
}

impl FutexKey {
    pub fn new_private(ctx: &ProcessCtx, uaddr: TUA<u32>) -> Self {
        let pid = ctx.shared().process.tgid.value();

        Self::Private {
            pid,
            addr: uaddr.value(),
        }
    }

    pub fn new_shared(ctx: &ProcessCtx, uaddr: TUA<u32>) -> Result<Self> {
        let pg_info = ctx
            .shared()
            .vm
            .lock_save_irq()
            .mm_mut()
            .address_space_mut()
            .translate(VA::from_value(uaddr.value()))
            .ok_or(KernelError::Fault)?;

        Ok(Self::Shared {
            frame: pg_info.pfn.value(),
            offset: uaddr.page_offset(),
        })
    }
}
