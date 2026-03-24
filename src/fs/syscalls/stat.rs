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
use super::at::stat::Stat;
use crate::memory::uaccess::copy_to_user;
use crate::{process::fd_table::Fd, sched::syscall_ctx::ProcessCtx};
use libkernel::error::Result;
use libkernel::{error::KernelError, memory::address::TUA};

pub async fn sys_fstat(ctx: &ProcessCtx, fd: Fd, statbuf: TUA<Stat>) -> Result<usize> {
    let fd = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let inode = fd.inode().ok_or(KernelError::BadFd)?;

    let attr = inode.getattr().await?;

    copy_to_user(statbuf, attr.into()).await?;

    Ok(0)
}
