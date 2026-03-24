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
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Uid(u32);

impl Uid {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn is_root(self) -> bool {
        self.0 == 0
    }

    pub fn new_root() -> Self {
        Self(0)
    }
}

impl From<u64> for Uid {
    /// Convenience implementation for syscalls.
    fn from(value: u64) -> Self {
        Self(value as _)
    }
}

impl From<Uid> for u32 {
    fn from(value: Uid) -> Self {
        value.0
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Gid(u32);

impl Gid {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn new_root_group() -> Self {
        Self(0)
    }
}

impl From<u64> for Gid {
    /// Convenience implementation for syscalls.
    fn from(value: u64) -> Self {
        Self(value as _)
    }
}

impl From<Gid> for u32 {
    fn from(value: Gid) -> Self {
        value.0
    }
}
