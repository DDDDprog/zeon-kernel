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
use crate::arch::{Arch, ArchImpl};
use crate::drivers::timer::uptime;
use crate::kernel::cpu_id::CpuId;
use crate::process::TASK_LIST;
use crate::process::clone::NUM_FORKS;
use crate::sched::sched_task::state::TaskState;
use crate::sched::{CpuStat, NUM_CONTEXT_SWITCHES, get_cpu_stat};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use async_trait::async_trait;
use core::sync::atomic::Ordering;
use libkernel::fs::attr::FileAttr;
use libkernel::fs::{InodeId, SimpleFile};

pub struct ProcStatInode {
    id: InodeId,
    attr: FileAttr,
}

impl ProcStatInode {
    pub fn new(inode_id: InodeId) -> Self {
        Self {
            id: inode_id,
            attr: FileAttr {
                file_type: libkernel::fs::FileType::File,
                ..FileAttr::default()
            },
        }
    }
}

#[async_trait]
impl SimpleFile for ProcStatInode {
    fn id(&self) -> InodeId {
        self.id
    }

    async fn getattr(&self) -> libkernel::error::Result<FileAttr> {
        Ok(self.attr.clone())
    }

    async fn read(&self) -> libkernel::error::Result<Vec<u8>> {
        let mut stat_content = String::new();

        let mut cpu_stats = Vec::new();
        for cpu_id in 0..ArchImpl::cpu_count() {
            cpu_stats.push(get_cpu_stat(CpuId::from_value(cpu_id)));
        }
        let total: CpuStat<usize> =
            cpu_stats
                .iter()
                .fold(CpuStat::default(), |acc, stat| CpuStat {
                    user: acc.user + stat.user,
                    nice: acc.nice + stat.nice,
                    system: acc.system + stat.system,
                    idle: acc.idle + stat.idle,
                    iowait: acc.iowait + stat.iowait,
                    irq: acc.irq + stat.irq,
                    softirq: acc.softirq + stat.softirq,
                    steal: acc.steal + stat.steal,
                    guest: acc.guest + stat.guest,
                    guest_nice: acc.guest_nice + stat.guest_nice,
                });
        cpu_stats.insert(0, total);
        for (i, stat) in cpu_stats.iter().enumerate() {
            let label = if i == 0 {
                "cpu".to_string()
            } else {
                format!("cpu{}", i - 1)
            };
            stat_content.push_str(&format!(
                "{label} {} {} {} {} {} {} {} {} {} {}\n",
                stat.user,
                stat.nice,
                stat.system,
                stat.idle,
                stat.iowait,
                stat.irq,
                stat.softirq,
                stat.steal,
                stat.guest,
                stat.guest_nice
            ));
        }
        let tasks = TASK_LIST.lock_save_irq();
        let mut procs_running = 0;
        let mut procs_blocked = 0;
        for task in tasks.values().filter_map(|t| t.upgrade()) {
            let state = task.state.load(Ordering::Relaxed);
            match state {
                TaskState::Running | TaskState::Runnable | TaskState::Woken => procs_running += 1,
                TaskState::Sleeping
                | TaskState::Stopped
                | TaskState::PendingSleep
                | TaskState::PendingStop => procs_blocked += 1,
                _ => {}
            }
        }
        stat_content.push_str(&format!(
            "ctxt {}\n",
            NUM_CONTEXT_SWITCHES.load(Ordering::Relaxed)
        ));
        stat_content.push_str(&format!("btime {}\n", uptime().as_secs()));
        stat_content.push_str(&format!(
            "processes {}\n",
            NUM_FORKS.load(Ordering::Relaxed)
        ));
        stat_content.push_str(&format!("procs_running {procs_running}\n",));
        stat_content.push_str(&format!("procs_blocked {procs_blocked}\n",));
        Ok(stat_content.into_bytes())
    }
}
