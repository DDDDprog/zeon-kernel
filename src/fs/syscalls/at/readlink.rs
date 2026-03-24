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
use crate::{
    fs::{
        VFS,
        syscalls::at::{AtFlags, resolve_at_start_node},
    },
    memory::uaccess::{copy_to_user_slice, cstr::UserCStr},
    process::fd_table::Fd,
    sched::syscall_ctx::ProcessCtx,
};
use core::{cmp::min, ffi::c_char};
use libkernel::{
    error::{FsError, Result},
    fs::{FileType, path::Path},
    memory::address::{TUA, UA},
};

pub async fn sys_readlinkat(
    ctx: &ProcessCtx,
    dirfd: Fd,
    path: TUA<c_char>,
    buf: UA,
    size: usize,
) -> Result<usize> {
    let mut path_buf = [0; 1024];

    let task = ctx.shared().clone();
    let path = Path::new(
        UserCStr::from_ptr(path)
            .copy_from_user(&mut path_buf)
            .await?,
    );

    let start = resolve_at_start_node(ctx, dirfd, path, AtFlags::empty()).await?;
    let name = path.file_name().ok_or(FsError::InvalidInput)?;

    let parent = if let Some(p) = path.parent() {
        VFS.resolve_path_nofollow(p, start.clone(), &task).await?
    } else {
        start
    };

    let inode = parent.lookup(name).await?;
    let attr = inode.getattr().await?;

    if attr.file_type != FileType::Symlink {
        return Err(FsError::InvalidInput.into());
    }

    let target = inode.readlink().await?;
    let bytes = target.as_str().as_bytes();
    let len = min(bytes.len(), size);

    copy_to_user_slice(&bytes[..len], buf).await?;
    Ok(len)
}
