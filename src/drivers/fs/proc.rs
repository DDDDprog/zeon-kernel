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
#![allow(clippy::module_name_repetitions)]

mod cmdline;
mod meminfo;
mod root;
mod stat;
mod task;

use crate::drivers::{Driver, FilesystemDriver};
use crate::sync::OnceLock;
use alloc::{boxed::Box, sync::Arc};
use async_trait::async_trait;
use core::hash::Hasher;
use libkernel::{
    error::{KernelError, Result},
    fs::{BlockDevice, Filesystem, Inode, PROCFS_ID},
};
use log::warn;
use root::ProcRootInode;

/// Deterministically generates an inode ID for the given path segments within the procfs filesystem.
fn get_inode_id(path_segments: &[&str]) -> u64 {
    let mut hasher = rustc_hash::FxHasher::default();
    // Ensure non-collision if other filesystems also use this method
    hasher.write(b"procfs");
    for segment in path_segments {
        hasher.write(segment.as_bytes());
    }
    let hash = hasher.finish();
    assert_ne!(hash, 0, "Generated inode ID cannot be zero");
    hash
}

pub struct ProcFs {
    root: Arc<ProcRootInode>,
}

impl ProcFs {
    fn new() -> Arc<Self> {
        let root_inode = Arc::new(ProcRootInode::new());
        Arc::new(Self { root: root_inode })
    }
}

#[async_trait]
impl Filesystem for ProcFs {
    async fn root_inode(&self) -> Result<Arc<dyn Inode>> {
        Ok(self.root.clone())
    }

    fn id(&self) -> u64 {
        PROCFS_ID
    }

    fn magic(&self) -> u64 {
        0x9fa0 // procfs magic number
    }
}

static PROCFS_INSTANCE: OnceLock<Arc<ProcFs>> = OnceLock::new();

/// Initializes and/or returns the global singleton [`ProcFs`] instance.
/// This is the main entry point for the rest of the kernel to interact with procfs.
pub fn procfs() -> Arc<ProcFs> {
    PROCFS_INSTANCE
        .get_or_init(|| {
            log::info!("procfs initialized");
            ProcFs::new()
        })
        .clone()
}

pub struct ProcFsDriver;

impl ProcFsDriver {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Driver for ProcFsDriver {
    fn name(&self) -> &'static str {
        "procfs"
    }

    fn as_filesystem_driver(self: Arc<Self>) -> Option<Arc<dyn FilesystemDriver>> {
        Some(self)
    }
}

#[async_trait]
impl FilesystemDriver for ProcFsDriver {
    async fn construct(
        &self,
        _fs_id: u64,
        device: Option<Box<dyn BlockDevice>>,
    ) -> Result<Arc<dyn Filesystem>> {
        if device.is_some() {
            warn!("procfs should not be constructed with a block device");
            return Err(KernelError::InvalidValue);
        }
        Ok(procfs())
    }
}
