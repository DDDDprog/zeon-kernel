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
use core::mem::transmute;

use libkernel::error::KernelError;

use super::SigId;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UserSigId(u32);

impl TryFrom<UserSigId> for SigId {
    type Error = KernelError;

    fn try_from(value: UserSigId) -> core::result::Result<Self, Self::Error> {
        if value.0 < 1 || value.0 > 31 {
            Err(KernelError::InvalidValue)
        } else {
            // SAFETY: The above bounds check ensure that the value is within
            // range.
            Ok(unsafe { transmute::<u32, SigId>(value.0 - 1) })
        }
    }
}

impl From<u64> for UserSigId {
    fn from(value: u64) -> Self {
        Self(value as _)
    }
}
