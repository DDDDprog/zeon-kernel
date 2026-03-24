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
use super::fops::FileOps;
use crate::{
    process::fd_table::select::PollFlags,
    sync::{AsyncMutexGuard, Mutex},
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{future, pin::Pin, task::Poll};
use libkernel::{
    error::Result,
    fs::{Inode, OpenFlags, path::Path, pathbuf::PathBuf},
};

pub struct FileCtx {
    pub flags: OpenFlags,
    pub pos: u64,
}

impl FileCtx {
    pub fn new(flags: OpenFlags) -> Self {
        Self { flags, pos: 0 }
    }
}

pub struct OpenFile {
    inode: Option<Arc<dyn Inode>>,
    path: Option<PathBuf>,
    state: Mutex<(Box<dyn FileOps>, FileCtx)>,
}

impl OpenFile {
    pub fn new(ops: Box<dyn FileOps>, flags: OpenFlags) -> Self {
        Self {
            state: Mutex::new((ops, FileCtx::new(flags))),
            inode: None,
            path: None,
        }
    }

    pub fn update(&mut self, inode: Arc<dyn Inode>, path: PathBuf) {
        self.inode = Some(inode);
        self.path = Some(path);
    }

    pub fn inode(&self) -> Option<Arc<dyn Inode>> {
        self.inode.clone()
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub async fn flags(&self) -> OpenFlags {
        self.state.lock().await.1.flags
    }

    pub async fn set_flags(&self, flags: OpenFlags) {
        self.state.lock().await.1.flags = flags;
    }

    pub async fn lock(&self) -> AsyncMutexGuard<'_, (Box<dyn FileOps>, FileCtx)> {
        self.state.lock().await
    }

    pub async fn poll(
        &self,
        flags: PollFlags,
    ) -> impl Future<Output = Result<PollFlags>> + Send + use<> {
        let mut futs = Vec::new();

        {
            let (ops, _) = &mut *self.lock().await;

            if flags.contains(PollFlags::POLLIN) {
                let read_fut = ops.poll_read_ready();

                futs.push(
                    Box::pin(async move { read_fut.await.map(|_| PollFlags::POLLIN) })
                        as Pin<Box<dyn Future<Output = _> + Send>>,
                );
            }

            if flags.contains(PollFlags::POLLOUT) {
                let write_fut = ops.poll_write_ready();

                futs.push(Box::pin(async move {
                    write_fut.await.map(|_| PollFlags::POLLOUT)
                }));
            }
        }

        future::poll_fn(move |cx| {
            let mut flags = PollFlags::empty();

            // If no events were requested, return immediately.
            if futs.is_empty() {
                return Poll::Ready(Ok(PollFlags::empty()));
            }

            for fut in futs.iter_mut() {
                match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(flag)) => flags.insert(flag),
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                    Poll::Pending => continue,
                }
            }

            if flags.is_empty() {
                Poll::Pending
            } else {
                Poll::Ready(Ok(flags))
            }
        })
    }
}
