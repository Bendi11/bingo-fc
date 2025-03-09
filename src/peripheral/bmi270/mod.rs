use embedded_hal::spi::SpiDevice;

pub mod regs;

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

    pub fn read_reg<T: super::Register>(&mut self) -> Result<T, D::Error> {
        let mut buf = [T::ADDRESS as u8, 0u8];
        self.spi.transfer_in_place(&mut buf)?;

        Ok(
            T::from(buf[1])
        )
    }
}
