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
use crate::arch::ArchImpl;

pub mod per_cpu;

pub type SpinLock<T> = libkernel::sync::spinlock::SpinLockIrq<T, ArchImpl>;
pub type Mutex<T> = libkernel::sync::mutex::Mutex<T, ArchImpl>;
pub type AsyncMutexGuard<'a, T> = libkernel::sync::mutex::AsyncMutexGuard<'a, T, ArchImpl>;
#[expect(dead_code)]
pub type Rwlock<T> = libkernel::sync::rwlock::Rwlock<T, ArchImpl>;
#[expect(dead_code)]
pub type AsyncRwlockReadGuard<'a, T> =
    libkernel::sync::rwlock::AsyncRwlockReadGuard<'a, T, ArchImpl>;
#[expect(dead_code)]
pub type AsyncRwlockWriteGuard<'a, T> =
    libkernel::sync::rwlock::AsyncRwlockWriteGuard<'a, T, ArchImpl>;
pub type OnceLock<T> = libkernel::sync::once_lock::OnceLock<T, ArchImpl>;
pub type CondVar<T> = libkernel::sync::condvar::CondVar<T, ArchImpl>;
// pub type Reciever<T> = libkernel::sync::mpsc::Reciever<T, ArchImpl>;
// pub type Sender<T> = libkernel::sync::mpsc::Sender<T, ArchImpl>;

// pub fn channel<T: Send>() -> (Sender<T>, Reciever<T>) {
//     libkernel::sync::mpsc::channel()
// }
