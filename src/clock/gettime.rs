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
use core::sync::atomic::Ordering;
use core::time::Duration;
use libkernel::{
    error::{KernelError, Result},
    memory::address::TUA,
};

use super::{ClockId, realtime::date, timespec::TimeSpec};
use crate::drivers::timer::{Instant, now};
use crate::sched::syscall_ctx::ProcessCtx;
use crate::{drivers::timer::uptime, memory::uaccess::copy_to_user};

pub async fn sys_clock_gettime(
    ctx: &ProcessCtx,
    clockid: i32,
    time_spec: TUA<TimeSpec>,
) -> Result<usize> {
    let time = match ClockId::try_from(clockid).map_err(|_| KernelError::InvalidValue)? {
        ClockId::Realtime => date(),
        ClockId::Monotonic => uptime(),
        ClockId::ProcessCpuTimeId => {
            let task = ctx.shared();
            let total_time = task.process.stime.load(Ordering::Relaxed) as u64
                + task.process.utime.load(Ordering::Relaxed) as u64;
            let last_update = Instant::from_user_normalized(
                task.process.last_account.load(Ordering::Relaxed) as u64,
            );
            let now = now().unwrap();
            let delta = now - last_update;
            Duration::from(Instant::from_user_normalized(total_time)) + delta
        }
        ClockId::ThreadCpuTimeId => {
            let task = ctx.shared();
            let total_time = task.stime.load(Ordering::Relaxed) as u64
                + task.utime.load(Ordering::Relaxed) as u64;
            let last_update =
                Instant::from_user_normalized(task.last_account.load(Ordering::Relaxed) as u64);
            let now = now().unwrap();
            let delta = now - last_update;
            Duration::from(Instant::from_user_normalized(total_time)) + delta
        }
        _ => return Err(KernelError::InvalidValue),
    };

    copy_to_user(time_spec, time.into()).await?;

    Ok(0)
}
