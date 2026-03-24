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
use alloc::sync::{Arc, Weak};
use core::{
    fmt::{self, Write},
    ptr::addr_of_mut,
    str,
};
use libkernel::{driver::CharDevDescriptor, error::KernelError};
use log::{LevelFilter, Log};
use tty::TtyInputHandler;

use crate::{drivers::timer::uptime, sync::SpinLock};

mod buf;
pub mod tty;
use buf::BufConsole;
pub mod chardev;

/// Trait for a console device.
pub trait Console: Send + Sync {
    fn write_char(&self, c: char);
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    fn write_buf(&self, buf: &[u8]);

    /// Registers a handler that will receive input bytes.
    fn register_input_handler(&self, handler: Weak<dyn TtyInputHandler>);
}

static mut EARLY_BOOT_BUFFER: BufConsole = BufConsole::new();

/// Current console state.
enum ConsoleState {
    /// Early boot, messages are written to a temporary memory buffer.
    Buffered,
    /// A real console driver has been initialized.
    Device(Arc<dyn Console>, CharDevDescriptor),
}

static CONSOLE: SpinLock<ConsoleState> = SpinLock::new(ConsoleState::Buffered);

/// Writes formatted output to the active console.
pub fn write_fmt(args: fmt::Arguments) -> fmt::Result {
    let console_state = CONSOLE.lock_save_irq();

    match *console_state {
        ConsoleState::Buffered => {
            // SAFETY: The lock on CONSOLE_STATE ensures that no other thread
            // can be reading or writing to the buffer at the same time.
            unsafe { (*addr_of_mut!(EARLY_BOOT_BUFFER)).write_fmt(args) }
        }
        ConsoleState::Device(ref console, _) => console.write_fmt(args),
    }
}

/// Switches the active console from buffer to a real device and flushes output.
pub fn set_active_console(
    console: Arc<dyn Console>,
    char_dev: CharDevDescriptor,
) -> Result<(), KernelError> {
    let mut console_state = CONSOLE.lock_save_irq();

    let old_state = core::mem::replace(
        &mut *console_state,
        ConsoleState::Device(console.clone(), char_dev),
    );

    // If the old state was the buffer, flush its contents to the new device.
    if let ConsoleState::Buffered = old_state {
        // SAFETY: We still hold the lock, and since we just transitioned the
        // state away from `Buffered`, we have exclusive, one-time access to
        // read the buffer's contents. No new writers can appear.
        let buf_contents = unsafe { (*addr_of_mut!(EARLY_BOOT_BUFFER)).data() };

        if let Ok(s) = str::from_utf8(buf_contents) {
            let _ = console.write_fmt(format_args!("{s}"));
        }
    }

    Ok(())
}

struct ConsoleLogger;
static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

impl Log for ConsoleLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let uptime = uptime();
        let _ = write_fmt(format_args!(
            "[{:5}.{:06}] {}: {}\r\n",
            uptime.as_secs(),
            uptime.as_micros(),
            record
                .module_path()
                .map(|x| x.strip_prefix("Zeon::").unwrap_or(x))
                .unwrap_or(""),
            *record.args()
        ));
    }

    fn flush(&self) {}
}

pub fn setup_console_logger() {
    let _ = log::set_logger(&CONSOLE_LOGGER);
    log::set_max_level(LevelFilter::Trace);
}
