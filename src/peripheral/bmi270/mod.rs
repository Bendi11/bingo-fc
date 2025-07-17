use arbitrary_int::{u12, u24, u4, Number};
use embedded_hal::{delay::DelayNs, spi::{Operation, SpiDevice}};
use stm32f4xx_hal::rcc::Clocks;

pub mod regs;

/// Driver for the BMI270 IMU on an SPI bus
pub struct Bmi270<S: SpiDevice, D: DelayNs> {
    spi: S,
    delay: D,
}

impl<S: SpiDevice, D: DelayNs> Bmi270<S, D> {
    const BMI270_MAX_FIFO_UCODE: &[u8 ; 328] = include_bytes!("./ucode/ucode-max-fifo.bin");

    /// Create a new BMI270 driver from an SpiDevice type
    pub fn new(spi: S, delay: D) -> Self {
        Self {
            spi,
            delay,
        }
    }
    
    /// Get the sensor time from the sensor
    pub fn sensor_time(&mut self) -> Result<u24, S::Error> {
        let mut buf = [0u8 ; 4];
        self.spi.transaction(&mut [
            Operation::Write(&[<regs::SensorTime0 as super::Register>::ADDRESS as u8 | 0b10000000]),
            Operation::Read(&mut buf)
        ])?;

        Ok(u24::from_le_bytes([buf[1], buf[2], buf[3]]))
    }

    pub fn data(&mut self) -> Result<((i16, i16, i16), (i16, i16, i16)), S::Error> {
        let mut buf = [0u8 ; 13];
        self.spi.transaction(&mut [
            Operation::Write(&[0x0C | 0b10000000]),
            Operation::Read(&mut buf)
        ])?;

        let decode = |idx| i16::from_le_bytes([buf[idx], buf[idx + 1]]);

        Ok((
            (decode(1), decode(3), decode(5)),
            (decode(7), decode(9), decode(11))
        ))
    } 

    pub fn status(&mut self) -> Result<regs::InternalStatus, S::Error> {
        self.read::<regs::InternalStatus>()
    }
    
    /// Initialize the IMU configuration file and power settings
    pub fn init(&mut self) -> Result<regs::InternalStatusMessage, S::Error> {
        //Issue unused read to take the BMI270 out of I2C mode if it has not been already
        let _ = self.read::<regs::ChipId>()?;
        self.delay.delay_ms(10);

        let id = self.read::<regs::ChipId>()?;
        if id.raw_value() != 0x24 {
            return Ok(regs::InternalStatusMessage::DrvErr)
        }

        self.write(regs::PwrConf::DEFAULT.with_adv_power_save(false))?;

        self.delay.delay_ms(1);


        self.write(regs::InitCtrl::DEFAULT.with_init_ctrl(false))?;

        self.delay.delay_ms(1);

        self.set_init_addr(u12::new(0))?;


        self.delay.delay_ms(1);

        self.burst_write::<regs::InitData>(Self::BMI270_MAX_FIFO_UCODE)?;


        self.write(regs::InitCtrl::DEFAULT.with_init_ctrl(true))?;


        self.delay.delay_ms(200);

        let status = self.read::<regs::InternalStatus>()?;

        Ok(status.message())
    }
    
    pub fn enable(&mut self) -> Result<(), S::Error> {
        self.write(regs::PwrConf::DEFAULT.with_fifo_self_wake_up(true).with_adv_power_save(false).with_fup_en(true))?;

        self.delay.delay_ms(1); 

        self.write(regs::AccConf::DEFAULT
            .with_acc_odr(regs::OutputDataRate::Odr100)
            .with_acc_filter_perf(true)
            .with_acc_bwp(regs::AccBwp::NormAvg4)
        )?;


        self.delay.delay_ms(1); 

        self.write(regs::AccRange::DEFAULT
            .with_acc_range(regs::AccRangeMode::Range2G)
        )?;


        self.delay.delay_ms(1); 

        self.write(regs::GyrConf::DEFAULT
            .with_gyr_odr(regs::OutputDataRate::Odr200)
            .with_gyr_filter_perf(true)
            .with_gyr_noise_perf(false)
            .with_gyro_bwp(regs::GyrBwp::Norm)
        )?;

        self.delay.delay_ms(1); 

        self.write(regs::PwrCtrl::DEFAULT.with_acc_en(true).with_gyr_en(true).with_temp_en(true).with_aux_en(false))?;

        Ok(())
    }
    
    /// Read the value from the given register
    pub fn read<T: super::Register>(&mut self) -> Result<T, S::Error> {
        let mut buf = [0xff, 0xff];
        self.spi.transaction(&mut [
            Operation::Write(&[T::ADDRESS as u8 | 0x80]),
            Operation::Read(&mut buf)
        ])?;

        Ok(
            T::from(buf[1])
        )
    }
    
    /// Burst write `buf` to the register address given
    pub fn burst_write<T: super::Register>(&mut self, buf: &[u8]) -> Result<(), S::Error> {
        self.spi.transaction(&mut [
            Operation::Write(&[T::ADDRESS as u8]),
            Operation::Write(buf)
        ])
    }
    
    /// Write the register bitfield into the given address
    pub fn write<T: super::Register>(&mut self, v: T) -> Result<(), S::Error> {
        self.spi.write(&[T::ADDRESS as u8, v.into()])
    }


    
    /// Set the address used for initializing config file
    fn set_init_addr(&mut self, addr: u12) -> Result<(), S::Error> {
        let bits_0_3 = u4::masked_new(addr);
        let bits_4_11 = (addr.as_u16() >> 4) as u8;
        self.spi.transaction(&mut [
            Operation::Write(&[<regs::InitAddr0 as super::Register>::ADDRESS as u8]),
            Operation::Write(&[bits_0_3.as_u8(), bits_4_11])
        ])
    }
}
