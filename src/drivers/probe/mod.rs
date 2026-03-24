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
use core::fmt::Display;

use alloc::{boxed::Box, sync::Arc};
use libkernel::error::Result;

use super::{Driver, DriverManager};

bitflags::bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct FdtFlags: u32 {
        const ACTIVE_CONSOLE = 1;
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DeviceMatchType {
    FdtCompatible(&'static str),
}

#[derive(Clone)]
pub enum DeviceDescriptor {
    Fdt(fdt_parser::Node<'static>, FdtFlags),
}

impl Display for DeviceDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeviceDescriptor::Fdt(node, _) => f.write_str(node.name),
        }
    }
}

pub type ProbeFn =
    Box<dyn Fn(&mut DriverManager, DeviceDescriptor) -> Result<Arc<dyn Driver>> + Send>;
