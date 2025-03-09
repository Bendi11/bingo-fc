pub mod bmi270;

pub trait Register {
    const ADDRESS: u32;
}
