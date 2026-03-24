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
//! SPI Driver for Zeon
//! Serial Peripheral Interface bus driver

use crate::drivers::Driver;
use crate::error::KernelResult;

/// SPI Bus
pub struct SpiBus {
    pub base_addr: usize,
    pub clock_hz: u32,
    pub mode: SpiMode,
    initialized: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum SpiMode {
    Mode0, // CPOL=0, CPHA=0
    Mode1, // CPOL=0, CPHA=1
    Mode2, // CPOL=1, CPHA=0
    Mode3, // CPOL=1, CPHA=1
}

impl SpiBus {
    /// Create new SPI bus
    pub fn new(base_addr: usize, clock_hz: u32) -> Self {
        Self {
            base_addr,
            clock_hz,
            mode: SpiMode::Mode0,
            initialized: false,
        }
    }

    /// Initialize SPI bus
    pub fn init(&mut self) -> KernelResult {
        if self.initialized {
            return Ok(());
        }
        // Configure SPI mode and clock
        self.initialized = true;
        Ok(())
    }

    /// Transfer data (full duplex)
    pub fn transfer(&self, data: &[u8]) -> KernelResult<Vec<u8>> {
        if !self.initialized {
            return Err(crate::error::KernelError::NotInitialized);
        }
        // SPI transfer
        Ok(data.to_vec())
    }

    /// Set SPI mode
    pub fn set_mode(&mut self, mode: SpiMode) -> KernelResult {
        if !self.initialized {
            return Err(crate::error::KernelError::NotInitialized);
        }
        self.mode = mode;
        Ok(())
    }
}

impl Driver for SpiBus {
    fn name(&self) -> &str {
        "spi"
    }

    fn init(&mut self) -> KernelResult {
        self.init()
    }
}