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
    fs::{
        fops::FileOps,
        open_file::{FileCtx, OpenFile},
    },
    kernel::rand::fill_random_bytes,
    kernel_driver,
    memory::uaccess::copy_to_user_slice,
};
use alloc::{boxed::Box, string::ToString, sync::Arc, vec};
use async_trait::async_trait;
use core::{future::Future, pin::Pin};
use libkernel::{
    driver::CharDevDescriptor,
    error::Result,
    fs::{OpenFlags, attr::FilePermissions},
    memory::address::UA,
};

struct RandomFileOps;

#[async_trait]
impl FileOps for RandomFileOps {
    async fn read(&mut self, _ctx: &mut FileCtx, buf: UA, count: usize) -> Result<usize> {
        self.readat(buf, count, 0).await
    }

    async fn writeat(&mut self, _buf: UA, count: usize, _offset: u64) -> Result<usize> {
        // Just consume the write.
        Ok(count)
    }

    async fn readat(&mut self, buf: UA, count: usize, _offset: u64) -> Result<usize> {
        // TODO: Add an implementation of `/dev/urandom` which doesn't block if
        // the entropy pool hasn't yet been seeded.
        let mut kbuf = vec![0u8; count];
        fill_random_bytes(&mut kbuf).await;
        copy_to_user_slice(&kbuf, buf).await?;
        Ok(count)
    }

    fn poll_read_ready(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        Box::pin(async { Ok(()) })
    }
}

struct RandomDev;

impl OpenableDevice for RandomDev {
    fn open(&self, flags: OpenFlags) -> Result<Arc<OpenFile>> {
        Ok(Arc::new(OpenFile::new(Box::new(RandomFileOps), flags)))
    }
}

struct RandomCharDev {
    random_dev: Arc<dyn OpenableDevice>,
}

impl RandomCharDev {
    fn new() -> Result<Self> {
        devfs().mknod(
            "random".to_string(),
            CharDevDescriptor {
                major: ReservedMajors::Random as _,
                minor: 0,
            },
            FilePermissions::from_bits_retain(0o666),
        )?;

        Ok(Self {
            random_dev: Arc::new(RandomDev),
        })
    }
}

impl CharDriver for RandomCharDev {
    fn get_device(&self, minor: u64) -> Option<Arc<dyn OpenableDevice>> {
        if minor == 0 {
            Some(self.random_dev.clone())
        } else {
            None
        }
    }
}

/// Driver initialisation entry point invoked during kernel boot.
pub fn random_chardev_init(_bus: &mut PlatformBus, dm: &mut DriverManager) -> Result<()> {
    let cdev = RandomCharDev::new()?;
    dm.register_char_driver(ReservedMajors::Random as _, Arc::new(cdev))
}

kernel_driver!(random_chardev_init);
