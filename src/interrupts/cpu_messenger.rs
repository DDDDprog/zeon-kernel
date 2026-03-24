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
//! A module for sending messages between CPUs, utilising IPIs.

use core::task::Waker;

use super::{
    ClaimedInterrupt, InterruptConfig, InterruptDescriptor, InterruptHandler, get_interrupt_root,
};
use crate::kernel::cpu_id::CpuId;
use crate::sched::sched_task::Work;
use crate::{
    arch::ArchImpl,
    drivers::Driver,
    kernel::kpipe::KBuf,
    sched,
    sync::{OnceLock, SpinLock},
};
use alloc::{sync::Arc, vec::Vec};
use libkernel::{
    CpuOps,
    error::{KernelError, Result},
};
use log::warn;

pub enum Message {
    EnqueueWork(Arc<Work>),
    #[expect(unused)]
    WakeupTask(Waker),
}

struct CpuMessenger {
    mailboxes: SpinLock<Vec<KBuf<Message>>>,
    _irq: ClaimedInterrupt,
}

impl Driver for CpuMessenger {
    fn name(&self) -> &'static str {
        "CPU Messenger"
    }
}

impl InterruptHandler for CpuMessenger {
    fn handle_irq(&self, _desc: InterruptDescriptor) {
        while let Some(message) = CPU_MESSENGER
            .get()
            .unwrap()
            .mailboxes
            .lock_save_irq()
            .get(ArchImpl::id())
            .unwrap()
            .try_pop()
        {
            match message {
                Message::EnqueueWork(work) => sched::insert_work(work),
                Message::WakeupTask(waker) => waker.wake(),
            }
        }
    }
}

const MESSENGER_IRQ_DESC: InterruptDescriptor = InterruptDescriptor::Ipi(0);

pub fn cpu_messenger_init(num_cpus: usize) {
    let cpu_messenger = get_interrupt_root()
        .expect("Interrupt root should be available")
        .claim_interrupt(
            InterruptConfig {
                descriptor: MESSENGER_IRQ_DESC,
                trigger: super::TriggerMode::EdgeRising,
            },
            |irq| {
                let mut mailboxes = Vec::new();

                for _ in 0..num_cpus {
                    mailboxes.push(KBuf::new().expect("Could not allocate CPU mailbox"));
                }

                CpuMessenger {
                    mailboxes: SpinLock::new(mailboxes),
                    _irq: irq,
                }
            },
        )
        .expect("Could not claim messenger IRQ");

    if CPU_MESSENGER.set(cpu_messenger).is_err() {
        warn!("Attempted to initialise cpu messenger multiple times");
    }
}

pub fn message_cpu(cpu_id: CpuId, message: Message) -> Result<()> {
    let messenger = CPU_MESSENGER.get().ok_or(KernelError::InvalidValue)?;
    let irq = get_interrupt_root().ok_or(KernelError::InvalidValue)?;

    messenger
        .mailboxes
        .lock_save_irq()
        .get(cpu_id.value())
        .ok_or(KernelError::InvalidValue)?
        .try_push(message)
        .map_err(|_| KernelError::NoMemory)?;

    irq.raise_ipi(cpu_id.value());

    Ok(())
}

static CPU_MESSENGER: OnceLock<Arc<CpuMessenger>> = OnceLock::new();
