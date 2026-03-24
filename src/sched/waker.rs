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
use core::task::{RawWaker, RawWakerVTable, Waker};

use super::{
    insert_work_cross_cpu,
    sched_task::{Work, state::WakerAction},
};

unsafe fn clone_waker(data: *const ()) -> RawWaker {
    let data: *const Work = data.cast();

    unsafe { Arc::increment_strong_count(data) };

    RawWaker::new(data.cast(), &VTABLE)
}

/// Wakes the task. This does not consume the waker.
unsafe fn wake_waker_no_consume(data: *const ()) {
    let data: *const Work = data.cast();

    // Increment the strong count first so that Arc::from_raw does not
    // consume the waker's own reference.
    unsafe { Arc::increment_strong_count(data) };
    let work = unsafe { Arc::from_raw(data) };

    match work.state.wake() {
        WakerAction::Enqueue => {
            insert_work_cross_cpu(work);
        }
        WakerAction::PreventedSleep | WakerAction::None => {}
    }
}

unsafe fn wake_waker_consume(data: *const ()) {
    unsafe {
        wake_waker_no_consume(data);
        drop_waker(data);
    }
}

unsafe fn drop_waker(data: *const ()) {
    let data: *const Work = data.cast();
    unsafe { Arc::decrement_strong_count(data) };
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone_waker,
    wake_waker_consume,
    wake_waker_no_consume,
    drop_waker,
);

/// Creates a `Waker` for a given `Pid`.
pub fn create_waker(work: Arc<Work>) -> Waker {
    let raw_waker = RawWaker::new(Arc::into_raw(work).cast(), &VTABLE);

    // SAFETY: We have correctly implemented the VTable functions.
    unsafe { Waker::from_raw(raw_waker) }
}
