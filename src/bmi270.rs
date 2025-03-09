use embedded_hal::spi::SpiDevice;

pub struct Bmi270<D: SpiDevice> {
    spi: D,
}

impl<D: SpiDevice> Bmi270<D> {
    pub fn new(spi: D) -> Self {
        Self {
            spi,
        }
    }
}
