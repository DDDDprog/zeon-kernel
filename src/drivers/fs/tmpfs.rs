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
    arch::ArchImpl,
    drivers::Driver,
    fs::FilesystemDriver,
    memory::{PageOffsetTranslator, page::PgAllocGetter},
};
use alloc::{boxed::Box, sync::Arc};
use async_trait::async_trait;
use libkernel::{
    error::{KernelError, Result},
    fs::{BlockDevice, Filesystem},
};
use log::warn;

pub struct TmpFsDriver {}

impl TmpFsDriver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Driver for TmpFsDriver {
    fn name(&self) -> &'static str {
        "tmpfs"
    }

    fn as_filesystem_driver(self: Arc<Self>) -> Option<Arc<dyn FilesystemDriver>> {
        Some(self)
    }
}

#[async_trait]
impl FilesystemDriver for TmpFsDriver {
    async fn construct(
        &self,
        fs_id: u64,
        device: Option<Box<dyn BlockDevice>>,
    ) -> Result<Arc<dyn Filesystem>> {
        match device {
            Some(_) => {
                warn!("Unexpected block device for tmpfs");
                Err(KernelError::InvalidValue)
            }
            None => Ok(libkernel::fs::filesystems::tmpfs::TmpFs::<
                ArchImpl,
                PgAllocGetter,
                PageOffsetTranslator,
            >::new(fs_id)),
        }
    }
}
