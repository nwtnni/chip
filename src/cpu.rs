/// 4-bit general purpose register
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Reg(u8);

impl From<u8> for Reg {
    fn from(reg: u8) -> Self {
        debug_assert!(reg < 0x10);
        Reg(reg)
    }
}

#[derive(Debug)]
pub struct CPU {
    /// Program counter
    pc: u16,

    /// Stack pointer
    sp: u16, 

    /// Sound timer
    st: u8,

    /// Delay timer
    dt: u8,

    /// Index register
    idx: u16, 

    /// General purpose registers
    reg: [u8; 16],
}
