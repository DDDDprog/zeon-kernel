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
use core::time::Duration;

use libkernel::{
    error::{KernelError, Result},
    memory::address::TUA,
};

use crate::memory::uaccess::{UserCopyable, copy_from_user};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TimeSpec {
    pub tv_sec: i64,
    pub tv_nsec: u64,
}

unsafe impl UserCopyable for TimeSpec {}

impl From<TimeSpec> for Duration {
    fn from(value: TimeSpec) -> Self {
        Duration::new(value.tv_sec as _, value.tv_nsec as _)
    }
}

impl From<Duration> for TimeSpec {
    fn from(value: Duration) -> Self {
        TimeSpec {
            tv_sec: value.as_secs() as _,
            tv_nsec: value.subsec_nanos() as _,
        }
    }
}

impl TimeSpec {
    pub async fn copy_from_user(src: TUA<Self>) -> Result<Self> {
        let timespec = copy_from_user(src).await?;

        // Sanity checking.
        if timespec.tv_nsec > 999_999_999 {
            return Err(KernelError::InvalidValue);
        }

        if timespec.tv_sec < 0 {
            return Err(KernelError::InvalidValue);
        }

        Ok(timespec)
    }
}
