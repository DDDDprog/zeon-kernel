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
use crate::net::sops::SendFlags;
use crate::net::{SocketLen, parse_sockaddr};
use crate::process::fd_table::Fd;
use crate::sched::syscall_ctx::ProcessCtx;
use libkernel::error::Result;
use libkernel::memory::address::UA;

// pub async fn sys_send(fd: Fd, buf: UA, len: usize, flags: i32) -> Result<usize> {
//     let file = crate::sched::current::current_task()
//         .fd_table
//         .lock_save_irq()
//         .get(fd)
//         .ok_or(libkernel::error::KernelError::BadFd)?;
//     if flags != 0 {
//         log::warn!("sys_send: flags parameter is not supported yet: {}", flags);
//     }
//
//     let (ops, ctx) = &mut *file.lock().await;
//     let socket = ops
//         .as_socket()
//         .ok_or(libkernel::error::KernelError::NotASocket)?;
//     let flags = SendFlags::from_bits(flags as u32).unwrap_or(SendFlags::empty());
//     socket.send(ctx, buf, len, flags).await
// }

const MSG_NOSIGNAL: i32 = 0x4000;

pub async fn sys_sendto(
    ctx: &ProcessCtx,
    fd: Fd,
    buf: UA,
    len: usize,
    flags: i32,
    addr: UA,
    addrlen: SocketLen,
) -> Result<usize> {
    let file = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(libkernel::error::KernelError::BadFd)?;
    if flags != 0 && flags != MSG_NOSIGNAL {
        log::warn!("sys_sendto: flags parameter is not supported yet: {flags}");
    }

    let (ops, ctx) = &mut *file.lock().await;
    let socket = ops
        .as_socket()
        .ok_or(libkernel::error::KernelError::NotASocket)?;
    let flags = SendFlags::from_bits(flags as u32).unwrap_or(SendFlags::empty());
    if addr.is_null() || addrlen == 0 {
        // No destination address, use connected peer
        return socket.send(ctx, buf, len, flags).await;
    }
    let addr = parse_sockaddr(addr, addrlen).await?;
    socket.sendto(ctx, buf, len, flags, addr).await
}
