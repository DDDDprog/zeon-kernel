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
use crate::memory::PAGE_ALLOC;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use async_trait::async_trait;
use libkernel::fs::attr::FileAttr;
use libkernel::fs::{InodeId, SimpleFile};
use libkernel::memory::PAGE_SIZE;

pub struct ProcMeminfoInode {
    id: InodeId,
    attr: FileAttr,
}

impl ProcMeminfoInode {
    pub fn new(inode_id: InodeId) -> Self {
        Self {
            id: inode_id,
            attr: FileAttr {
                file_type: libkernel::fs::FileType::File,
                ..FileAttr::default()
            },
        }
    }
}

#[async_trait]
impl SimpleFile for ProcMeminfoInode {
    fn id(&self) -> InodeId {
        self.id
    }

    async fn getattr(&self) -> libkernel::error::Result<FileAttr> {
        Ok(self.attr.clone())
    }

    async fn read(&self) -> libkernel::error::Result<Vec<u8>> {
        // Gather memory statistics from the global page allocator.
        let page_alloc = PAGE_ALLOC.get().expect("PAGE_ALLOC must be initialised");

        let total_pages = page_alloc.total_pages();
        let free_pages = page_alloc.free_pages();

        let total_ram = (total_pages * PAGE_SIZE) / 1024;
        let free_ram = (free_pages * PAGE_SIZE) / 1204;
        let mut meminfo_content = String::new();
        meminfo_content.push_str(&format!("MemTotal: {total_ram} kB\n"));
        meminfo_content.push_str(&format!("MemFree: {free_ram} kB\n"));
        Ok(meminfo_content.into_bytes())
    }
}
