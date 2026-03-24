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
use core::ffi::c_char;

use libkernel::{
    error::{FsError, KernelError, Result},
    fs::{
        attr::{AccessMode, FileAttr},
        path::Path,
    },
    memory::address::TUA,
    proc::caps::CapabilitiesFlags,
};
use ringbuf::Arc;

use crate::process::Task;
use crate::{
    clock::{realtime::date, timespec::TimeSpec},
    fs::syscalls::at::{AtFlags, resolve_at_start_node, resolve_path_flags},
    memory::uaccess::{copy_from_user, cstr::UserCStr},
    process::fd_table::Fd,
    sched::syscall_ctx::ProcessCtx,
};

const UTIME_NOW: u64 = (1 << 30) - 1;
const UTIME_OMIT: u64 = (1 << 30) - 2;

pub async fn sys_utimensat(
    ctx: &ProcessCtx,
    dirfd: Fd,
    path: TUA<c_char>,
    times: TUA<[TimeSpec; 2]>,
    flags: i32,
) -> Result<usize> {
    let task = ctx.shared().clone();

    // linux specifically uses NULL path to indicate futimens, see utimensat(2)
    let node = if path.is_null() {
        task.fd_table
            .lock_save_irq()
            .get(dirfd)
            .ok_or(KernelError::BadFd)?
            .inode()
            .ok_or(KernelError::BadFd)?
    } else {
        let mut buf = [0; 1024];

        let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
        let flags = AtFlags::from_bits_retain(flags);
        let start_node = resolve_at_start_node(ctx, dirfd, path, flags).await?;

        resolve_path_flags(dirfd, path, start_node, &task, flags).await?
    };

    let mut attr = node.getattr().await?;

    if times.is_null() {
        test_creds(task, &attr)?;
        attr.atime = date();
        attr.mtime = date();
        attr.ctime = date();
    } else {
        let times = copy_from_user(times).await?;
        if times[0].tv_nsec == UTIME_NOW && times[1].tv_nsec == UTIME_NOW {
            test_creds(task, &attr)?;
        } else if times[0].tv_nsec != UTIME_OMIT && times[1].tv_nsec != UTIME_OMIT {
            let creds = task.creds.lock_save_irq();
            if creds.euid() != attr.uid
                && !creds.caps().is_capable(CapabilitiesFlags::CAP_FOWNER)
                && !creds.caps().is_capable(CapabilitiesFlags::CAP_DAC_OVERRIDE)
            {
                return Err(FsError::PermissionDenied.into());
            }
        }

        let atime = match times[0].tv_nsec {
            UTIME_NOW => date(),
            UTIME_OMIT => attr.atime,
            _ => times[0].into(),
        };
        let mtime = match times[1].tv_nsec {
            UTIME_NOW => date(),
            UTIME_OMIT => attr.mtime,
            _ => times[1].into(),
        };

        attr.atime = atime;
        attr.mtime = mtime;
        attr.ctime = date();
    }

    node.setattr(attr).await?;

    Ok(0)
}

fn test_creds(task: Arc<Task>, attr: &FileAttr) -> Result<()> {
    let creds = task.creds.lock_save_irq();
    if attr
        .check_access(creds.uid(), creds.gid(), creds.caps(), AccessMode::W_OK)
        .is_err()
        && creds.euid() != attr.uid
        && !creds.caps().is_capable(CapabilitiesFlags::CAP_FOWNER)
        && !creds.caps().is_capable(CapabilitiesFlags::CAP_DAC_OVERRIDE)
    {
        Err(FsError::PermissionDenied.into())
    } else {
        Ok(())
    }
}
