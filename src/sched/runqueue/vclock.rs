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
    drivers::timer::Instant,
    sched::{VCLOCK_EPSILON, VT_FIXED_SHIFT, sched_task::RunnableTask},
};

pub struct VClock {
    last_update: Option<Instant>,
    clk: u128,
}

impl VClock {
    pub fn new() -> Self {
        Self {
            last_update: None,
            clk: 0,
        }
    }

    pub fn now(&self) -> u128 {
        self.clk
    }

    pub fn is_task_eligible(&self, tsk: &RunnableTask) -> bool {
        tsk.v_eligible.saturating_sub(self.clk) <= VCLOCK_EPSILON
    }

    /// Fast forward the clk to the specified clock, `new_clk`.
    pub fn fast_forward(&mut self, new_clk: u128) {
        self.clk = new_clk;
    }

    /// Advance the virtual clock (`v_clock`) by converting the elapsed real time
    /// since the last update into 65.63-format fixed-point virtual-time units:
    /// v += (delta t << VT_FIXED_SHIFT) / sum w The caller must pass the
    /// current real time (`now_inst`).
    pub fn advance(&mut self, now_inst: Instant, weight: u64) {
        if let Some(prev) = self.last_update {
            let delta_real = now_inst - prev;

            if weight > 0 {
                let delta_vt = ((delta_real.as_nanos()) << VT_FIXED_SHIFT) / weight as u128;
                self.clk = self.clk.saturating_add(delta_vt);
            }
        }
        self.last_update = Some(now_inst);
    }
}
