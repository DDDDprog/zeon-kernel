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
use crate::clock::ClockId;
use crate::clock::realtime::set_date;
use crate::clock::timespec::TimeSpec;
use crate::memory::uaccess::copy_from_user;
use libkernel::error::KernelError;
use libkernel::memory::address::TUA;

pub async fn sys_clock_settime(
    clockid: i32,
    time_spec: TUA<TimeSpec>,
) -> libkernel::error::Result<usize> {
    let time_spec = copy_from_user(time_spec).await?;
    if time_spec.tv_sec < 0 || time_spec.tv_nsec >= 1_000_000_000 {
        return Err(KernelError::InvalidValue);
    }
    match ClockId::try_from(clockid).map_err(|_| KernelError::InvalidValue)? {
        ClockId::Monotonic | ClockId::MonotonicCoarse | ClockId::MonotonicRaw => {
            // Monotonic clock cannot be set
            Err(KernelError::InvalidValue)
        }
        ClockId::Realtime => {
            set_date(time_spec.into());
            Ok(0)
        }
        _ => Err(KernelError::NotSupported),
    }
}
