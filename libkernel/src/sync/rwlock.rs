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
use super::spinlock::SpinLockIrq;
use crate::CpuOps;
use crate::sync::mutex::Mutex;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

struct RwlockState<CPU: CpuOps> {
    num_readers: SpinLockIrq<usize, CPU>,
    writer_lock: Mutex<(), CPU>,
}

/// An asynchronous, rwlock primitive.
///
/// This rwlock can be used to protect shared data across asynchronous tasks.
/// `lock()` returns a future that resolves to a guard. When the guard is
/// dropped, the lock is released.
pub struct Rwlock<T: ?Sized, CPU: CpuOps> {
    state: RwlockState<CPU>,
    data: UnsafeCell<T>,
}

/// A guard that provides read-only access to the data in an `AsyncRwlock`.
///
/// When an `AsyncRwlockReadGuard` is dropped, it automatically decreases the
/// read count and wakes up the next task if necessary.
#[must_use = "if unused, the Rwlock will immediately unlock"]
pub struct AsyncRwlockReadGuard<'a, T: ?Sized, CPU: CpuOps> {
    rwlock: &'a Rwlock<T, CPU>,
}

/// A guard that provides exclusive access to the data in an `AsyncRwlock`.
///
/// When an `AsyncRwlockWriteGuard` is dropped, it automatically releases the lock and
/// wakes up the next task.
#[must_use = "if unused, the Rwlock will immediately unlock"]
pub struct AsyncRwlockWriteGuard<'a, T: ?Sized, CPU: CpuOps> {
    rwlock: &'a Rwlock<T, CPU>,
}

impl<T, CPU: CpuOps> Rwlock<T, CPU> {
    /// Creates a new asynchronous rwlock in an unlocked state.
    pub fn new(data: T) -> Self {
        Self {
            state: RwlockState {
                num_readers: SpinLockIrq::new(0),
                writer_lock: Mutex::new(()),
            },
            data: UnsafeCell::new(data),
        }
    }

    /// Consumes the rwlock, returning the underlying data.
    ///
    /// This is safe because consuming `self` guarantees no other code can
    /// access the rwlock.
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: ?Sized, CPU: CpuOps> Rwlock<T, CPU> {
    /// Acquires rwlock read.
    ///
    /// Returns a guard asynchronously. The guard is released when the
    /// returned [`AsyncRwlockReadGuard`] is dropped.
    pub async fn read(&self) -> AsyncRwlockReadGuard<'_, T, CPU> {
        let mut num_readers = self.state.num_readers.lock_save_irq();
        *num_readers += 1;
        if *num_readers == 1 {
            self.state.writer_lock.acquire().await;
        }
        AsyncRwlockReadGuard { rwlock: self }
    }

    /// Acquires rwlock write.
    ///
    /// Returns a guard asynchronously. The guard is released when the
    /// returned [`AsyncRwlockWriteGuard`] is dropped.
    pub async fn write(&self) -> AsyncRwlockWriteGuard<'_, T, CPU> {
        self.state.writer_lock.acquire().await;
        AsyncRwlockWriteGuard { rwlock: self }
    }
}

impl<T: ?Sized, CPU: CpuOps> Drop for AsyncRwlockReadGuard<'_, T, CPU> {
    fn drop(&mut self) {
        let mut num_readers = self.rwlock.state.num_readers.lock_save_irq();
        *num_readers -= 1;
        if *num_readers == 0 {
            unsafe { self.rwlock.state.writer_lock.release() };
        }
    }
}

impl<T: ?Sized, CPU: CpuOps> Deref for AsyncRwlockReadGuard<'_, T, CPU> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: This is safe because the existence of this guard guarantees
        // we have read access to the data without any writers.
        unsafe { &*self.rwlock.data.get() }
    }
}

impl<T: ?Sized, CPU: CpuOps> Drop for AsyncRwlockWriteGuard<'_, T, CPU> {
    fn drop(&mut self) {
        unsafe { self.rwlock.state.writer_lock.release() };
    }
}

impl<T: ?Sized, CPU: CpuOps> Deref for AsyncRwlockWriteGuard<'_, T, CPU> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: This is safe because the existence of this guard guarantees
        // we have exclusive access to the data.
        unsafe { &*self.rwlock.data.get() }
    }
}

impl<T: ?Sized, CPU: CpuOps> DerefMut for AsyncRwlockWriteGuard<'_, T, CPU> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: This is safe because the existence of this guard guarantees
        // we have exclusive access to the data.
        unsafe { &mut *self.rwlock.data.get() }
    }
}

unsafe impl<T: ?Sized + Send, CPU: CpuOps> Send for Rwlock<T, CPU> {}
unsafe impl<T: ?Sized + Send, CPU: CpuOps> Sync for Rwlock<T, CPU> {}
