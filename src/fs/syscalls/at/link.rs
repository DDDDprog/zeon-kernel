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
    fs::{FileType, path::Path},
    memory::address::TUA,
    proc::caps::CapabilitiesFlags,
};

use crate::{
    fs::{
        VFS,
        syscalls::at::{AtFlags, resolve_at_start_node, resolve_path_flags},
    },
    memory::uaccess::cstr::UserCStr,
    process::fd_table::Fd,
    sched::syscall_ctx::ProcessCtx,
};

pub async fn sys_linkat(
    ctx: &ProcessCtx,
    old_dirfd: Fd,
    old_path: TUA<c_char>,
    new_dirfd: Fd,
    new_path: TUA<c_char>,
    flags: i32,
) -> Result<usize> {
    let mut buf = [0; 1024];
    let mut buf2 = [0; 1024];

    let task = ctx.shared().clone();
    let mut flags = AtFlags::from_bits_retain(flags);

    // following symlinks is implied for any other syscall.
    // for linkat though, we need to specify nofollow since
    // linkat implicitly does not follow symlinks unless specified.
    if !flags.contains(AtFlags::AT_SYMLINK_FOLLOW) {
        flags.insert(AtFlags::AT_SYMLINK_NOFOLLOW);
    }

    if flags.contains(AtFlags::AT_EMPTY_PATH)
        && !task
            .creds
            .lock_save_irq()
            .caps()
            .is_capable(CapabilitiesFlags::CAP_DAC_READ_SEARCH)
    {
        return Err(FsError::NotFound.into()); // weird error but thats what linkat(2) says
    }

    let old_path = Path::new(
        UserCStr::from_ptr(old_path)
            .copy_from_user(&mut buf)
            .await?,
    );
    let new_path = Path::new(
        UserCStr::from_ptr(new_path)
            .copy_from_user(&mut buf2)
            .await?,
    );
    let old_start_node = resolve_at_start_node(ctx, old_dirfd, old_path, flags).await?;
    let new_start_node = resolve_at_start_node(ctx, new_dirfd, new_path, flags).await?;

    let target_inode =
        resolve_path_flags(old_dirfd, old_path, old_start_node.clone(), &task, flags).await?;

    let attr = target_inode.getattr().await?;

    if attr.file_type == FileType::Directory {
        return Err(FsError::IsADirectory.into());
    }

    // newpath does not follow flags, and doesnt follow symlinks either
    if VFS
        .resolve_path_nofollow(new_path, new_start_node.clone(), &task)
        .await
        .is_ok()
    {
        return Err(FsError::AlreadyExists.into());
    }

    // parent newpath should follow symlinks though
    let parent_inode = if let Some(parent) = new_path.parent() {
        VFS.resolve_path(parent, new_start_node, &task).await?
    } else {
        new_start_node
    };

    if parent_inode.getattr().await?.file_type != FileType::Directory {
        return Err(FsError::NotADirectory.into());
    }

    VFS.link(
        target_inode,
        parent_inode,
        new_path.file_name().ok_or(KernelError::InvalidValue)?,
    )
    .await?;

    Ok(0)
}
