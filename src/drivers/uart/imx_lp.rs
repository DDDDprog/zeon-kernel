/*

struct Imx8UlpLp {
    regs: &'static mut LpuartRegBank,
}

unsafe impl Send for Imx8UlpLp {}
unsafe impl Sync for Imx8UlpLp {}

impl Imx8UlpLp {
    pub fn new(addr: VA) -> Self {
        let regs = unsafe { &mut *(addr.as_ptr_mut() as *mut LpuartRegBank) };

        // Enable transmitter, receiver, and receive interrupts.
        regs.ctrl
            .modify(CTRL::TE::Enable + CTRL::RE::Enable + CTRL::RIE::Enable);

        Self { regs }
    }
}

impl UartDriver for Imx8UlpLp {
    fn write_buf(&mut self, buf: &[u8]) {
        for c in buf {
            // Wait until the transmit data register is empty.
            while !self.regs.stat.is_set(STAT::TDRE) {
                spin_loop();
            }

            // Write the byte to the data register.
            self.regs.data.write(DATA::DATA.val(*c as u32));
        }
    }

    fn drain_uart_rx(&mut self, buf: &mut [u8]) -> usize {
        let mut bytes_read = 0;

        while self.regs.stat.is_set(STAT::RDRF) && bytes_read < buf.len() {
            // Reading the data register clears the RDRF flag.
            let byte = self.regs.data.read(DATA::DATA) as u8;

            buf[bytes_read] = byte;
            bytes_read += 1;
        }

        bytes_read
    }
}

impl core::fmt::Write for Imx8UlpLp {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_buf(s.as_bytes());

        Ok(())
    }
}

pub fn imx8ulp_lpuart_probe(
    dm: &mut DriverManager,
    d: DeviceDescriptor,
) -> Result<Arc<dyn Driver>> {
    match d {
        DeviceDescriptor::Fdt(fdt_node, flags) => {
            use libkernel::error::ProbeError::*;

            let mut regs = fdt_node.reg().ok_or(NoReg)?;
            let region = regs.next().ok_or(NoReg)?;
            let size = region.size.ok_or(NoRegSize)?;

            let mut interrupts = fdt_node
                .interrupts()
                .ok_or(NoInterrupts)?
                .next()
                .ok_or(NoInterrupts)?;

            let interrupt_node = fdt_node.interrupt_parent().ok_or(NoParentInterrupt)?.node;

            let interrupt_manager = dm
                .find_by_name(interrupt_node.name)
                .ok_or(Deferred)?
                .as_interrupt_manager()
                .ok_or(NotInterruptController)?;

            let uart_cdev = UART_CHAR_DEV.get().ok_or(Deferred)?;

            let interrupt_config = interrupt_manager.parse_fdt_interrupt_regs(&mut interrupts)?;

            let mem =
                ArchImpl::kern_address_space()
                    .lock_save_irq()
                    .map_mmio(PhysMemoryRegion::new(
                        PA::from_value(region.address as usize),
                        size,
                    ))?;

            let dev = interrupt_manager.claim_interrupt(interrupt_config, |claimed_interrupt| {
                Uart::new(Imx8UlpLp::new(mem), claimed_interrupt, fdt_node.name)
            })?;

            uart_cdev.register_console(dev.clone(), flags.contains(FdtFlags::ACTIVE_CONSOLE))?;

            Ok(dev)
        }
    }
}

pub fn imx8ulp_uart_init(bus: &mut PlatformBus, _dm: &mut DriverManager) -> Result<()> {
    bus.register_platform_driver(
        DeviceMatchType::FdtCompatible("fsl,imx8ulp-lpuart"),
        Box::new(imx8ulp_lpuart_probe),
    );

    Ok(())
}

kernel_driver!(imx8ulp_uart_init);
