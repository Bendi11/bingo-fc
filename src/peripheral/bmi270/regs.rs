use bingofc_derive::register;


pub trait Register: From<u8> + Into<u8> {
    const ADDRESS: u32;
}


#[register(addr = 0x00, reset = 0x24)]
pub struct ChipId {
    #[bits(0..=7, r)]
    pub id: u8
}

