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
use crate::{ArchImpl, arch::Arch, sched::syscall_ctx::ProcessCtx};
use core::sync::atomic::AtomicBool;
use libkernel::{
    error::{KernelError, Result},
    proc::caps::CapabilitiesFlags,
};

pub static CAD_ENABLED: AtomicBool = AtomicBool::new(false);

pub async fn sys_reboot(
    ctx: &ProcessCtx,
    magic: u32,
    magic2: u32,
    op: u32,
    _arg: usize,
) -> Result<usize> {
    ctx.shared()
        .creds
        .lock_save_irq()
        .caps()
        .check_capable(CapabilitiesFlags::CAP_SYS_BOOT)?;

    const LINUX_REBOOT_MAGIC1: u32 = 0xfee1_dead;
    const LINUX_REBOOT_MAGIC2: u32 = 672274793;
    const LINUX_REBOOT_MAGIC2A: u32 = 852072454;
    const LINUX_REBOOT_MAGIC2B: u32 = 369367448;
    const LINUX_REBOOT_MAGIC2C: u32 = 537993216;
    const LINUX_REBOOT_CMD_CAD_OFF: u32 = 0x0000_0000;
    const LINUX_REBOOT_CMD_CAD_ON: u32 = 0x89ab_cdef;
    // const LINUX_REBOOT_CMD_HALT: u32 = 0xcdef_0123;
    // const LINUX_REBOOT_CMD_KEXEC: u32 = 0x4558_4543;
    const LINUX_REBOOT_CMD_POWER_OFF: u32 = 0x4321_fedc;
    const LINUX_REBOOT_CMD_RESTART: u32 = 0x0123_4567;
    // const LINUX_REBOOT_CMD_RESTART2: u32 = 0xa1b2_c3d4;
    // const LINUX_REBOOT_CMD_SW_SUSPEND: u32 = 0xd000_fce1;
    if magic != LINUX_REBOOT_MAGIC1
        || (magic2 != LINUX_REBOOT_MAGIC2
            && magic2 != LINUX_REBOOT_MAGIC2A
            && magic2 != LINUX_REBOOT_MAGIC2B
            && magic2 != LINUX_REBOOT_MAGIC2C)
    {
        return Err(KernelError::InvalidValue);
    }
    match op {
        LINUX_REBOOT_CMD_POWER_OFF => {
            // User is supposed to sync first.
            ArchImpl::power_off()
        }
        LINUX_REBOOT_CMD_RESTART => ArchImpl::restart(),
        LINUX_REBOOT_CMD_CAD_ON => {
            CAD_ENABLED.store(true, core::sync::atomic::Ordering::SeqCst);
            Ok(0)
        }
        LINUX_REBOOT_CMD_CAD_OFF => {
            CAD_ENABLED.store(false, core::sync::atomic::Ordering::SeqCst);
            Ok(0)
        }
        // TODO: Implement other reboot operations.
        _ => Err(KernelError::InvalidValue),
    }
}
