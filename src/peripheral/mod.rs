pub mod bmi270;

pub trait Register: From<u8> + Into<u8> {
    const ADDRESS: u32;
}
