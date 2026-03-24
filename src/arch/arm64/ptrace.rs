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
use crate::memory::uaccess::UserCopyable;

use super::exceptions::ExceptionState;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Arm64PtraceGPRegs {
    pub x: [u64; 31], // x0-x30
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
}

unsafe impl UserCopyable for Arm64PtraceGPRegs {}

impl From<&ExceptionState> for Arm64PtraceGPRegs {
    fn from(value: &ExceptionState) -> Self {
        Self {
            x: value.x,
            sp: value.sp_el0,
            pc: value.elr_el1,
            pstate: value.spsr_el1,
        }
    }
}
