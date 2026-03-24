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

use libkernel::{error::Result, fs::path::Path, memory::address::TUA};

use crate::{
    fs::{
        VFS,
        syscalls::at::{AtFlags, resolve_at_start_node},
    },
    memory::uaccess::cstr::UserCStr,
    process::fd_table::Fd,
    sched::syscall_ctx::ProcessCtx,
};

// As defined in linux/fcntl.h ─ enables directory removal via unlinkat.
const AT_REMOVEDIR: u32 = 0x200;

/// unlinkat(2) implementation.
///
/// The semantics are:
/// - If `flags & AT_REMOVEDIR` is set, behave like `rmdir`.
/// - Otherwise behave like `unlink`.
pub async fn sys_unlinkat(
    ctx: &ProcessCtx,
    dirfd: Fd,
    path: TUA<c_char>,
    flags: u32,
) -> Result<usize> {
    // Copy the user-provided path into kernel memory.
    let mut buf = [0u8; 1024];
    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);

    let task = ctx.shared().clone();

    // Determine the starting inode for path resolution.
    let flags = AtFlags::from_bits_retain(flags as _);
    let start_node = resolve_at_start_node(ctx, dirfd, path, flags).await?;

    let remove_dir = flags.bits() as u32 & AT_REMOVEDIR != 0;

    VFS.unlink(path, start_node, remove_dir, &task).await?;

    Ok(0)
}
