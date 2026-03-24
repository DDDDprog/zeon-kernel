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
use crate::fs::VFS;
use crate::memory::uaccess::copy_from_user_slice;
use crate::memory::uaccess::cstr::UserCStr;
use crate::process::fd_table::Fd;
use crate::sched::syscall_ctx::ProcessCtx;
use alloc::sync::Arc;
use alloc::vec;
use bitflags::bitflags;
use core::ffi::c_char;
use libkernel::error::{KernelError, Result};
use libkernel::fs::Inode;
use libkernel::fs::path::Path;
use libkernel::memory::address::{TUA, UA};

bitflags! {
    pub struct SetXattrFlags: i32 {
        const CREATE = 0x1;
        const REPLACE = 0x2;
    }
}

async fn setxattr(
    node: Arc<dyn Inode>,
    name: &str,
    value: UA,
    size: usize,
    flags: i32,
) -> Result<usize> {
    let flags = match flags {
        0 => SetXattrFlags::all(),
        1 => SetXattrFlags::CREATE,
        2 => SetXattrFlags::REPLACE,
        _ => return Err(KernelError::InvalidValue),
    };
    if name.is_empty() || name.len() > 255 {
        return Err(KernelError::RangeError);
    }
    if size > 2 * 1024 * 1024 {
        return Err(KernelError::RangeError);
    }
    let mut value_vec = vec![0u8; size];
    copy_from_user_slice(value, &mut value_vec[..]).await?;
    node.setxattr(
        name,
        &value_vec,
        flags.contains(SetXattrFlags::CREATE),
        flags.contains(SetXattrFlags::REPLACE),
    )
    .await?;
    Ok(size)
}

pub async fn sys_setxattr(
    ctx: &ProcessCtx,
    path: TUA<c_char>,
    name: TUA<c_char>,
    value: UA,
    size: usize,
    flags: i32,
) -> Result<usize> {
    let mut buf = [0; 1024];

    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let task = ctx.shared().clone();

    let node = VFS.resolve_path(path, VFS.root_inode(), &task).await?;
    let mut buf = [0; 1024];
    setxattr(
        node,
        UserCStr::from_ptr(name).copy_from_user(&mut buf).await?,
        value,
        size,
        flags,
    )
    .await
}

pub async fn sys_lsetxattr(
    ctx: &ProcessCtx,
    path: TUA<c_char>,
    name: TUA<c_char>,
    value: UA,
    size: usize,
    flags: i32,
) -> Result<usize> {
    let mut buf = [0; 1024];

    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let task = ctx.shared().clone();

    let node = VFS
        .resolve_path_nofollow(path, VFS.root_inode(), &task)
        .await?;
    let mut buf = [0; 1024];
    setxattr(
        node,
        UserCStr::from_ptr(name).copy_from_user(&mut buf).await?,
        value,
        size,
        flags,
    )
    .await
}

pub async fn sys_fsetxattr(
    ctx: &ProcessCtx,
    fd: Fd,
    name: TUA<c_char>,
    value: UA,
    size: usize,
    flags: i32,
) -> Result<usize> {
    let node = {
        let task = ctx.shared().clone();
        let file = task
            .fd_table
            .lock_save_irq()
            .get(fd)
            .ok_or(KernelError::BadFd)?;

        file.inode().ok_or(KernelError::BadFd)?
    };
    let mut buf = [0; 1024];
    setxattr(
        node,
        UserCStr::from_ptr(name).copy_from_user(&mut buf).await?,
        value,
        size,
        flags,
    )
    .await
}
