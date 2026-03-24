//! Network Device Module for Zeon
//! Network device drivers (virtio-net, etc.)

pub mod virtio;

// Re-export common types
pub use self::virtio::{VirtioNet, NetworkStats};