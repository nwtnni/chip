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
