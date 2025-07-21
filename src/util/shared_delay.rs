use core::{borrow::Borrow, cell::RefCell};

use cortex_m::interrupt::Mutex;
use embedded_hal::delay::DelayNs;

pub struct CortexSharedDelayFactory<D>(Mutex<RefCell<D>>);

#[derive(Clone,)]
pub struct CortexSharedDelay<'a, D>(&'a Mutex<RefCell<D>>);

impl<D> CortexSharedDelayFactory<D> {
    pub const fn new(delay: D) -> Self {
        Self(Mutex::new(RefCell::new(delay)))
    }

    pub const fn borrow(&self) -> CortexSharedDelay<D> {
        CortexSharedDelay(&self.0)
    }
}


impl<'a, D: DelayNs> DelayNs for CortexSharedDelay<'a, D> {
    fn delay_ns(&mut self, ns: u32) {
        cortex_m::interrupt::free(|cs| self.0.borrow(cs).borrow_mut().delay_ns(ns))
    }
    fn delay_us(&mut self, us: u32) {
        cortex_m::interrupt::free(|cs| self.0.borrow(cs).borrow_mut().delay_us(us))
    }
    fn delay_ms(&mut self, ms: u32) {
        cortex_m::interrupt::free(|cs| self.0.borrow(cs).borrow_mut().delay_ms(ms))
    }
}
