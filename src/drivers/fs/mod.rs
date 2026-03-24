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
use alloc::sync::Arc;
use cgroup::CgroupFsDriver;
use dev::DevFsDriver;
use ext4::Ext4FsDriver;
use fat32::Fat32FsDriver;
use proc::ProcFsDriver;
use sys::SysFsDriver;
use tmpfs::TmpFsDriver;

use super::DM;

pub mod cgroup;
pub mod dev;
pub mod ext4;
pub mod fat32;
pub mod proc;
pub mod sys;
pub mod tmpfs;

pub fn register_fs_drivers() {
    let mut dm = DM.lock_save_irq();

    dm.insert_driver(Arc::new(Ext4FsDriver::new()));
    dm.insert_driver(Arc::new(Fat32FsDriver::new()));
    dm.insert_driver(Arc::new(DevFsDriver::new()));
    dm.insert_driver(Arc::new(ProcFsDriver::new()));
    dm.insert_driver(Arc::new(SysFsDriver::new()));
    dm.insert_driver(Arc::new(TmpFsDriver::new()));
    dm.insert_driver(Arc::new(CgroupFsDriver::new()));
}
