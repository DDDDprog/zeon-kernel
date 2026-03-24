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
use crate::drivers::virtio_hal::VirtioHal;
use crate::sync::SpinLock;
use crate::{
    arch::ArchImpl,
    drivers::{
        Driver, DriverManager,
        init::PlatformBus,
        probe::{DeviceDescriptor, DeviceMatchType},
    },
    kernel::rand::{EntropySource, register_entropy_source},
    kernel_driver,
};
use alloc::{boxed::Box, sync::Arc};
use core::ptr::NonNull;
use libkernel::{
    KernAddressSpace, VirtualMemory,
    error::{KernelError, ProbeError, Result},
    memory::{
        address::{PA, VA},
        region::PhysMemoryRegion,
    },
};
use log::info;
use virtio_drivers::{
    device::rng::VirtIORng,
    transport::{
        DeviceType, Transport,
        mmio::{MmioTransport, VirtIOHeader},
    },
};

pub struct VirtioRngDriver {
    rng: SpinLock<VirtIORng<VirtioHal, MmioTransport<'static>>>,
}

impl Driver for VirtioRngDriver {
    fn name(&self) -> &'static str {
        "virtio-rng"
    }
}

impl EntropySource for VirtioRngDriver {
    fn get_entropy(&self, buf: &mut [u8]) -> (usize, usize) {
        let mut rng = self.rng.lock_save_irq();
        match rng.request_entropy(buf) {
            Ok(n) => (n, n * 8),
            Err(_) => (0, 0),
        }
    }
}

fn virtio_rng_probe(_dm: &mut DriverManager, d: DeviceDescriptor) -> Result<Arc<dyn Driver>> {
    match d {
        DeviceDescriptor::Fdt(fdt_node, _flags) => {
            let region = fdt_node
                .reg()
                .ok_or(ProbeError::NoReg)?
                .next()
                .ok_or(ProbeError::NoReg)?;

            let size = region.size.ok_or(ProbeError::NoRegSize)?;

            let mapped: VA =
                ArchImpl::kern_address_space()
                    .lock_save_irq()
                    .map_mmio(PhysMemoryRegion::new(
                        PA::from_value(region.address as usize),
                        size,
                    ))?;

            let header = NonNull::new(mapped.value() as *mut VirtIOHeader)
                .ok_or(KernelError::InvalidValue)?;

            let transport = unsafe {
                match MmioTransport::new(header, size) {
                    Ok(t) => t,
                    Err(_) => return Err(KernelError::Probe(ProbeError::NoMatch)),
                }
            };

            if !matches!(transport.device_type(), DeviceType::EntropySource) {
                return Err(KernelError::Probe(ProbeError::NoMatch));
            }

            info!("virtio-rng found (node {})", fdt_node.name);

            let rng = VirtIORng::<VirtioHal, _>::new(transport)
                .map_err(|_| KernelError::Other("virtio-rng init failed"))?;

            let driver = Arc::new(VirtioRngDriver {
                rng: SpinLock::new(rng),
            });

            register_entropy_source(driver.clone());

            Ok(driver)
        }
    }
}

fn virtio_rng_init(bus: &mut PlatformBus, _dm: &mut DriverManager) -> Result<()> {
    bus.register_platform_driver(
        DeviceMatchType::FdtCompatible("virtio,mmio"),
        Box::new(virtio_rng_probe),
    );

    bus.register_platform_driver(
        DeviceMatchType::FdtCompatible("virtio-mmio"),
        Box::new(virtio_rng_probe),
    );

    Ok(())
}

kernel_driver!(virtio_rng_init);
