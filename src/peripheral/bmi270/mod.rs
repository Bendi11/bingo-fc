use arbitrary_int::{u12, u24, u4, Number};
use embedded_hal::spi::{Operation, SpiDevice};
use stm32f4xx_hal::rcc::Clocks;

pub mod regs;

/// Driver for the BMI270 IMU on an SPI bus
pub struct Bmi270<D: SpiDevice> {
    spi: D,
}

impl<D: SpiDevice> Bmi270<D> {
    const BMI270_UCODE: &[u8] = include_bytes!("./ucode.bin");

    /// Create a new BMI270 driver from an SpiDevice type
    pub fn new(spi: D) -> Self {
        Self {
            spi,
        }
    }
    
    /// Get the sensor time from the sensor
    pub fn sensor_time(&mut self) -> Result<u24, D::Error> {
        let st0 = self.read::<regs::SensorTime0>()?.raw_value();
        let st1 = self.read::<regs::SensorTime1>()?.raw_value();
        let st2 = self.read::<regs::SensorTime2>()?.raw_value();

        Ok(u24::from_le_bytes([st0, st1, st2]))
    }
    
    /// Initialize the IMU configuration file and power settings
    pub fn init(&mut self, clk: &Clocks, delay: &mut impl embedded_hal::delay::DelayNs) -> Result<regs::InternalStatusMessage, D::Error> {
        //Issue unused read to take the BMI270 out of I2C mode if it has not been already
        let _ = self.read::<regs::ChipId>()?;

        'outer: loop {
            self.write(regs::Cmd::DEFAULT.with_field(regs::CmdField::SoftReset))?;

            let pwrconf = self.read::<regs::PwrConf>()?;
            self.write(pwrconf.with_adv_power_save(false))?;

            delay.delay_ms(1); 

            self.set_init_addr(u12::new(0))?;
            self.write(regs::InitCtrl::DEFAULT.with_init_ctrl(false))?;

            self.burst_write::<regs::InitData>(Self::BMI270_UCODE)?;

            self.write(regs::InitCtrl::DEFAULT.with_init_ctrl(true))?;
            
            let mut cycles = 0;
            loop {
                cycles += 1;
                let status = self.read::<regs::InternalStatus>()?;
                match status.message() {
                    regs::InternalStatusMessage::NotInit if cycles > 10_000 => continue 'outer,
                    regs::InternalStatusMessage::NotInit => (),
                    stat => break 'outer Ok(stat)
                }
            }
        }
    }
    
    /// Read the value from the given register
    pub fn read<T: super::Register>(&mut self) -> Result<T, D::Error> {
        let mut buf = [0xff, 0xff];
        self.spi.transaction(&mut [
            Operation::Write(&[T::ADDRESS as u8 | 0b10000000]),
            Operation::Read(&mut buf)
        ])?;

        Ok(
            T::from(buf[1])
        )
    }
    
    /// Burst write `buf` to the register address given
    pub fn burst_write<T: super::Register>(&mut self, buf: &[u8]) -> Result<(), D::Error> {
        for oct in buf.iter().copied() {
            self.write::<T>(T::from(oct))?;
        }

        Ok(())
    }
    
    /// Write the register bitfield into the given address
    pub fn write<T: super::Register>(&mut self, v: T) -> Result<(), D::Error> {
        self.spi.write(&[T::ADDRESS as u8, v.into()])
    }


    
    /// Set the address used for initializing config file
    fn set_init_addr(&mut self, addr: u12) -> Result<(), D::Error> {
        let bits_0_3 = u4::masked_new(addr);
        let bits_4_11 = (addr.as_u16() >> 4) as u8;
        self.write(regs::InitAddr0::DEFAULT.with_base_0_3(bits_0_3))?;
        self.write(regs::InitAddr1::DEFAULT.with_base_11_4(bits_4_11))
    }
}
