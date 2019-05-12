use crate::ram;
use crate::stack;

/// 4-bit general purpose register
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Reg(u8);

impl From<u8> for Reg {
    fn from(reg: u8) -> Self {
        debug_assert!(reg < 0x10);
        Reg(reg)
    }
}

impl From<Reg> for u8 {
    fn from(reg: Reg) -> Self {
        reg.0
    }
}

#[derive(Debug, Default)]
pub struct CPU {
    /// Program counter
    pub pc: ram::Addr,

    /// Stack pointer
    pub sp: stack::Addr, 

    /// Sound timer
    pub st: u8,

    /// Delay timer
    pub dt: u8,

    /// Index register
    pub idx: u16, 

    /// General purpose registers
    pub reg: [u8; 16],
}
