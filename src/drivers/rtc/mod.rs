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
//! Real-time clock (RTC) drivers.
//!
//! RTCs often differ in how they represent time, so the idea is to return a [`Duration`] since the Unix epoch,
//! with each driver responsible for converting/handling hardware bugs.

pub mod pl031;

use crate::sync::OnceLock;
use alloc::sync::Arc;
use core::time::Duration;

pub trait Rtc: Send + Sync {
    /// Gets the current RTC time as a `Duration` since the Unix epoch.
    fn time(&self) -> Option<Duration>;

    /// Sets the RTC time. The provided `Duration` should represent the time since the Unix epoch.
    #[expect(unused)]
    fn set_time(&mut self, time: Duration) -> libkernel::error::Result<()>;
}

pub static RTC_DRIVER: OnceLock<Arc<dyn Rtc>> = OnceLock::new();

pub fn get_rtc() -> Option<&'static Arc<dyn Rtc>> {
    RTC_DRIVER.get()
}

fn set_rtc_driver(driver: Arc<dyn Rtc>) -> bool {
    RTC_DRIVER.set(driver).is_ok()
}
