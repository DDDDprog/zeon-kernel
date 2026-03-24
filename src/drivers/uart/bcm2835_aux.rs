// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

impl Bcm2835AuxRegBank {
    pub fn new(addr: VA) -> &'static mut Self {
        unsafe { &mut *(addr.as_ptr_mut() as *mut Self) }
    }

    pub fn init(&mut self) {
        self.ier.set(1);
    }

    pub fn put_char(&mut self, c: char) {
        if c == '\n' {
            self.send_byte(b'\r');
            self.send_byte(b'\n');
        } else {
            let mut buf = [0u8; 4];
            for b in c.encode_utf8(&mut buf).bytes() {
                self.send_byte(b);
            }
        }
    }

    fn send_byte(&mut self, b: u8) {
        loop {
            match self.lsr.read_as_enum(AUX_MU_LSR_REG::TransmitterEmpty) {
                Some(AUX_MU_LSR_REG::TransmitterEmpty::Value::CanAccept) => break,
                _ => spin_loop(),
            }
        }
        self.io.write(AUX_MU_IO_REG::DATA.val(b as u32));
    }
}

impl fmt::Write for Bcm2835AuxRegBank {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for char in s.chars() {
            self.put_char(char);
        }

        Ok(())
    }
}

pub struct Bcm2835AuxUart {
    regs: SpinLock<&'static mut Bcm2835AuxRegBank>,
    fdt_name: Option<&'static str>,
    _interrupt: ClaimedInterrupt,
    tty_handler: SpinLock<Option<Weak<dyn TtyInputHandler>>>,
}

impl Bcm2835AuxUart {
    pub fn new(addr: VA, fdt_name: &'static str, interrupt: ClaimedInterrupt) -> Self {
        let regs = Bcm2835AuxRegBank::new(addr);

        regs.init();

        Self {
            regs: SpinLock::new(regs),
            fdt_name: Some(fdt_name),
            _interrupt: interrupt,
            tty_handler: SpinLock::new(None),
        }
    }
}

impl Console for Bcm2835AuxUart {
    fn write_fmt(&self, args: core::fmt::Arguments) -> core::fmt::Result {
        let mut regs = self.regs.lock_save_irq();
        regs.write_fmt(args)
    }

    fn write_buf(&self, buf: &[u8]) {
        let mut regs = self.regs.lock_save_irq();
        buf.iter().for_each(|b| regs.send_byte(*b));
    }

    fn register_input_handler(&self, handler: Weak<dyn TtyInputHandler>) {
        *self.tty_handler.lock_save_irq() = Some(handler);
    }
}

impl Driver for Bcm2835AuxUart {
    fn name(&self) -> &'static str {
        self.fdt_name.unwrap_or("BCM2835 Aux Mini UART")
    }

    fn as_console(self: Arc<Self>) -> Option<Arc<dyn Console>> {
        Some(self)
    }
}

impl InterruptHandler for Bcm2835AuxUart {
    fn handle_irq(&self, _desc: crate::interrupts::InterruptDescriptor) {
        let regs = self.regs.lock_save_irq();
        regs.iir.get();
        let data = regs.io.read(AUX_MU_IO_REG::DATA) as u8;

        if let Some(ref handler) = *self.tty_handler.lock_save_irq()
            && let Some(handler) = handler.upgrade()
        {
            handler.push_byte(data);
        }
    }
}

pub fn bcm2835_aux_probe(
    mut dm: &mut DriverManager,
    d: DeviceDescriptor,
) -> Result<Arc<dyn Driver>> {
    match d {
        DeviceDescriptor::Fdt(fdt_node) => {
            use libkernel::error::ProbeError::*;

            let mut regs = fdt_node.reg().ok_or(NoReg)?;
            let region = regs.next().ok_or(NoReg)?;
            let size = region.size.ok_or(NoRegSize)?;
            let mem =
                ArchImpl::kern_address_space()
                    .lock_save_irq()
                    .map_mmio(PhysMemoryRegion::new(
                        PA::from_value(region.address as usize),
                        size,
                    ))?;

            let mut interrupts = fdt_node
                .interrupts()
                .ok_or(NoInterrupts)?
                .next()
                .ok_or(NoInterrupts)?;

            let interrupt_node = fdt_node.interrupt_parent().ok_or(NoParentIntterupt)?.node;

            let interrupt_manager = fdt_prober::get_or_init_dev(&interrupt_node, &mut dm)?
                .ok_or(NoParentIntterupt)?
                .as_interrupt_manager()
                .ok_or(NotInterruptController)?;

            let interrupt_config = interrupt_manager.parse_fdt_interrupt_regs(&mut interrupts)?;

            info!("BCM Aux UART: Claiming interrupt: {:?}", interrupt_config);
            info!("BCM Regs mapped at: {:?}", mem);

            Ok(
                interrupt_manager.claim_interrupt(interrupt_config, |claimed_interrupt| {
                    Bcm2835AuxUart::new(mem, fdt_node.name, claimed_interrupt)
                })?,
            )
        }
    }
}

register_driver!(
    BCM2835AUX,
    DeviceMatchType::FdtCompatible("brcm,bcm2835-aux-uart"),
    bcm2835_aux_probe
);
