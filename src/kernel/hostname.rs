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
use crate::memory::uaccess::cstr::UserCStr;
use crate::sched::syscall_ctx::ProcessCtx;
use crate::sync::OnceLock;
use crate::sync::SpinLock;
use alloc::string::{String, ToString};
use alloc::vec;
use core::ffi::c_char;
use libkernel::error::{KernelError, Result};
use libkernel::memory::address::TUA;
use libkernel::proc::caps::CapabilitiesFlags;

static HOSTNAME: OnceLock<SpinLock<String>> = OnceLock::new();

pub fn hostname() -> &'static SpinLock<String> {
    HOSTNAME.get_or_init(|| SpinLock::new(String::from("Zeon-machine")))
}

const HOST_NAME_MAX: usize = 64;

pub async fn sys_sethostname(
    ctx: &ProcessCtx,
    name_ptr: TUA<c_char>,
    name_len: usize,
) -> Result<usize> {
    {
        let creds = ctx.shared().creds.lock_save_irq();
        creds
            .caps()
            .check_capable(CapabilitiesFlags::CAP_SYS_ADMIN)?;
    }

    if name_len > HOST_NAME_MAX {
        return Err(KernelError::NameTooLong);
    }
    let mut buf = vec![0u8; name_len];
    let name = UserCStr::from_ptr(name_ptr)
        .copy_from_user(&mut buf)
        .await?;
    *hostname().lock_save_irq() = name.to_string();
    Ok(0)
}

// pub async fn sys_gethostname(name_ptr: TUA<c_char>, name_len: usize) -> Result<usize> {
//     let hostname = hostname().lock_save_irq();
//     let bytes = hostname.as_bytes();
//     let len = core::cmp::min(bytes.len(), name_len);
//     copy_to_user_slice(&bytes[..len], name_ptr.to_untyped()).await?;
//     // Null-terminate if there's space
//     if name_len > len {
//         copy_to_user(name_ptr.add_bytes(len), 0u8).await?;
//     }
//     Ok(0)
// }
