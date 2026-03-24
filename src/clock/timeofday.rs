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
use super::timespec::TimeSpec;
use crate::clock::realtime::{date, set_date};
use crate::memory::uaccess::{UserCopyable, copy_from_user, copy_to_user};
use core::time::Duration;
use libkernel::{error::Result, memory::address::TUA};

#[derive(Copy, Clone)]
pub struct TimeZone {
    _tz_minuteswest: i32,
    _tz_dsttime: i32,
}

unsafe impl UserCopyable for TimeZone {}

pub async fn sys_gettimeofday(tv: TUA<TimeSpec>, tz: TUA<TimeZone>) -> Result<usize> {
    let time: TimeSpec = date().into();

    copy_to_user(tv, time).await?;

    if !tz.is_null() {
        copy_to_user(
            tz,
            TimeZone {
                _tz_minuteswest: 0,
                _tz_dsttime: 0,
            },
        )
        .await?;
    }

    Ok(0)
}

pub async fn sys_settimeofday(tv: TUA<TimeSpec>, _tz: TUA<TimeZone>) -> Result<usize> {
    // TODO: Handle timezone
    if !tv.is_null() {
        let time: TimeSpec = copy_from_user(tv).await?;
        let duration: Duration = time.into();
        set_date(duration);
    }
    Ok(0)
}
