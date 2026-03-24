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
    process::{
        Tid,
        thread_group::{Pgid, Tgid, ThreadGroup, pid::PidT},
    },
    sched::syscall_ctx::ProcessCtx,
};

use super::{SigId, uaccess::UserSigId};
use crate::process::thread_group::TG_LIST;
use libkernel::error::{KernelError, Result};

pub fn sys_kill(ctx: &ProcessCtx, pid: PidT, signal: UserSigId) -> Result<usize> {
    let signal: SigId = signal.try_into()?;

    let current_task = ctx.shared();
    // Kill ourselves
    if pid == current_task.process.tgid.value() as PidT {
        current_task.process.deliver_signal(signal);

        return Ok(0);
    }

    match pid {
        p if p > 0 => {
            let target_tg = ThreadGroup::get(Tgid(p as _)).ok_or(KernelError::NoProcess)?;
            target_tg.deliver_signal(signal);
        }

        0 => {
            let our_pgid = *current_task.process.pgid.lock_save_irq();
            // Iterate over all thread groups and signal the ones that are in
            // the same PGID.
            for tg_weak in crate::process::thread_group::TG_LIST
                .lock_save_irq()
                .values()
            {
                if let Some(tg) = tg_weak.upgrade()
                    && *tg.pgid.lock_save_irq() == our_pgid
                {
                    tg.deliver_signal(signal);
                }
            }
        }

        p if p < 0 && p != -1 => {
            let target_pgid = Pgid((-p) as _);
            for tg_weak in crate::process::thread_group::TG_LIST
                .lock_save_irq()
                .values()
            {
                if let Some(tg) = tg_weak.upgrade()
                    && *tg.pgid.lock_save_irq() == target_pgid
                {
                    tg.deliver_signal(signal);
                }
            }
        }

        _ => return Err(KernelError::NotSupported),
    }

    Ok(0)
}

pub fn sys_tkill(ctx: &ProcessCtx, tid: PidT, signal: UserSigId) -> Result<usize> {
    let target_tid = Tid(tid as _);
    let current_task = ctx.shared();

    let signal: SigId = signal.try_into()?;

    // The fast-path case.
    if current_task.tid == target_tid {
        current_task
            .process
            .pending_signals
            .lock_save_irq()
            .set_signal(signal);
    } else {
        let task = current_task
            .process
            .tasks
            .lock_save_irq()
            .get(&target_tid)
            .and_then(|t| t.upgrade())
            .ok_or(KernelError::NoProcess)?;

        task.process
            .pending_signals
            .lock_save_irq()
            .set_signal(signal);
    }

    Ok(0)
}

pub fn send_signal_to_pg(pgid: Pgid, signal: SigId) {
    for tg_weak in TG_LIST.lock_save_irq().values() {
        if let Some(tg) = tg_weak.upgrade()
            && *tg.pgid.lock_save_irq() == pgid
        {
            tg.deliver_signal(signal);
        }
    }
}
