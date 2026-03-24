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
use core::{
    any::Any,
    sync::atomic::{AtomicU64, Ordering},
};

use alloc::{
    collections::btree_map::{BTreeMap, Entry},
    sync::Arc,
    vec::Vec,
};
use libkernel::{
    error::{KernelError, Result},
    fs::OpenFlags,
};
use probe::DeviceDescriptor;

use crate::{
    fs::{FilesystemDriver, open_file::OpenFile},
    interrupts::InterruptManager,
    sync::SpinLock,
};

pub mod chrdev;
pub mod display;
pub mod fdt_prober;
pub mod fs;
pub mod init;
pub mod interrupts;
pub mod probe;
pub mod rng;
pub mod rtc;
pub mod timer;
pub mod uart;

// New driver modules
pub mod video;
pub mod i2c;
pub mod spi;
pub mod gpu;
pub mod audio;
pub mod usb;
pub mod block;
pub mod net;

mod virtio_hal;

#[repr(u64)]
pub enum ReservedMajors {
    Null = 1,
    Zero = 2,
    Random = 3,
    Console = 5,
    Fb = 6,
    Uart = 10,
    End = 11,
}

pub trait Driver: Send + Sync + Any {
    fn name(&self) -> &'static str;

    fn as_interrupt_manager(self: Arc<Self>) -> Option<Arc<InterruptManager>> {
        None
    }

    fn as_filesystem_driver(self: Arc<Self>) -> Option<Arc<dyn FilesystemDriver>> {
        None
    }
}

pub trait OpenableDevice: Send + Sync {
    fn open(&self, args: OpenFlags) -> Result<Arc<OpenFile>>;
}

/// A driver that should be exposed to userspace via the VFS.
pub trait CharDriver: Send + Sync + 'static {
    /// Given a minor number, this function creates the Inode for that specific
    /// device instance. It can fail if the minor number is invalid for this
    /// driver.
    fn get_device(&self, minor: u64) -> Option<Arc<dyn OpenableDevice>>;
}

pub struct DriverManager {
    /// Every driver instance in the system.
    active_drivers: Vec<Arc<dyn Driver>>,
    _next_major: AtomicU64,
    /// Maps a major number to an instance of a CharDriver.
    char_drivers: BTreeMap<u64, Arc<dyn CharDriver>>,
}

impl DriverManager {
    pub const fn new() -> Self {
        Self {
            active_drivers: Vec::new(),
            _next_major: AtomicU64::new(ReservedMajors::End as _),
            char_drivers: BTreeMap::new(),
        }
    }

    pub fn insert_driver(&mut self, driver: Arc<dyn Driver>) {
        self.active_drivers.push(driver);
    }

    pub fn find_by_name(&self, name: &str) -> Option<Arc<dyn Driver>> {
        self.active_drivers.iter().find_map(|drv| {
            if drv.name() == name {
                Some(drv.clone())
            } else {
                None
            }
        })
    }

    pub fn _allocate_major(&self) -> u64 {
        self._next_major.fetch_add(1, Ordering::SeqCst)
    }

    pub fn register_char_driver(&mut self, major: u64, driver: Arc<dyn CharDriver>) -> Result<()> {
        match self.char_drivers.entry(major) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(driver);
                Ok(())
            }
            Entry::Occupied(_) => Err(KernelError::InUse),
        }
    }

    pub fn find_char_driver(&self, major: u64) -> Option<Arc<dyn CharDriver>> {
        self.char_drivers.get(&major).cloned()
    }
}

pub static DM: SpinLock<DriverManager> = SpinLock::new(DriverManager::new());
