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

impl Asm {
    pub fn parse(hi: u8, lo: u8) -> Option<Self> {
        let op = ((hi as u16) << 8) | (lo as u16);
        match (hi >> 4, hi & 0xF, lo >> 4, lo & 0xF) {
        | (0x0, 0x0, 0xE, 0x0) => Some(Asm::CLS),
        | (0x0, 0x0, 0xE, 0xE) => Some(Asm::RET),
        | (0x0,   _,   _,   _) => Some(Asm::SYS(op.into())),
        | (0x1,   _,   _,   _) => Some(Asm::JP(op.into())),
        | (0x2,   _,   _,   _) => Some(Asm::CALL(op.into())),
        | (0x3,   x,   _,   _) => Some(Asm::SEC(x.into(), op as u8)),
        | (0x4,   x,   _,   _) => Some(Asm::SNEC(x.into(), op as u8)),
        | (0x5,   x,   y, 0x0) => Some(Asm::SER(x.into(), y.into())),
        | (0x6,   x,   _,   _) => Some(Asm::LDC(x.into(), op as u8)),
        | (0x7,   x,   _,   _) => Some(Asm::ADDC(x.into(), op as u8)),
        | (0x8,   x,   y, 0x0) => Some(Asm::LDR(x.into(), y.into())),
        | (0x8,   x,   y, 0x1) => Some(Asm::OR(x.into(), y.into())),
        | (0x8,   x,   y, 0x2) => Some(Asm::AND(x.into(), y.into())),
        | (0x8,   x,   y, 0x3) => Some(Asm::XOR(x.into(), y.into())),
        | (0x8,   x,   y, 0x4) => Some(Asm::ADDR(x.into(), y.into())),
        | (0x8,   x,   y, 0x5) => Some(Asm::SUB(x.into(), y.into())),
        | (0x8,   x,   _, 0x6) => Some(Asm::SHR(x.into())),
        | (0x8,   x,   y, 0x7) => Some(Asm::SUBN(x.into(), y.into())),
        | (0x8,   x,   _, 0xE) => Some(Asm::SHL(x.into())),
        | (0x9,   x,   y, 0x0) => Some(Asm::SNER(x.into(), y.into())),
        | (0xA,   _,   _,   _) => Some(Asm::LDI(op.into())),
        | (0xB,   _,   _,   _) => Some(Asm::JO(op.into())),
        | (0xC,   x,   _,   _) => Some(Asm::RND(x.into(), op as u8)),
        | (0xD,   x,   y,   n) => Some(Asm::DRW(x.into(), y.into(), n)),
        | (0xE,   x, 0x9, 0xE) => Some(Asm::SKP(x.into())),
        | (0xE,   x, 0xA, 0x1) => Some(Asm::SKNP(x.into())),
        | (0xF,   x, 0x0, 0x7) => Some(Asm::LDTR(x.into())),
        | (0xF,   x, 0x0, 0xA) => Some(Asm::LDK(x.into())),
        | (0xF,   x, 0x1, 0x5) => Some(Asm::LDRT(x.into())),
        | (0xF,   x, 0x1, 0x8) => Some(Asm::LDRS(x.into())),
        | (0xF,   x, 0x1, 0xE) => Some(Asm::ADDI(x.into())),
        | (0xF,   x, 0x2, 0x9) => Some(Asm::LDS(x.into())),
        | (0xF,   x, 0x3, 0x3) => Some(Asm::LDB(x.into())),
        | (0xF,   x, 0x5, 0x5) => Some(Asm::WR(x.into())),
        | (0xF,   x, 0x6, 0x5) => Some(Asm::RD(x.into())),
        | _ => None,
        }
    }
}

impl std::fmt::Display for Asm {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
        | Asm::SYS(addr)    => write!(fmt, "SYS {}", addr),
        | Asm::CLS          => write!(fmt, "CLS"),
        | Asm::RET          => write!(fmt, "RET"),
        | Asm::JP(addr)     => write!(fmt, "JP {}", addr),
        | Asm::CALL(addr)   => write!(fmt, "CALL {}", addr),
        | Asm::SEC(x, kk)   => write!(fmt, "SE {}, {:#04X}", x, kk),
        | Asm::SNEC(x, kk)  => write!(fmt, "SNE {}, {:#04X}", x, kk),
        | Asm::SER(x, y)    => write!(fmt, "SE {}, {}", x, y),
        | Asm::LDC(x, kk)   => write!(fmt, "LD {}, {:#04X}", x, kk),
        | Asm::ADDC(x, kk)  => write!(fmt, "ADD {}, {:#04X}", x, kk),
        | Asm::LDR(x, y)    => write!(fmt, "LD {}, {}", x, y),
        | Asm::OR(x, y)     => write!(fmt, "OR {}, {}", x, y),
        | Asm::AND(x, y)    => write!(fmt, "AND {}, {}", x, y),
        | Asm::XOR(x, y)    => write!(fmt, "XOR {}, {}", x, y),
        | Asm::ADDR(x, y)   => write!(fmt, "ADD {}, {}", x, y),
        | Asm::SUB(x, y)    => write!(fmt, "SUB {}, {}", x, y),
        | Asm::SHR(x)       => write!(fmt, "SHR {}", x),
        | Asm::SUBN(x, y)   => write!(fmt, "SUBN {}, {}", x, y),
        | Asm::SHL(x)       => write!(fmt, "SHL {}", x),
        | Asm::SNER(x, y)   => write!(fmt, "SNE {}, {}", x, y),
        | Asm::LDI(addr)    => write!(fmt, "LD I, {}", addr),
        | Asm::JO(addr)     => write!(fmt, "JP V0, {}", addr),
        | Asm::RND(x, kk)   => write!(fmt, "RND {}, {}", x, kk),
        | Asm::DRW(x, y, k) => write!(fmt, "DRW {}, {}, {}", x, y, k),
        | Asm::SKP(x)       => write!(fmt, "SKP {}", x),
        | Asm::SKNP(x)      => write!(fmt, "SKNP {}", x),
        | Asm::LDTR(x)      => write!(fmt, "LD {}, DT", x),
        | Asm::LDK(x)       => write!(fmt, "LD {}, K", x),
        | Asm::LDRT(x)      => write!(fmt, "LD DT, {}", x),
        | Asm::LDRS(x)      => write!(fmt, "LD ST, {}", x),
        | Asm::ADDI(x)      => write!(fmt, "ADD I, {}", x),
        | Asm::LDS(x)       => write!(fmt, "LD F, {}", x),
        | Asm::LDB(x)       => write!(fmt, "LD B, {}", x),
        | Asm::WR(x)        => write!(fmt, "LD [I], V{:X}", x),
        | Asm::RD(x)        => write!(fmt, "LD V{:X}, [I]", x),
        }
    }
}
