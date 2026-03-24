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
use libkernel::memory::address::TUA;

use super::{SigId, SigSet, sigaction::SigActionFlags};

#[derive(Clone, Copy, Debug)]
pub struct UserspaceSigAction {
    pub action: TUA<extern "C" fn(i32)>,
    pub restorer: Option<TUA<extern "C" fn()>>,
    pub flags: SigActionFlags,
    pub mask: SigSet,
}

#[derive(Clone, Copy, Debug)]
/// How the kernel should respond to a signal.
pub enum KSignalAction {
    Term,
    Core,
    Stop,
    Continue,
    Userspace(SigId, UserspaceSigAction),
}

impl KSignalAction {
    /// Returns the default action for a given signal.
    ///
    /// For signals whose default is to be ignored, `None` is returned.
    pub const fn default_action(signal: SigId) -> Option<Self> {
        match signal {
            SigId::SIGABRT => Some(Self::Core),
            SigId::SIGALRM => Some(Self::Term),
            SigId::SIGBUS => Some(Self::Core),
            SigId::SIGCHLD => None,
            SigId::SIGCONT => Some(Self::Continue),
            SigId::SIGFPE => Some(Self::Core),
            SigId::SIGHUP => Some(Self::Term),
            SigId::SIGILL => Some(Self::Core),
            SigId::SIGINT => Some(Self::Term),
            SigId::SIGIO => Some(Self::Term),
            SigId::SIGKILL => Some(Self::Term),
            SigId::SIGPIPE => Some(Self::Term),
            SigId::SIGPROF => Some(Self::Term),
            SigId::SIGPWR => Some(Self::Term),
            SigId::SIGQUIT => Some(Self::Core),
            SigId::SIGSEGV => Some(Self::Core),
            SigId::SIGSTKFLT => Some(Self::Term),
            SigId::SIGSTOP => Some(Self::Stop),
            SigId::SIGTSTP => Some(Self::Stop),
            SigId::SIGTERM => Some(Self::Term),
            SigId::SIGTRAP => Some(Self::Core),
            SigId::SIGTTIN => Some(Self::Stop),
            SigId::SIGTTOU => Some(Self::Stop),
            SigId::SIGUNUSED => Some(Self::Core),
            SigId::SIGURG => None,
            SigId::SIGUSR1 => Some(Self::Term),
            SigId::SIGUSR2 => Some(Self::Term),
            SigId::SIGVTALRM => Some(Self::Term),
            SigId::SIGXCPU => Some(Self::Core),
            SigId::SIGXFSZ => Some(Self::Core),
            SigId::SIGWINCH => None,
        }
    }
}
