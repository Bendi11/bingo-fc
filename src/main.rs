#![no_std]
#![no_main]

use bmi270::Bmi270;
use cortex_m::Peripherals;
use embedded_hal::spi::{Mode, Phase, Polarity};
use embedded_hal_bus::spi::ExclusiveDevice;
use stm32f4xx_hal::{gpio::{GpioExt, PinSpeed, Speed}, otg_fs::USB, pac::{self, NVIC, SPI1}, prelude::*, rcc::RccExt, timer::SysDelay};
use synopsys_usb_otg::UsbBus;
use usb_device::{device::{StringDescriptors, UsbDeviceBuilder}, LangID, UsbError};
use usbd_serial::{embedded_io::Write, SerialPort, USB_CLASS_CDC};

use panic_halt as _;

mod peripheral;

use peripheral::bmi270;

#[cortex_m_rt::entry]
fn main() -> ! {
    static mut USB_EP_BUF: &mut [u32] = &mut [0 ; 1024];

    let peripherals = pac::Peripherals::take().unwrap();
    let core_peripherals = pac::CorePeripherals::take().unwrap();
    peripherals.RCC.ahb1enr().write(|w| w.gpiocen().enabled().gpioaen().enabled());
    
    let gpioc = peripherals.GPIOC.split();
    let mut pc8 = gpioc.pc8.into_push_pull_output();
    pc8.set_high();

    let rcc = peripherals.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .require_pll48clk()
        .freeze();

    let mut gpioa = peripherals.GPIOA.split();

    let mut spi1_cs = gpioa.pa4.into_push_pull_output_in_state(stm32f4xx_hal::gpio::PinState::High);

    let usb_dm = gpioa.pa11;
    let usb_dp = gpioa.pa12;

    let usb = stm32f4xx_hal::otg_fs::USB::new(
        (peripherals.OTG_FS_GLOBAL, peripherals.OTG_FS_DEVICE, peripherals.OTG_FS_PWRCLK),
        (usb_dm, usb_dp),
        &clocks,
    );

    let usb_bus = UsbBus::new(usb, USB_EP_BUF);
    
    let mut serial = SerialPort::new(&usb_bus);

    let mut device = match UsbDeviceBuilder::new(&usb_bus, usb_device::device::UsbVidPid(0xbeef, 0x0911))
        .strings(&[
            StringDescriptors::new(LangID::EN)
                .manufacturer("Crosstalk Labs")
                .product("Bingo Flight Controller")
                .serial_number("BFC")
        ])
        .map(|d|
            d.device_class(USB_CLASS_CDC)
            .build()
    ) {
        Ok(d) => d,
        Err(_) => {
            pc8.set_high();
            loop {
                core::hint::spin_loop();
            }
        }
    };

    let mut mosi = gpioa.pa7.into_push_pull_output();
    mosi.set_speed(Speed::VeryHigh);
    
    let mut spi = peripherals.SPI1.spi(
        (gpioa.pa5, gpioa.pa6, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition
        },
        5.MHz(),
        &clocks
    ).init();

    spi.bit_format(stm32f4xx_hal::spi::BitFormat::MsbFirst);

    let mut delay = core_peripherals.SYST.delay(&clocks);

    let spi1 = match ExclusiveDevice::new_no_delay(spi, spi1_cs) {
        Ok(d) => {
            let _ = serial.write_all(b"Created SPI device for BMI270\n");
            d
        },
        Err(_) => {
            let _ = serial.write_all(b"Failed to create SPI device for BMI270\n");
            loop {}
        }
    };

    
    let mut bmi = Bmi270::new(spi1);

    let mut init = false;
    let mut status = None;

    loop {
        if !device.poll(&mut [&mut serial]) {
            continue
        }

        let mut buf = [0u8 ; 256];
        
        match serial.read(&mut buf[..]) {
            Ok(_) => {
                if !init {
                    status = Some(bmi.init(&mut delay));
                    init = true;
                }

                write!(serial, "Chip Status is {:?} - t{}\r\n", status, bmi.sensor_time().unwrap());
                pc8.set_high();
            },
            Err(UsbError::WouldBlock) => continue,
            Err(_) => {
                let _ = serial.write_all(b"Failed to read serial from USB device");
            }
        }
    }
}
