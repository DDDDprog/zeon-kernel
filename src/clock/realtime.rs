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
    drivers::timer::{Instant, now, uptime},
    sync::SpinLock,
};
use core::time::Duration;

// Return a duration from the epoch.
pub fn date() -> Duration {
    let epoch_info = *EPOCH_DURATION.lock_save_irq();

    if let Some(ep_info) = epoch_info
        && let Some(now) = now()
    {
        let duraton_since_ep_info = now - ep_info.1;
        ep_info.0 + duraton_since_ep_info
    } else {
        uptime()
    }
}

pub fn set_date(duration: Duration) {
    if let Some(now) = now() {
        let mut epoch_info = EPOCH_DURATION.lock_save_irq();
        *epoch_info = Some((duration, now));
    }
}

// Represents a known duration since the epoch at the associated instant.
static EPOCH_DURATION: SpinLock<Option<(Duration, Instant)>> = SpinLock::new(None);

#[cfg(test)]
mod tests {
    use super::*;
    use Zeon_macros::ktest;

    #[ktest]
    fn test_date_and_set_date() {
        let initial_date = date();
        let new_date = Duration::from_secs(1_000_000);
        set_date(new_date);
        let updated_date = date();
        assert_ne!(
            initial_date, updated_date,
            "Date should change after set_date"
        );
        assert!(
            updated_date >= new_date,
            "Updated date should be at least the new date set"
        );
    }
}
