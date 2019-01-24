use crate::mem;
use crate::reg;

pub enum Op {

    /// Jump to machine code routine at address
    SYS(mem::Addr),

    /// Clear screen
    CLS,

    /// Return from subroutine
    RET,

    /// Jump to address
    JP(mem::Addr),

    /// Jump to address plus offset V0
    JO(mem::Addr),

    /// Call subroutine at address
    CALL(mem::Addr),

    /// Skip next instruction if register contents equal constant
    SEC(reg::Reg, u8),

    /// Skip next instruction if register contents do not equal constant
    SNEC(reg::Reg, u8),

    /// Skip next instruction if register contents are equal
    SER(reg::Reg, reg::Reg),

    /// Skip next instruction if register contents are not equal
    SNER(reg::Reg, reg::Reg),

    /// Load constant into register
    LDC(reg::Reg, u8),

    /// Load register contents into register
    LDR(reg::Reg, reg::Reg), 

    /// Load address into register I
    LDI(mem::Addr),

    /// Destructive constant addition
    ADDC(reg::Reg, u8),

    /// Destructive register addition into I
    ADDI(reg::Reg),

    /// Destructive register addition
    ADDR(reg::Reg, reg::Reg),

    /// Destructive register bitwise AND
    AND(reg::Reg, reg::Reg),

    /// Destructive register bitwise OR
    OR(reg::Reg, reg::Reg), 

    /// Destructive register bitwise XOR
    XOR(reg::Reg, reg::Reg), 

    /// Destructive left-to-right register subtraction
    SUB(reg::Reg, reg::Reg),

    /// Destructive right-to-left register subtraction
    SUBN(reg::Reg, reg::Reg),

    /// Shift right by one
    SHR(reg::Reg),

    /// Shift left by one
    SHL(reg::Reg),

    /// Generate random number and bitwise AND with constant
    RND(reg::Reg, u8),

    /// Display n-byte sprite
    DRW(reg::Reg, reg::Reg, ux::u4),

    /// Skip if key with value is pressed
    SKP(reg::Reg),

    /// Skip if key with value is not pressed
    SKNP(reg::Reg),

    /// Load value of delay timer into register
    LDTR(reg::Reg),

    /// Load value of key press
    LDK(reg::Reg),

    /// Load value of register into delay timer
    LDRT(reg::Reg),

    /// Load value of register into sound timer
    LDRS(reg::Reg),

    /// Set I to location of sprite for digit
    LDS(reg::Reg),

    /// Store BCD representation of register in memory locations I, I + 1, I + 2
    LDB(reg::Reg),

    /// Write registers V0 through Reg in memory starting at location I
    WR(reg::Reg),

    /// Read registers V0 through Reg from memory starting at location I
    RD(reg::Reg),
}
