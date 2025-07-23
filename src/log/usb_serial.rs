use usb_device::bus::{UsbBus, UsbBusAllocator};



pub struct UsbSerialLogger<'buf, B: UsbBus> {
    usb: usbd_serial::SerialPort<'buf, B>,
}

impl<'buf, B: UsbBus> UsbSerialLogger<'buf, B> {
    /// Create a new USB serial logger from the given bus allocator
    pub fn new(allocator: &'buf UsbBusAllocator<B>) -> Self {
        Self {
            usb: usbd_serial::SerialPort::new(allocator)
        }
    }
}
