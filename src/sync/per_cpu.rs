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
/// A declarative macro to define a static `PerCpu` variable and register it
/// for automatic initialization.
#[macro_export]
macro_rules! per_cpu_shared {
    ($vis:vis static $name:ident: $type:ty = $initializer:expr;) => {
        $vis static $name: libkernel::sync::per_cpu::PerCpu<$type, $crate::arch::ArchImpl> =
            libkernel::sync::per_cpu::PerCpu::new($initializer);

        paste::paste! {
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".percpu")]
        #[used(linker)]
        static [<$name _PERCPU_INITIALIZER>]: &'static (
                     dyn libkernel::sync::per_cpu::PerCpuInitializer + Sync
                 ) = &$name;
        }
    };
}

/// Wraps with a [`RefCell`] for convenience
#[macro_export]
macro_rules! per_cpu_private {
    ($vis:vis static $name:ident: $type:ty = $initializer:expr;) => {
        $vis static $name: libkernel::sync::per_cpu::PerCpu<
            core::cell::RefCell<$type>,
            $crate::arch::ArchImpl,
        > = libkernel::sync::per_cpu::PerCpu::new(|| {core::cell::RefCell::new($initializer())});

        paste::paste! {
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".percpu")]
        #[used(linker)]
        static [<$name _PERCPU_INITIALIZER>]: &'static (
                     dyn libkernel::sync::per_cpu::PerCpuInitializer + Sync
                 ) = &$name;
        }
    };
}
