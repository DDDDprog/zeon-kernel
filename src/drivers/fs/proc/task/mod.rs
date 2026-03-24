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
mod fd;
// TODO: allowlist this across the codebase
#[expect(clippy::module_inception)]
mod task;
mod task_file;

use crate::drivers::fs::proc::task::task_file::{ProcTaskFileInode, TaskFileType};
use crate::drivers::fs::proc::{get_inode_id, procfs};
use crate::process::Tid;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::vec::Vec;
use async_trait::async_trait;
use libkernel::error::FsError;
use libkernel::fs::attr::{FileAttr, FilePermissions};
use libkernel::fs::{
    DirStream, Dirent, FileType, Filesystem, Inode, InodeId, PROCFS_ID, SimpleDirStream,
};

pub struct ProcTaskInode {
    id: InodeId,
    attr: FileAttr,
    tid: Tid,
    is_task_dir: bool,
}

impl ProcTaskInode {
    pub fn new(tid: Tid, is_task_dir: bool, inode_id: InodeId) -> Self {
        Self {
            id: inode_id,
            attr: FileAttr {
                file_type: FileType::Directory,
                permissions: FilePermissions::from_bits_retain(0o555),
                ..FileAttr::default()
            },
            tid,
            is_task_dir,
        }
    }
}

#[async_trait]
impl Inode for ProcTaskInode {
    fn id(&self) -> InodeId {
        self.id
    }

    async fn lookup(&self, name: &str) -> libkernel::error::Result<Arc<dyn Inode>> {
        let fs = procfs();
        let inode_id = InodeId::from_fsid_and_inodeid(
            fs.id(),
            get_inode_id(&[&self.tid.value().to_string(), name]),
        );
        if name == "fdinfo" {
            return Ok(Arc::new(fd::ProcFdInode::new(self.tid, true, inode_id)));
        } else if name == "fd" {
            return Ok(Arc::new(fd::ProcFdInode::new(self.tid, false, inode_id)));
        } else if name == "task" && !self.is_task_dir {
            return Ok(Arc::new(task::ProcTaskDirInode::new(self.tid, inode_id)));
        }
        if let Ok(file_type) = TaskFileType::try_from(name) {
            Ok(Arc::new(ProcTaskFileInode::new(
                self.tid,
                file_type,
                self.is_task_dir,
                inode_id,
            )))
        } else {
            Err(FsError::NotFound.into())
        }
    }

    async fn getattr(&self) -> libkernel::error::Result<FileAttr> {
        Ok(self.attr.clone())
    }

    async fn readdir(&self, start_offset: u64) -> libkernel::error::Result<Box<dyn DirStream>> {
        let mut entries: Vec<Dirent> = Vec::new();
        let initial_str = self.tid.value().to_string();
        entries.push(Dirent::new(
            "status".to_string(),
            InodeId::from_fsid_and_inodeid(PROCFS_ID, get_inode_id(&[&initial_str, "status"])),
            FileType::File,
            1,
        ));
        entries.push(Dirent::new(
            "comm".to_string(),
            InodeId::from_fsid_and_inodeid(PROCFS_ID, get_inode_id(&[&initial_str, "comm"])),
            FileType::File,
            2,
        ));
        entries.push(Dirent::new(
            "state".to_string(),
            InodeId::from_fsid_and_inodeid(PROCFS_ID, get_inode_id(&[&initial_str, "state"])),
            FileType::File,
            3,
        ));
        entries.push(Dirent::new(
            "cwd".to_string(),
            InodeId::from_fsid_and_inodeid(PROCFS_ID, get_inode_id(&[&initial_str, "cwd"])),
            FileType::Symlink,
            4,
        ));
        entries.push(Dirent::new(
            "stat".to_string(),
            InodeId::from_fsid_and_inodeid(PROCFS_ID, get_inode_id(&[&initial_str, "stat"])),
            FileType::File,
            5,
        ));
        entries.push(Dirent::new(
            "fd".to_string(),
            InodeId::from_fsid_and_inodeid(PROCFS_ID, get_inode_id(&[&initial_str, "fd"])),
            FileType::Directory,
            6,
        ));
        entries.push(Dirent::new(
            "fdinfo".to_string(),
            InodeId::from_fsid_and_inodeid(PROCFS_ID, get_inode_id(&[&initial_str, "fdinfo"])),
            FileType::Directory,
            7,
        ));
        entries.push(Dirent::new(
            "maps".to_string(),
            InodeId::from_fsid_and_inodeid(PROCFS_ID, get_inode_id(&[&initial_str, "maps"])),
            FileType::File,
            8,
        ));
        entries.push(Dirent::new(
            "exe".to_string(),
            InodeId::from_fsid_and_inodeid(PROCFS_ID, get_inode_id(&[&initial_str, "exe"])),
            FileType::File,
            9,
        ));
        if !self.is_task_dir {
            entries.push(Dirent::new(
                "task".to_string(),
                InodeId::from_fsid_and_inodeid(PROCFS_ID, get_inode_id(&[&initial_str, "task"])),
                FileType::Directory,
                10,
            ));
        }

        Ok(Box::new(SimpleDirStream::new(entries, start_offset)))
    }
}
