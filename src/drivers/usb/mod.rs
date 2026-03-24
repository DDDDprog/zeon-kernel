// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

/// USB Host Controller
pub struct UsbController {
    pub base_addr: usize,
    pub version: UsbVersion,
    initialized: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum UsbVersion {
    Usb1_1,
    Usb2_0,
    Usb3_0,
    Usb3_1,
}

impl UsbController {
    /// Create new USB controller
    pub fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
            version: UsbVersion::Usb2_0,
            initialized: false,
        }
    }

    /// Initialize USB controller
    pub fn init(&mut self) -> KernelResult {
        if self.initialized {
            return Ok(());
        }
        // Reset USB host controller
        // Configure ports and enable interrupts
        self.initialized = true;
        Ok(())
    }

    /// Get connected devices
    pub fn enumerate(&self) -> KernelResult<Vec<UsbDevice>> {
        if !self.initialized {
            return Err(crate::error::KernelError::NotInitialized);
        }
        // Enumerate USB devices
        Ok(vec![])
    }

    /// Submit USB transfer
    pub fn submit_transfer(&self, transfer: &UsbTransfer) -> KernelResult {
        if !self.initialized {
            return Err(crate::error::KernelError::NotInitialized);
        }
        Ok(())
    }
}

/// USB Device
#[derive(Debug)]
pub struct UsbDevice {
    pub vendor_id: u16,
    pub product_id: u16,
    pub class: u8,
    pub speed: UsbSpeed,
}

#[derive(Debug, Clone, Copy)]
pub enum UsbSpeed {
    Low,
    Full,
    High,
    Super,
}

/// USB Transfer
#[derive(Debug)]
pub struct UsbTransfer {
    pub device_addr: u8,
    pub endpoint: u8,
    pub data: Vec<u8>,
    pub transfer_type: TransferType,
}

#[derive(Debug, Clone, Copy)]
pub enum TransferType {
    Control,
    Interrupt,
    Bulk,
    Isochronous,
}

impl Driver for UsbController {
    fn name(&self) -> &str {
        "usb"
    }

    fn init(&mut self) -> KernelResult {
        self.init()
    }
}