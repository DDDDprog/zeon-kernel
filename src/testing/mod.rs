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
use crate::console::write_fmt;
use crate::drivers::timer::uptime;
use alloc::format;
use core::fmt::Display;

const TEXT_GREEN: &str = "\x1b[32m";
const TEXT_RED: &str = "\x1b[31m";
const TEXT_RESET: &str = "\x1b[0m";

pub enum TestResult {
    Ok,
    Failed,
    Skipped,
}

impl Display for TestResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TestResult::Ok => write!(f, "{TEXT_GREEN}ok{TEXT_RESET}"),
            TestResult::Failed => write!(f, "{TEXT_GREEN}failed{TEXT_RESET}"),
            TestResult::Skipped => write!(f, "skipped"),
        }
    }
}

pub struct Test {
    pub name: &'static str,
    pub test_fn: fn() -> TestResult,
}

#[cfg(test)]
pub fn test_runner(tests: &[&Test]) {
    write_fmt(format_args!("\nrunning {} tests\n", tests.len())).unwrap();
    let mut passed = 0;
    let mut failed = 0;
    let mut ignored = 0;
    let start = uptime();
    for test in tests {
        let result = (test.test_fn)();
        match result {
            TestResult::Ok => passed += 1,
            TestResult::Failed => failed += 1,
            TestResult::Skipped => ignored += 1,
        }
        write_fmt(format_args!("test {} ... {}\n", test.name, result)).unwrap();
    }
    let duration = uptime() - start;
    write_fmt(format_args!(
        "\ntest result: {}. {passed} passed; {failed} failed; {ignored} ignored; finished in {}.{}s\n",
        if failed == 0 {
            format!("{TEXT_GREEN}ok{TEXT_RESET}")
        } else {
            format!("{TEXT_RED}FAILED{TEXT_RESET}")
        },
        duration.as_secs(),
        duration.subsec_millis() / 10
    ))
    .unwrap();
    ArchImpl::power_off();
}

pub fn panic_noop(_: *mut u8, _: *mut u8) {}

#[macro_export]
macro_rules! ktest_impl {
    ($name:ident, fn $fn_name:ident() $body:block) => {
        #[cfg(test)]
        fn $fn_name(_: *mut u8) {
            $body
        }

        paste::paste! {
            #[cfg(test)]
            #[test_case]
            static [<__TEST_ $name>]: crate::testing::Test = crate::testing::Test {
                name: concat!(module_path!(), "::", stringify!($name)),
                test_fn: || {
                    let result = unsafe {
                        core::intrinsics::catch_unwind(
                            $fn_name as fn(*mut u8),
                            core::ptr::null_mut(),
                            crate::testing::panic_noop,
                        )
                    };
                    match result {
                        0 => crate::testing::TestResult::Ok,
                        1 => crate::testing::TestResult::Failed,
                        _ => unreachable!("catch_unwind should only return 0 or 1"),
                    }
                },
            };
        }
    };
    (fn $name:ident() $body:block) => {
        crate::ktest_impl!($name, fn $name() $body);
    };
    (async fn $name:ident() $body:block) => {
        async fn $name() {
            $body
        }

        paste::paste! {
            crate::ktest_impl! {
                $name,
                fn [<__sync_ $name>]() {
                    let mut fut = alloc::boxed::Box::pin($name());
                    let waker = crate::sched::current_work_waker();
                    let mut ctx = core::task::Context::from_waker(&waker);
                    loop {
                        match fut.as_mut().poll(&mut ctx) {
                            core::task::Poll::Ready(()) => break,
                            _ => {},
                        }
                    }
                }
            }
        }

    }
}
