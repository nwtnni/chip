/// Little-endian 12-bit memory address
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Addr(u16);

impl From<u16> for Addr {
    fn from(addr: u16) -> Self {
        Addr(addr & 0x0FFF)
    }
}
