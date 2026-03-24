//! Video/Display Module for Zeon
//! Video and display device drivers

pub mod virtio_gpu;

// Re-export common types
pub use self::virtio_gpu::VirtioGpu;