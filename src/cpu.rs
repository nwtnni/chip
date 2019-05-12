use crate::ram;
use crate::stack;

/// 4-bit general purpose register
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Reg(u8);

pub const VF: Reg = Reg(0x0F);

impl From<u8> for Reg {
    fn from(reg: u8) -> Self {
        debug_assert!(reg < 0x10);
        Reg(reg)
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

impl std::ops::Index<Reg> for CPU {
    type Output = u8;
    fn index(&self, idx: Reg) -> &Self::Output {
        &self.reg[idx.0 as usize]
    }
}

impl std::ops::IndexMut<Reg> for CPU {
    fn index_mut(&mut self, idx: Reg) -> &mut Self::Output {
        &mut self.reg[idx.0 as usize]
    }
}
