#![no_std]
#![no_main]

use stm32f4xx_hal::{pac, gpio::GpioExt};

extern crate panic_halt;

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    loop {
        core::hint::spin_loop();
    }
}
