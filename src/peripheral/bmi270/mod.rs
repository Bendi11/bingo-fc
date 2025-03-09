use embedded_hal::spi::SpiDevice;

mod regs;

/// Driver for the BMI270 IMU on an SPI bus
pub struct Bmi270<D: SpiDevice> {
    spi: D,
}

impl<D: SpiDevice> Bmi270<D> {
    /// Create a new BMI270 driver from an SpiDevice type
    pub fn new(spi: D) -> Self {
        Self {
            spi,
        }
    }
}
