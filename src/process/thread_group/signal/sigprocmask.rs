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
use crate::memory::uaccess::{copy_from_user, copy_to_user};
use crate::sched::syscall_ctx::ProcessCtx;
use libkernel::error::{KernelError, Result};
use libkernel::memory::address::TUA;

use super::{SigSet, UNMASKABLE_SIGNALS};

pub const SIG_BLOCK: u32 = 0;
pub const SIG_UNBLOCK: u32 = 1;
pub const SIG_SETMASK: u32 = 2;

pub async fn sys_rt_sigprocmask(
    ctx: &mut ProcessCtx,
    how: u32,
    set: TUA<SigSet>,
    oldset: TUA<SigSet>,
    sigset_size: usize,
) -> Result<usize> {
    if sigset_size != size_of::<SigSet>() {
        return Err(KernelError::InvalidValue);
    }

    let set = if !set.is_null() {
        Some(copy_from_user(set).await?)
    } else {
        None
    };

    let old_sigmask = {
        let task = ctx.shared();
        let old_sigmask = task.sig_mask.load();

        if let Some(set) = set {
            let mut new_sigmask = match how {
                SIG_BLOCK => old_sigmask.union(set),
                SIG_UNBLOCK => old_sigmask.difference(set),
                SIG_SETMASK => set,
                _ => return Err(KernelError::InvalidValue),
            };

            // SIGSTOP and SIGKILL can never be masked.
            new_sigmask.remove(UNMASKABLE_SIGNALS);

            task.sig_mask.store(new_sigmask);
        }

        old_sigmask
    };

    if !oldset.is_null() {
        copy_to_user(oldset, old_sigmask).await?;
    }

    Ok(0)
}
