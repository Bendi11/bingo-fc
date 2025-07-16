use arbitrary_int::{u12, u24, u4, Number};
use embedded_hal::spi::{Operation, SpiDevice};
use stm32f4xx_hal::rcc::Clocks;

pub mod regs;

/// Driver for the BMI270 IMU on an SPI bus
pub struct Bmi270<D: SpiDevice> {
    spi: D,
}

impl<D: SpiDevice> Bmi270<D> {
    const BMI270_MAX_FIFO_UCODE: [u8 ; 328] = [
        0xc8, 0x2e, 0x00, 0x2e, 0x80, 0x2e, 0x1a, 0x00, 0xc8, 0x2e, 0x00, 0x2e, 0xc8, 0x2e, 0x00, 0x2e, 0xc8, 0x2e, 0x00,
        0x2e, 0xc8, 0x2e, 0x00, 0x2e, 0xc8, 0x2e, 0x00, 0x2e, 0xc8, 0x2e, 0x00, 0x2e, 0x90, 0x32, 0x21, 0x2e, 0x59, 0xf5,
        0x10, 0x30, 0x21, 0x2e, 0x6a, 0xf5, 0x1a, 0x24, 0x22, 0x00, 0x80, 0x2e, 0x3b, 0x00, 0xc8, 0x2e, 0x44, 0x47, 0x22,
        0x00, 0x37, 0x00, 0xa4, 0x00, 0xff, 0x0f, 0xd1, 0x00, 0x07, 0xad, 0x80, 0x2e, 0x00, 0xc1, 0x80, 0x2e, 0x00, 0xc1,
        0x80, 0x2e, 0x00, 0xc1, 0x80, 0x2e, 0x00, 0xc1, 0x80, 0x2e, 0x00, 0xc1, 0x80, 0x2e, 0x00, 0xc1, 0x80, 0x2e, 0x00,
        0xc1, 0x80, 0x2e, 0x00, 0xc1, 0x80, 0x2e, 0x00, 0xc1, 0x80, 0x2e, 0x00, 0xc1, 0x80, 0x2e, 0x00, 0xc1, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x11, 0x24, 0xfc, 0xf5, 0x80, 0x30, 0x40, 0x42, 0x50, 0x50, 0x00, 0x30, 0x12, 0x24, 0xeb,
        0x00, 0x03, 0x30, 0x00, 0x2e, 0xc1, 0x86, 0x5a, 0x0e, 0xfb, 0x2f, 0x21, 0x2e, 0xfc, 0xf5, 0x13, 0x24, 0x63, 0xf5,
        0xe0, 0x3c, 0x48, 0x00, 0x22, 0x30, 0xf7, 0x80, 0xc2, 0x42, 0xe1, 0x7f, 0x3a, 0x25, 0xfc, 0x86, 0xf0, 0x7f, 0x41,
        0x33, 0x98, 0x2e, 0xc2, 0xc4, 0xd6, 0x6f, 0xf1, 0x30, 0xf1, 0x08, 0xc4, 0x6f, 0x11, 0x24, 0xff, 0x03, 0x12, 0x24,
        0x00, 0xfc, 0x61, 0x09, 0xa2, 0x08, 0x36, 0xbe, 0x2a, 0xb9, 0x13, 0x24, 0x38, 0x00, 0x64, 0xbb, 0xd1, 0xbe, 0x94,
        0x0a, 0x71, 0x08, 0xd5, 0x42, 0x21, 0xbd, 0x91, 0xbc, 0xd2, 0x42, 0xc1, 0x42, 0x00, 0xb2, 0xfe, 0x82, 0x05, 0x2f,
        0x50, 0x30, 0x21, 0x2e, 0x21, 0xf2, 0x00, 0x2e, 0x00, 0x2e, 0xd0, 0x2e, 0xf0, 0x6f, 0x02, 0x30, 0x02, 0x42, 0x20,
        0x26, 0xe0, 0x6f, 0x02, 0x31, 0x03, 0x40, 0x9a, 0x0a, 0x02, 0x42, 0xf0, 0x37, 0x05, 0x2e, 0x5e, 0xf7, 0x10, 0x08,
        0x12, 0x24, 0x1e, 0xf2, 0x80, 0x42, 0x83, 0x84, 0xf1, 0x7f, 0x0a, 0x25, 0x13, 0x30, 0x83, 0x42, 0x3b, 0x82, 0xf0,
        0x6f, 0x00, 0x2e, 0x00, 0x2e, 0xd0, 0x2e, 0x12, 0x40, 0x52, 0x42, 0x00, 0x2e, 0x12, 0x40, 0x52, 0x42, 0x3e, 0x84,
        0x00, 0x40, 0x40, 0x42, 0x7e, 0x82, 0xe1, 0x7f, 0xf2, 0x7f, 0x98, 0x2e, 0x6a, 0xd6, 0x21, 0x30, 0x23, 0x2e, 0x61,
        0xf5, 0xeb, 0x2c, 0xe1, 0x6f
    ];

