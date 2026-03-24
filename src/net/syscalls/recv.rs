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
use crate::memory::uaccess::{copy_from_user, copy_to_user, copy_to_user_slice};
use crate::net::sops::RecvFlags;
use crate::net::{SocketLen, parse_sockaddr};
use crate::process::fd_table::Fd;
use crate::sched::syscall_ctx::ProcessCtx;
use libkernel::error::KernelError;
use libkernel::memory::address::{TUA, UA};

pub async fn sys_recvfrom(
    ctx: &ProcessCtx,
    fd: Fd,
    buf: UA,
    len: usize,
    flags: i32,
    addr: UA,
    addrlen: TUA<SocketLen>,
) -> libkernel::error::Result<usize> {
    let file = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;
    if flags != 0 {
        log::warn!("sys_recvfrom: flags parameter is not supported yet: {flags}");
    }

    let (ops, ctx) = &mut *file.lock().await;
    let socket = ops.as_socket().ok_or(KernelError::NotASocket)?;
    let flags = RecvFlags::from_bits(flags as u32).unwrap_or(RecvFlags::empty());
    let socket_addr = if !addr.is_null() {
        let addrlen_val = copy_from_user(addrlen).await?;
        Some(parse_sockaddr(addr, addrlen_val).await?)
    } else {
        None
    };
    let (message_len, recv_addr) = socket.recvfrom(ctx, buf, len, flags, socket_addr).await?;
    if let Some(recv_addr) = recv_addr
        && addr.is_null()
    {
        if addrlen.is_null() {
            return Err(KernelError::InvalidValue);
        }
        let addrlen_val = copy_from_user(addrlen).await?;
        let bytes = recv_addr.to_bytes();
        let to_copy = bytes.len().min(addrlen_val);
        copy_to_user_slice(&bytes[..to_copy], addr).await?;
        copy_to_user(addrlen, bytes.len()).await?;
    }
    Ok(message_len)
}
