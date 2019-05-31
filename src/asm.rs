use crate::cpu;
use crate::ram;

/// CHIP-8 opcodes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Asm {
    /// Jump to machine code routine at address
    SYS(ram::Addr),

    /// Clear screen
    CLS,

    /// Return from subroutine
    RET,

    /// Jump to address
    JP(ram::Addr),

    /// Call subroutine at address
    CALL(ram::Addr),

    /// Skip next instruction if register contents equal constant
    SEC(cpu::Reg, u8),

    /// Skip next instruction if register contents do not equal constant
    SNEC(cpu::Reg, u8),

    /// Skip next instruction if register contents are equal
    SER(cpu::Reg, cpu::Reg),

    /// Load constant into register
    LDC(cpu::Reg, u8),

    /// Destructive constant addition
    ADDC(cpu::Reg, u8),

    /// Load register contents into register
    LDR(cpu::Reg, cpu::Reg),

    /// Destructive register bitwise OR
    OR(cpu::Reg, cpu::Reg),

    /// Destructive register bitwise AND
    AND(cpu::Reg, cpu::Reg),

    /// Destructive register bitwise XOR
    XOR(cpu::Reg, cpu::Reg),

    /// Destructive register addition
    ADDR(cpu::Reg, cpu::Reg),

    /// Destructive left-to-right register subtraction
    SUB(cpu::Reg, cpu::Reg),

    /// Shift right by one
    SHR(cpu::Reg),

    /// Destructive right-to-left register subtraction
    SUBN(cpu::Reg, cpu::Reg),

    /// Shift left by one
    SHL(cpu::Reg),

    /// Skip next instruction if register contents are not equal
    SNER(cpu::Reg, cpu::Reg),

    /// Load address into register I
    LDI(ram::Addr),

    /// Jump to address plus offset V0
    JO(ram::Addr),

    /// Generate random number and bitwise AND with constant
    RND(cpu::Reg, u8),

    /// Display n-byte sprite
    DRW(cpu::Reg, cpu::Reg, u8),

    /// Skip if key with value is pressed
    SKP(cpu::Reg),

    /// Skip if key with value is not pressed
    SKNP(cpu::Reg),

    /// Load value of delay timer into register
    LDTR(cpu::Reg),

    /// Load value of key press
    LDK(cpu::Reg),

    /// Load value of register into delay timer
    LDRT(cpu::Reg),

    /// Load value of register into sound timer
    LDRS(cpu::Reg),

    /// Destructive register addition into I
    ADDI(cpu::Reg),

    /// Set I to location of sprite for digit
    LDS(cpu::Reg),

    /// Store BCD representation of register in memory locations I, I + 1, I + 2
    LDB(cpu::Reg),

    /// Write registers V0 through Vx in memory starting at location I
    WR(u8),

    /// Read registers V0 through Vx from memory starting at location I
    RD(u8),
}

impl From<u16> for Asm {
    fn from(op: u16) -> Self {

        macro_rules! nibble {
            ($n:expr) => ((op >> (4 * $n) & 0x000F) as u8)
        }

        match (nibble!(3), nibble!(2), nibble!(1), nibble!(0)) {
        | (0x0, 0x0, 0xE, 0x0) => Asm::CLS,
        | (0x0, 0x0, 0xE, 0xE) => Asm::RET,
        | (0x0,   _,   _,   _) => Asm::SYS(op.into()),
        | (0x1,   _,   _,   _) => Asm::JP(op.into()),
        | (0x2,   _,   _,   _) => Asm::CALL(op.into()),
        | (0x3,   x,   _,   _) => Asm::SEC(x.into(), op as u8),
        | (0x4,   x,   _,   _) => Asm::SNEC(x.into(), op as u8),
        | (0x5,   x,   y, 0x0) => Asm::SER(x.into(), y.into()),
        | (0x6,   x,   _,   _) => Asm::LDC(x.into(), op as u8),
        | (0x7,   x,   _,   _) => Asm::ADDC(x.into(), op as u8),
        | (0x8,   x,   y, 0x0) => Asm::LDR(x.into(), y.into()),
        | (0x8,   x,   y, 0x1) => Asm::OR(x.into(), y.into()),
        | (0x8,   x,   y, 0x2) => Asm::AND(x.into(), y.into()),
        | (0x8,   x,   y, 0x3) => Asm::XOR(x.into(), y.into()),
        | (0x8,   x,   y, 0x4) => Asm::ADDR(x.into(), y.into()),
        | (0x8,   x,   y, 0x5) => Asm::SUB(x.into(), y.into()),
        | (0x8,   x,   _, 0x6) => Asm::SHR(x.into()),
        | (0x8,   x,   y, 0x7) => Asm::SUBN(x.into(), y.into()),
        | (0x8,   x,   _, 0xE) => Asm::SHL(x.into()),
        | (0x9,   x,   y, 0x0) => Asm::SNER(x.into(), y.into()),
        | (0xA,   _,   _,   _) => Asm::LDI(op.into()),
        | (0xB,   _,   _,   _) => Asm::JO(op.into()),
        | (0xC,   x,   _,   _) => Asm::RND(x.into(), op as u8),
        | (0xD,   x,   y,   n) => Asm::DRW(x.into(), y.into(), n),
        | (0xE,   x, 0x9, 0xE) => Asm::SKP(x.into()),
        | (0xE,   x, 0xA, 0x1) => Asm::SKNP(x.into()),
        | (0xF,   x, 0x0, 0x7) => Asm::LDTR(x.into()),
        | (0xF,   x, 0x0, 0xA) => Asm::LDK(x.into()),
        | (0xF,   x, 0x1, 0x5) => Asm::LDRT(x.into()),
        | (0xF,   x, 0x1, 0x8) => Asm::LDRS(x.into()),
        | (0xF,   x, 0x1, 0xE) => Asm::ADDI(x.into()),
        | (0xF,   x, 0x2, 0x9) => Asm::LDS(x.into()),
        | (0xF,   x, 0x3, 0x3) => Asm::LDB(x.into()),
        | (0xF,   x, 0x5, 0x5) => Asm::WR(x.into()),
        | (0xF,   x, 0x6, 0x5) => Asm::RD(x.into()),
        | _ => panic!("[ASSEMBLY ERROR]: invalid opcode {:x}", op),
        }
    }
}