    /// Create a new BMI270 driver from an SpiDevice type
    pub fn new(spi: D) -> Self {
        Self {
            spi,
        }
    }
    
    /// Get the sensor time from the sensor
    pub fn sensor_time(&mut self) -> Result<u24, D::Error> {
        let mut buf = [0u8 ; 4];
        self.spi.transaction(&mut [
            Operation::Write(&[<regs::SensorTime0 as super::Register>::ADDRESS as u8 | 0b10000000]),
            Operation::Read(&mut buf)
        ])?;

        Ok(u24::from_le_bytes([buf[1], buf[2], buf[3]]))
    }

    pub fn data(&mut self) -> Result<((u16, u16, u16), (u16, u16, u16)), D::Error> {
        let mut buf = [0u8 ; 13];
        self.spi.transaction(&mut [
            Operation::Write(&[0x0C | 0b10000000]),
            Operation::Read(&mut buf)
        ])?;

        let decode = |idx| u16::from_le_bytes([buf[idx], buf[idx + 1]]);

        Ok((
            (decode(1), decode(3), decode(5)),
            (decode(7), decode(9), decode(11))
        ))
    } 

    pub fn status(&mut self) -> Result<regs::InternalStatus, D::Error> {
        self.read::<regs::InternalStatus>()
    }
    
    /// Initialize the IMU configuration file and power settings
    pub fn init(&mut self, clk: &Clocks, delay: &mut impl embedded_hal::delay::DelayNs) -> Result<regs::InternalStatusMessage, D::Error> {
        //Issue unused read to take the BMI270 out of I2C mode if it has not been already
        let id = self.read::<regs::ChipId>()?;


        self.write(regs::Cmd::DEFAULT.with_field(regs::CmdField::SoftReset))?;

        delay.delay_ms(200);

        let _ = self.read::<regs::ChipId>()?;

        let pwrconf = self.read::<regs::PwrConf>()?;
        self.write(pwrconf.with_adv_power_save(false))?;


        self.write(regs::InitCtrl::DEFAULT.with_init_ctrl(false))?;

        self.set_init_addr(u12::new(0))?;
        self.burst_write::<regs::InitData>(&Self::BMI270_MAX_FIFO_UCODE)?;


        self.write(regs::InitCtrl::DEFAULT.with_init_ctrl(true))?;

        let status = self.read::<regs::InternalStatus>()?;

        Ok(status.message())
    }
    
    pub fn enable(&mut self, delay: &mut impl embedded_hal::delay::DelayNs) -> Result<(), D::Error> {
        self.write(regs::PwrConf::DEFAULT.with_fifo_self_wake_up(true).with_adv_power_save(false).with_fup_en(true))?;

        self.write(regs::AccConf::DEFAULT
            .with_acc_odr(regs::OutputDataRate::Odr100)
            .with_acc_filter_perf(true)
            .with_acc_bwp(regs::AccBwp::NormAvg4)
        )?;


        delay.delay_ms(1); 

        self.write(regs::AccRange::DEFAULT
            .with_acc_range(regs::AccRangeMode::Range2G)
        )?;


        delay.delay_ms(1); 

        self.write(regs::GyrConf::DEFAULT
            .with_gyr_odr(regs::OutputDataRate::Odr200)
            .with_gyr_filter_perf(true)
            .with_gyr_noise_perf(false)
            .with_gyro_bwp(regs::GyrBwp::Norm)
        )?;


        self.write(regs::PwrCtrl::DEFAULT.with_acc_en(true).with_gyr_en(true).with_temp_en(true).with_aux_en(false))?;

        Ok(())
    }
    
    /// Read the value from the given register
    pub fn read<T: super::Register>(&mut self) -> Result<T, D::Error> {
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
    pub fn burst_write<T: super::Register>(&mut self, buf: &[u8]) -> Result<(), D::Error> {
        self.spi.transaction(&mut [
            Operation::Write(&[T::ADDRESS as u8]),
            Operation::Write(buf)
        ])
    }
    
    /// Write the register bitfield into the given address
    pub fn write<T: super::Register>(&mut self, v: T) -> Result<(), D::Error> {
        self.spi.write(&[T::ADDRESS as u8, v.into()])
    }


    
    /// Set the address used for initializing config file
    fn set_init_addr(&mut self, addr: u12) -> Result<(), D::Error> {
        let bits_0_3 = u4::masked_new(addr);
        let bits_4_11 = (addr.as_u16() >> 4) as u8;
        self.spi.transaction(&mut [
            Operation::Write(&[<regs::InitAddr0 as super::Register>::ADDRESS as u8]),
            Operation::Write(&[bits_0_3.as_u8(), bits_4_11])
        ])
    }
}
