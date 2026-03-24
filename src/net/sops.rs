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
use crate::fs::open_file::FileCtx;
use crate::net::{ShutdownHow, SockAddr};
use alloc::boxed::Box;
use async_trait::async_trait;
use bitflags::bitflags;
use libkernel::error::KernelError;
use libkernel::memory::address::UA;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct SendFlags: u32 {
        const MSG_CONFIRM = 0x800;
        const MSG_DONT_ROUTE = 0x4;
        const MSG_DONT_WAIT = 0x40;
        const MSG_EOR = 0x80;
        const MSG_MORE = 0x8000;
        const MSG_NO_SIGNAL = 0x4000;
        const MSG_OOB = 0x1;
    }
}

bitflags! {
    #[derive(Copy, Clone)]
    pub struct RecvFlags: u32 {
        // TODO: rest of flags
        const MSG_DONTWAIT = 0x40;
    }
}

#[async_trait]
pub trait SocketOps: Send + Sync {
    async fn bind(&self, _addr: SockAddr) -> libkernel::error::Result<()> {
        Err(KernelError::NotSupported)
    }

    async fn connect(&self, _addr: SockAddr) -> libkernel::error::Result<()> {
        Err(KernelError::NotSupported)
    }

    async fn listen(&self, _backlog: i32) -> libkernel::error::Result<()> {
        Err(KernelError::NotSupported)
    }

    async fn accept(&self) -> libkernel::error::Result<(Box<dyn SocketOps>, SockAddr)> {
        Err(KernelError::NotSupported)
    }

    async fn recv(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
        flags: RecvFlags,
    ) -> libkernel::error::Result<usize>;
    async fn recvfrom(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
        flags: RecvFlags,
        addr: Option<SockAddr>,
    ) -> libkernel::error::Result<(usize, Option<SockAddr>)>;
    async fn send(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
        flags: SendFlags,
    ) -> libkernel::error::Result<usize>;
    async fn sendto(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
        flags: SendFlags,
        addr: SockAddr,
    ) -> libkernel::error::Result<usize>;

    async fn shutdown(&self, _how: ShutdownHow) -> libkernel::error::Result<()> {
        Err(KernelError::NotSupported)
    }

    fn as_file(self: Box<Self>) -> Box<dyn FileOps>;
}

#[async_trait]
impl<T> FileOps for T
where
    T: SocketOps,
{
    async fn read(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
    ) -> libkernel::error::Result<usize> {
        self.recv(ctx, buf, count, RecvFlags::empty()).await
    }

    async fn readat(
        &mut self,
        _buf: UA,
        _count: usize,
        _offset: u64,
    ) -> libkernel::error::Result<usize> {
        Err(KernelError::NotSupported)
    }

    async fn write(
        &mut self,
        ctx: &mut FileCtx,
        buf: UA,
        count: usize,
    ) -> libkernel::error::Result<usize> {
        self.send(ctx, buf, count, SendFlags::empty()).await
    }

    async fn writeat(
        &mut self,
        _buf: UA,
        _count: usize,
        _offset: u64,
    ) -> libkernel::error::Result<usize> {
        Err(KernelError::NotSupported)
    }

    fn as_socket(&mut self) -> Option<&mut dyn SocketOps> {
        Some(self)
    }
}
