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
use crate::arch::{Arch, ArchImpl};
use alloc::boxed::Box;
use alloc::vec::Vec;
use async_trait::async_trait;
use libkernel::fs::attr::FileAttr;
use libkernel::fs::{InodeId, SimpleFile};

pub struct ProcCmdlineInode {
    id: InodeId,
    attr: FileAttr,
}

impl ProcCmdlineInode {
    pub fn new(id: InodeId) -> Self {
        Self {
            id,
            attr: FileAttr {
                file_type: libkernel::fs::FileType::File,
                permissions: libkernel::fs::attr::FilePermissions::from_bits_retain(0o444),
                ..FileAttr::default()
            },
        }
    }
}

#[async_trait]
impl SimpleFile for ProcCmdlineInode {
    fn id(&self) -> InodeId {
        self.id
    }

    async fn getattr(&self) -> libkernel::error::Result<FileAttr> {
        Ok(self.attr.clone())
    }

    async fn read(&self) -> libkernel::error::Result<Vec<u8>> {
        let cmdline = ArchImpl::get_cmdline().unwrap_or_default();
        Ok(cmdline.into_bytes())
    }
}
