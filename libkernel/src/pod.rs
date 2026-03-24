   _/ ___| 
 * | |_) | |_) |  / _ \ \___ \ | | \___ \ 
 * |  __/|  _ <  / ___ \ ___) || |  ___) |
 * |_|| \_\/_/   \____/ |_| |____/ 
 *
 * Zeon
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
/// An unsafe trait indicating that a type is "Plain Old Data".
///
/// A type is `Pod` if it is a simple collection of bytes with no invalid bit
/// patterns. This means it can be safely created by simply copying its byte
/// representation from memory or a device.
///
/// # Safety
///
/// The implementor of this trait MUST guarantee that:
/// 1. The type has a fixed, known layout. Using `#[repr(C)]` or
///    `#[repr(transparent)]` is a must! The Rust ABI is unstable.
/// 2. The type contains no padding bytes, or if it does, that reading those
///    padding bytes as uninitialized memory is not undefined behavior.
/// 3. All possible bit patterns for the type's size are valid instances of the type.
///    For example, a `bool` is NOT `Pod` because its valid representations are only
///    0x00 and 0x01, not any other byte value. A `u32` is `Pod` because all
///    2^32 bit patterns are valid `u32` values.
pub unsafe trait Pod: Sized {}

// Blanket implementations for primitive types that are definitely Pod.
unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for u128 {}
unsafe impl Pod for i8 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for i64 {}
unsafe impl Pod for i128 {}
unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}
