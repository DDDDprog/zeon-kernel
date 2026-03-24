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
//! USB Driver for Zeon
//! Universal Serial Bus host controller

use crate::drivers::Driver;
use crate::error::KernelResult;

/// USB Host Controller
pub struct UsbController {
    pub base_addr: usize,
    pub version: UsbVersion,
    initialized: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum UsbVersion {
    Usb1_1,
    Usb2_0,
    Usb3_0,
    Usb3_1,
}

impl UsbController {
    /// Create new USB controller
    pub fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
            version: UsbVersion::Usb2_0,
            initialized: false,
        }
    }

    /// Initialize USB controller
    pub fn init(&mut self) -> KernelResult {
        if self.initialized {
            return Ok(());
        }
        // Reset USB host controller
        // Configure ports and enable interrupts
        self.initialized = true;
        Ok(())
    }

    /// Get connected devices
    pub fn enumerate(&self) -> KernelResult<Vec<UsbDevice>> {
        if !self.initialized {
            return Err(crate::error::KernelError::NotInitialized);
        }
        // Enumerate USB devices
        Ok(vec![])
    }

    /// Submit USB transfer
    pub fn submit_transfer(&self, transfer: &UsbTransfer) -> KernelResult {
        if !self.initialized {
            return Err(crate::error::KernelError::NotInitialized);
        }
        Ok(())
    }
}

/// USB Device
#[derive(Debug)]
pub struct UsbDevice {
    pub vendor_id: u16,
    pub product_id: u16,
    pub class: u8,
    pub speed: UsbSpeed,
}

#[derive(Debug, Clone, Copy)]
pub enum UsbSpeed {
    Low,
    Full,
    High,
    Super,
}

/// USB Transfer
#[derive(Debug)]
pub struct UsbTransfer {
    pub device_addr: u8,
    pub endpoint: u8,
    pub data: Vec<u8>,
    pub transfer_type: TransferType,
}

#[derive(Debug, Clone, Copy)]
pub enum TransferType {
    Control,
    Interrupt,
    Bulk,
    Isochronous,
}

impl Driver for UsbController {
    fn name(&self) -> &str {
        "usb"
    }

    fn init(&mut self) -> KernelResult {
        self.init()
    }
}