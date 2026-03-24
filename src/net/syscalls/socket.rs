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
use crate::fs::fops::FileOps;
use crate::fs::open_file::OpenFile;
use crate::net::tcp::TcpSocket;
use crate::net::unix::UnixSocket;
use crate::net::{AF_INET, AF_UNIX, IPPROTO_TCP, SOCK_DGRAM, SOCK_SEQPACKET, SOCK_STREAM};
use crate::sched::syscall_ctx::ProcessCtx;
use alloc::boxed::Box;
use alloc::sync::Arc;
use libkernel::error::KernelError;
use libkernel::fs::OpenFlags;

pub const CLOSE_ON_EXEC: i32 = 0x80000;
pub const NONBLOCK: i32 = 0x800;

pub async fn sys_socket(
    ctx: &ProcessCtx,
    domain: i32,
    type_: i32,
    protocol: i32,
) -> libkernel::error::Result<usize> {
    let _close_on_exec = (type_ & CLOSE_ON_EXEC) != 0;
    let _nonblock = (type_ & NONBLOCK) != 0;
    // Mask out flags
    let type_ = type_ & !(CLOSE_ON_EXEC | NONBLOCK);
    let new_socket: Box<dyn FileOps> = match (domain, type_, protocol) {
        (AF_INET, SOCK_STREAM, 0) | (AF_INET, SOCK_STREAM, IPPROTO_TCP) => {
            Box::new(TcpSocket::new())
        }
        (AF_UNIX, SOCK_STREAM, _) => Box::new(UnixSocket::new_stream()),
        (AF_UNIX, SOCK_DGRAM, _) => Box::new(UnixSocket::new_datagram()),
        (AF_UNIX, SOCK_SEQPACKET, _) => Box::new(UnixSocket::new_seqpacket()),
        _ => return Err(KernelError::AddressFamilyNotSupported),
    };
    // TODO: Correct flags
    let open_file = OpenFile::new(new_socket, OpenFlags::empty());
    let fd = ctx
        .shared()
        .fd_table
        .lock_save_irq()
        .insert(Arc::new(open_file))?;
    Ok(fd.as_raw() as usize)
}
