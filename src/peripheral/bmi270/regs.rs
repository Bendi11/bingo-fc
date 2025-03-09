use arbitrary_int::u4;
use bingofc_derive::register;
use bitbybit::bitenum;


pub trait Register: From<u8> + Into<u8> {
    const ADDRESS: u32;
}


#[register(addr = 0x00, reset = 0x24)]
pub struct ChipId {
    #[bits(0..=7, r)]
    pub id: u8
}

#[register(addr = 0x02, reset = 0x00)]
pub struct ErrReg {
    #[bit(1, r)]
    pub fatal_err: bool,
    #[bits(1..=4, r)]
    pub internal_err: u4,
}

#[register(addr = 0x03, reset = 0x10)]
pub struct Status {
    #[bit(2, r)]
    pub aux_busy: bool,
    #[bit(4, r)]
    pub cmd_rdy: bool,
    #[bit(5, r)]
    pub drdy_aux: bool,
    #[bit(6, r)]
    pub drdy_gyr: bool,
    #[bit(7, r)]
    pub drdy_acc: bool,
}

#[bitenum(u2, exhaustive = true)]
pub enum ErrorCode {
    NoError = 0x00,
    AccErr = 0x01,
    GyrErr = 0x02,
    AccAndGyrErr = 0x03
}

#[register(addr = 0x1B, reset = 0x01)]
pub struct Event {
    #[bit(0, r)]
    pub por_detected: bool,
    #[bits(2..=3, r)]
    pub error_code: ErrorCode,
}

#[register(addr = 0x1C, reset = 0x00)]
pub struct IntStatus0 {
    #[bit(0, r)] pub sig_motion_out: bool,
    #[bit(1, r)] pub step_counter_out: bool,
    #[bit(2, r)] pub activity_out: bool,
    #[bit(3, r)] pub wrist_wear_wakeup_out: bool,
    #[bit(4, r)] pub wrist_gesture_out: bool,
    #[bit(5, r)] pub no_motion_out: bool,
    #[bit(6, r)] pub any_motion_out: bool,
}

#[register(addr = 0x1D, reset = 0x00)]
pub struct IntStatus1 {
    #[bit(0, r)] pub ffull_int: bool,
    #[bit(1, r)] pub fwm_int: bool,
    #[bit(2, r)] pub err_int: bool,
    #[bit(5, r)] pub aux_drdy_int: bool,
    #[bit(6, r)] pub gyr_drdy_int: bool,
    #[bit(7, r)] pub acc_drdy_int: bool,
}

#[bitenum(u3, exhaustive = true)]
pub enum InternalStatusMessage {
    NotInit = 0x00,
    InitOk = 0x01,
    InitErr = 0x02,
    DrvErr = 0x03,
    SnsStop = 0x04,
    NvmError = 0x05,
    StartUpError = 0x06,
    CompatError = 0x07,
}

#[register(addr = 0x21, reset = 0x00)]
pub struct InternalStatus {
    #[bits(0..=2, r)] pub message: InternalStatusMessage,
    #[bit(5, r)] pub axes_remap_error: bool,
    #[bit(6, r)] pub odr_50hz_error: bool,
}

#[bitenum(u4, exhaustive = true)]
pub enum OutputDataRate {
    Reserved = 0x00,
    Odr0p78 = 0x01,
    Odr1p5 = 0x02,
    Odr3p1 = 0x03,
    Odr6p25 = 0x04,
    Odr12p5 = 0x05,
    Odr25 = 0x06,
    Odr50 = 0x07,
    Odr100 = 0x08,
    Odr200 = 0x09,
    Odr400 = 0x0a,
    Odr800 = 0x0b,
    Odr1k6 = 0x0c,
    Odr3k2 = 0x0d,
    Odr6k4 = 0x0e,
    Odr12k8 = 0x0f
}

#[bitenum(u3, exhaustive = true)]
pub enum AccBwp {
    Osr4Avg1 = 0x00,
    Osr2Avg2 = 0x01,
    NormAvg4 = 0x02,
    CicAvg8 = 0x03,
    ResAvg16 = 0x04,
    ResAvg32 = 0x05,
    ResAvg64 = 0x06,
    ResAvg128 = 0x07,
}


#[register(addr = 0x40, reset = 0xA8)]
pub struct AccConf {
    #[bits(0..=3, rw)] pub acc_odr: OutputDataRate,
    #[bits(4..=6, rw)] pub acc_bwp: AccBwp,
    #[bit(7, rw)] pub acc_filter_perf: bool,
}

#[bitenum(u2, exhaustive = true)]
pub enum AccRangeMode {
    Range2G = 0x00,
    Range4G = 0x01,
    Range8G = 0x02,
    Range16G = 0x03,
}

#[register(addr = 0x41, reset = 0x02)]
pub struct AccRange {
    #[bits(0..=1, rw)] pub acc_range: AccRangeMode,
}

#[bitenum(u2, exhaustive = true)]
pub enum GyrBwp {
    Osr4 = 0x00,
    Osr2 = 0x01,
    Norm = 0x02,
    Reserved = 0x03,
}

#[register(addr = 0x42, reset = 0xA9)]
pub struct GyrConf {
    #[bits(0..=3, rw)] pub gyr_odr: OutputDataRate,
    #[bits(4..=5, rw)] pub gyro_bwp: GyrBwp,
    #[bit(6, rw)] pub gyr_noise_perf: bool,
    #[bit(7, rw)] pub gyr_filter_perf: bool,
}

#[bitenum(u3, exhaustive = true)]
pub enum GyrRangeMode {
    Range2000 = 0x00,
    Range1000 = 0x01,
    Range500 = 0x02,
    Range250 = 0x03,
    Range125 = 0x04,
    Reserved0 = 0x05,
    Reserved1 = 0x06,
    Reserved2 = 0x07
}

#[bitenum(u1, exhaustive = true)]
pub enum OisRange {
    Range250 = 0x00,
    Range2000 = 0x01,
}

#[register(addr = 0x43, reset = 0x00)]
pub struct GyrRange {
    #[bits(0..=2, rw)] pub gyr_range: GyrRangeMode,
    #[bit(3, rw)] pub ois_range: OisRange,
}
