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
use crate::{
    drivers::{
        CharDriver, DriverManager, OpenableDevice, ReservedMajors, fs::dev::devfs,
        init::PlatformBus,
    },
    fs::{fops::FileOps, open_file::OpenFile},
    kernel_driver,
};
use alloc::string::ToString;
use alloc::{boxed::Box, sync::Arc};
use async_trait::async_trait;
use core::{future::Future, pin::Pin};
use libkernel::{
    driver::CharDevDescriptor,
    error::Result,
    fs::{OpenFlags, attr::FilePermissions},
    memory::address::UA,
};

/// `/dev/null` file operations.
struct NullFileOps;

#[async_trait]
impl FileOps for NullFileOps {
    async fn readat(&mut self, _buf: UA, _count: usize, _offset: u64) -> Result<usize> {
        // EOF
        Ok(0)
    }

    async fn writeat(&mut self, _buf: UA, count: usize, _offset: u64) -> Result<usize> {
        // Pretend we wrote everything successfully.
        Ok(count)
    }

    fn poll_read_ready(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        // Always ready to read (but will return EOF).
        Box::pin(async { Ok(()) })
    }

    fn poll_write_ready(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        // Always ready to accept writes.
        Box::pin(async { Ok(()) })
    }
}

struct NullDev;

impl OpenableDevice for NullDev {
    fn open(&self, flags: OpenFlags) -> Result<Arc<OpenFile>> {
        Ok(Arc::new(OpenFile::new(Box::new(NullFileOps), flags)))
    }
}

struct NullCharDev {
    null_dev: Arc<dyn OpenableDevice>,
}

impl NullCharDev {
    fn new() -> Result<Self> {
        // Register the /dev/null node in devfs.
        devfs().mknod(
            "null".to_string(),
            CharDevDescriptor {
                major: ReservedMajors::Null as _,
                minor: 0,
            },
            // World-writable, world-readable like on Linux.
            FilePermissions::from_bits_retain(0o666),
        )?;

        Ok(Self {
            null_dev: Arc::new(NullDev),
        })
    }
}

impl CharDriver for NullCharDev {
    fn get_device(&self, minor: u64) -> Option<Arc<dyn OpenableDevice>> {
        if minor == 0 {
            Some(self.null_dev.clone())
        } else {
            None
        }
    }
}

pub fn null_chardev_init(_bus: &mut PlatformBus, dm: &mut DriverManager) -> Result<()> {
    let cdev = NullCharDev::new()?;
    dm.register_char_driver(ReservedMajors::Null as _, Arc::new(cdev))
}

kernel_driver!(null_chardev_init);
