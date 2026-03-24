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
use crate::drivers::fs::proc::task::ProcTaskInode;
use crate::drivers::fs::proc::{get_inode_id, procfs};
use crate::process::{Tid, find_task_by_tid};
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::vec::Vec;
use async_trait::async_trait;
use libkernel::error::FsError;
use libkernel::fs::attr::FileAttr;
use libkernel::fs::{DirStream, Dirent, FileType, Filesystem, Inode, InodeId, SimpleDirStream};

pub struct ProcTaskDirInode {
    id: InodeId,
    attr: FileAttr,
    tid: Tid,
}

impl ProcTaskDirInode {
    pub fn new(tid: Tid, inode_id: InodeId) -> Self {
        Self {
            id: inode_id,
            attr: FileAttr {
                file_type: FileType::Directory,
                // Define appropriate file attributes for fdinfo.
                ..FileAttr::default()
            },
            tid,
        }
    }
}

#[async_trait]
impl Inode for ProcTaskDirInode {
    fn id(&self) -> InodeId {
        self.id
    }

    async fn getattr(&self) -> libkernel::error::Result<FileAttr> {
        Ok(self.attr.clone())
    }

    async fn lookup(&self, name: &str) -> libkernel::error::Result<Arc<dyn Inode>> {
        let tid = match name.parse::<u32>() {
            Ok(tid) => Tid(tid),
            Err(_) => return Err(FsError::NotFound.into()),
        };
        let fs = procfs();
        let inode_id = InodeId::from_fsid_and_inodeid(
            fs.id(),
            get_inode_id(&[&self.tid.value().to_string(), &tid.value().to_string()]),
        );
        find_task_by_tid(self.tid).ok_or(FsError::NotFound)?;
        Ok(Arc::new(ProcTaskInode::new(self.tid, true, inode_id)))
    }

    async fn readdir(&self, start_offset: u64) -> libkernel::error::Result<Box<dyn DirStream>> {
        let process = &find_task_by_tid(self.tid).ok_or(FsError::NotFound)?.process;
        let tasks = process.tasks.lock_save_irq();
        let mut entries = Vec::new();
        for (i, (_tid, task)) in tasks.iter().enumerate().skip(start_offset as usize) {
            let Some(task) = task.upgrade() else {
                continue;
            };
            let id = InodeId::from_fsid_and_inodeid(
                procfs().id(),
                get_inode_id(&[&self.tid.value().to_string(), &task.tid.value().to_string()]),
            );
            entries.push(Dirent {
                id,
                offset: (i + 1) as u64,
                file_type: FileType::Directory,
                name: task.tid.value().to_string(),
            });
        }
        Ok(Box::new(SimpleDirStream::new(entries, start_offset)))
    }
}
