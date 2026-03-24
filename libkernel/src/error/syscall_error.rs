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
use crate::error::FsError;

use super::KernelError;

pub const EPERM: isize = -1;
pub const ENOENT: isize = -2;
pub const ESRCH: isize = -3;
pub const EINTR: isize = -4;
pub const EIO: isize = -5;
pub const ENXIO: isize = -6;
pub const E2BIG: isize = -7;
pub const ENOEXEC: isize = -8;
pub const EBADF: isize = -9;
pub const ECHILD: isize = -10;
pub const EAGAIN: isize = -11;
pub const ENOMEM: isize = -12;
pub const EACCES: isize = -13;
pub const EFAULT: isize = -14;
pub const ENOTBLK: isize = -15;
pub const EBUSY: isize = -16;
pub const EEXIST: isize = -17;
pub const EXDEV: isize = -18;
pub const ENODEV: isize = -19;
pub const ENOTDIR: isize = -20;
pub const EISDIR: isize = -21;
pub const EINVAL: isize = -22;
pub const ENFILE: isize = -23;
pub const EMFILE: isize = -24;
pub const ENOTTY: isize = -25;
pub const ETXTBSY: isize = -26;
pub const EFBIG: isize = -27;
pub const ENOSPC: isize = -28;
pub const ESPIPE: isize = -29;
pub const EROFS: isize = -30;
pub const EMLINK: isize = -31;
pub const EPIPE: isize = -32;
pub const EDOM: isize = -33;
pub const ERANGE: isize = -34;
pub const EWOULDBLOCK: isize = -EAGAIN;
pub const ENOSYS: isize = -38;
pub const EOPNOTSUPP: isize = -95;
pub const ETIMEDOUT: isize = -110;

pub fn kern_err_to_syscall(err: KernelError) -> isize {
    match err {
        KernelError::BadFd => EBADF,
        KernelError::InvalidValue => EINVAL,
        KernelError::Fault => EFAULT,
        KernelError::TryAgain => EAGAIN,
        KernelError::BrokenPipe => EPIPE,
        KernelError::Fs(FsError::NotFound) => ENOENT,
        KernelError::Fs(FsError::IsADirectory) => EISDIR,
        KernelError::Fs(FsError::NotADirectory) => ENOTDIR,
        KernelError::Fs(FsError::AlreadyExists) => EEXIST,
        KernelError::Fs(FsError::InvalidInput) => EINVAL, // TODO: Is this right?
        KernelError::NotATty => ENOTTY,
        KernelError::SeekPipe => ESPIPE,
        KernelError::NotSupported => ENOSYS,
        KernelError::NoMemory => ENOMEM,
        KernelError::TimedOut => ETIMEDOUT,
        KernelError::RangeError => ERANGE,
        KernelError::NoChildProcess => ECHILD,
        KernelError::OpNotSupported => EOPNOTSUPP,
        KernelError::Interrupted => EINTR,
        KernelError::NoProcess => ESRCH,
        e => todo!("{e}"),
    }
}
