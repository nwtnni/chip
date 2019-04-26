/// 4-bit general purpose register
pub struct Reg(u8);

impl From<u8> for Reg {
    fn from(reg: u8) -> Self {
        debug_assert!(reg < 0x10);
        Reg(reg)
    }
}
