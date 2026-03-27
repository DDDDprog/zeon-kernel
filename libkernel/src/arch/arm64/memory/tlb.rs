/*
 * Zeon - Pure Rust Operating System
 * https://github.com/DDDDprog/zeon-kernel
 */

pub trait TLBInvalidator {}

pub struct NullTlbInvalidator {}

impl TLBInvalidator for NullTlbInvalidator {}
