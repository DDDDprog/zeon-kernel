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
//! GPU Driver for Zeon
//! Graphics Processing Unit driver interface

use crate::drivers::Driver;
use crate::error::KernelResult;

/// GPU Device
pub struct GpuDevice {
    pub name: String,
    pub base_addr: usize,
    pub memory_size: usize,
    initialized: bool,
}

impl GpuDevice {
    /// Create new GPU device
    pub fn new(name: &str, base_addr: usize, memory_size: usize) -> Self {
        Self {
            name: name.to_string(),
            base_addr,
            memory_size,
            initialized: false,
        }
    }

    /// Initialize GPU
    pub fn init(&mut self) -> KernelResult {
        if self.initialized {
            return Ok(());
        }
        // Initialize GPU memory and configuration
        self.initialized = true;
        Ok(())
    }

    /// Allocate GPU buffer
    pub fn allocate_buffer(&self, size: usize) -> KernelResult<usize> {
        if !self.initialized {
            return Err(crate::error::KernelError::NotInitialized);
        }
        // Allocate GPU memory
        Ok(0x1000)
    }

    /// Submit GPU command
    pub fn submit_command(&self, cmd: &[u32]) -> KernelResult {
        if !self.initialized {
            return Err(crate::error::KernelError::NotInitialized);
        }
        Ok(())
    }

    /// Get display surface
    pub fn get_surface(&self, width: u32, height: u32) -> KernelResult<GpuSurface> {
        if !self.initialized {
            return Err(crate::error::KernelError::NotInitialized);
        }
        Ok(GpuSurface { width, height, format: PixelFormat::Rgba8888 })
    }
}

/// GPU Surface/Framebuffer
#[derive(Debug, Clone, Copy)]
pub struct GpuSurface {
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    Rgba8888,
    Rgb888,
    Bgra8888,
    Rgb565,
}

impl Driver for GpuDevice {
    fn name(&self) -> &str {
        "gpu"
    }

    fn init(&mut self) -> KernelResult {
        self.init()
    }
}