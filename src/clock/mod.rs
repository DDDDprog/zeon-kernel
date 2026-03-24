/*
 *  ███████╗███████╗ ██████╗ ███╗   ██╗
 *  ╚══███╔╝██╔════╝██╔═══██╗████╗  ██║
 *    ███╔╝ █████╗  ██║   ██║██╔██╗ ██║
 *   ███╔╝  ██╔══╝  ██║   ██║██║╚██╗██║
 *  ███████╗███████╗╚██████╔╝██║ ╚████║
 *  ╚══════╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝
 *
 * Zeon - Pure Rust Operating System
 * https://github.com/DDDDprog/zeon-kernel
 */

// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

   _/ ___| 
 * | |_) | |_) |  / _ \ \___ \ | | \___ \ 
 * |  __/|  _ <  / ___ \ ___) || |  ___) |
 * |_|| \_\/_/   \____/ |_| |____/ 
 *
 * Zeon
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
pub mod gettime;
pub mod realtime;
pub mod settime;
pub mod timeofday;
pub mod timespec;

pub enum ClockId {
    Realtime = 0,
    Monotonic = 1,
    ProcessCpuTimeId = 2,
    ThreadCpuTimeId = 3,
    MonotonicRaw = 4,
    RealtimeCoarse = 5,
    MonotonicCoarse = 6,
    BootTime = 7,
    RealtimeAlarm = 8,
    BootTimeAlarm = 9,
    Tai = 11,
}

impl TryFrom<i32> for ClockId {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ClockId::Realtime),
            1 => Ok(ClockId::Monotonic),
            2 => Ok(ClockId::ProcessCpuTimeId),
            3 => Ok(ClockId::ThreadCpuTimeId),
            4 => Ok(ClockId::MonotonicRaw),
            5 => Ok(ClockId::RealtimeCoarse),
            6 => Ok(ClockId::MonotonicCoarse),
            7 => Ok(ClockId::BootTime),
            8 => Ok(ClockId::RealtimeAlarm),
            9 => Ok(ClockId::BootTimeAlarm),
            11 => Ok(ClockId::Tai),
            _ => Err(()),
        }
    }
}
