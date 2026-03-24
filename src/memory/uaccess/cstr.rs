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
use core::ffi::{CStr, c_char};
use libkernel::error::{KernelError, Result};
use libkernel::memory::address::TUA;

use crate::arch::{Arch, ArchImpl};

pub struct UserCStr(TUA<c_char>);

impl UserCStr {
    pub fn from_ptr(ptr: TUA<c_char>) -> Self {
        Self(ptr)
    }

    pub async fn copy_from_user(self, buf: &mut [u8]) -> Result<&str> {
        // Ensure null-filled buffer.
        buf.fill(0);

        let len = unsafe {
            ArchImpl::copy_strn_from_user(self.0.to_untyped(), buf.as_mut_ptr(), buf.len())
        }
        .await?;

        if len == buf.len() {
            // We didn't find a NULL byte and filled up the buffer.
            return Err(KernelError::BufferFull);
        }

        let cstr = CStr::from_bytes_with_nul(&buf[..len + 1]).unwrap();

        Ok(cstr.to_str().unwrap())
    }
}
