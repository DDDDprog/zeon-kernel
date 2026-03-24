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
use alloc::boxed::Box;
use core::{pin::Pin, ptr};
use libkernel::error::Result;

pub type SignalWork = Pin<Box<dyn Future<Output = Result<UserCtx>>>>;
pub type KernelWork = Pin<Box<dyn Future<Output = ()>>>;
pub type UserCtx = <ArchImpl as Arch>::UserContext;

pub struct Context {
    signal: Option<SignalWork>,
    kernel: Option<KernelWork>,
    user: UserCtx,
}

impl Context {
    pub fn from_user_ctx(user_ctx: UserCtx) -> Self {
        Self {
            signal: None,
            kernel: None,
            user: user_ctx,
        }
    }

    pub fn user(&self) -> &UserCtx {
        &self.user
    }

    pub fn user_mut(&mut self) -> &mut UserCtx {
        &mut self.user
    }

    pub fn save_user_ctx(&mut self, ctx: *const UserCtx) {
        unsafe { ptr::copy_nonoverlapping(ctx, ptr::from_mut(&mut self.user), 1) };
    }

    pub fn restore_user_ctx(&self, ctx: *mut UserCtx) {
        unsafe {
            ptr::copy_nonoverlapping(&self.user as _, ctx, 1);
        }
    }

    pub fn put_signal_work(&mut self, work: SignalWork) {
        // We should never double-schedule signal work.
        debug_assert!(self.signal.is_none());

        self.signal = Some(work);
    }

    pub fn take_signal_work(&mut self) -> Option<SignalWork> {
        self.signal.take()
    }

    pub fn put_kernel_work(&mut self, work: KernelWork) {
        // We should never double-schedule kernel work.
        debug_assert!(self.kernel.is_none());

        self.kernel = Some(work);
    }

    pub fn take_kernel_work(&mut self) -> Option<KernelWork> {
        self.kernel.take()
    }
}
