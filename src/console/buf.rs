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
use core::fmt::{self, Write};

const BUF_CONSOLE_SZ: usize = 16 * 1024 * 1024;

/// A fixed-size buffer console.
pub(super) struct BufConsole {
    data: [u8; BUF_CONSOLE_SZ],
    ptr: usize,
}

impl BufConsole {
    pub const fn new() -> Self {
        Self {
            data: [0; BUF_CONSOLE_SZ],
            ptr: 0,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..self.ptr]
    }
}

impl Write for BufConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let available = BUF_CONSOLE_SZ.saturating_sub(self.ptr);
        let len = core::cmp::min(available, s.len());
        self.data[self.ptr..self.ptr + len].copy_from_slice(&s.as_bytes()[..len]);
        self.ptr += len;
        Ok(())
    }
}
