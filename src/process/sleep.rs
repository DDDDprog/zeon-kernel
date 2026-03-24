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
use super::thread_group::signal::{InterruptResult, Interruptable};
use crate::{
    clock::timespec::TimeSpec,
    drivers::timer::{now, sleep},
    memory::uaccess::copy_to_user,
};
use core::time::Duration;
use libkernel::{
    error::{KernelError, Result},
    memory::address::TUA,
};

pub async fn sys_nanosleep(rqtp: TUA<TimeSpec>, rmtp: TUA<TimeSpec>) -> Result<usize> {
    let timespec: Duration = TimeSpec::copy_from_user(rqtp).await?.into();
    let started_at = now().unwrap();

    match sleep(timespec).interruptable().await {
        InterruptResult::Interrupted => {
            if !rmtp.is_null() {
                let elapsed = now().unwrap() - started_at;
                copy_to_user(rmtp, (timespec - elapsed).into()).await?;
            }
            Err(KernelError::Interrupted)
        }
        InterruptResult::Uninterrupted(()) => Ok(0),
    }
}

pub async fn sys_clock_nanosleep(
    _clock_id: i32,
    _flags: u32,
    rqtp: TUA<TimeSpec>,
    rmtp: TUA<TimeSpec>,
) -> Result<usize> {
    sys_nanosleep(rqtp, rmtp).await
}
