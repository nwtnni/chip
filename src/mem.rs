/// Little-endian 12-bit memory address
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Addr(u16);

impl From<u16> for Addr {
    fn from(addr: u16) -> Self {
        Addr(addr & 0x0FFF)
    }
}

impl From<Addr> for u16 {
    fn from(addr: Addr) -> Self {
        addr.0
    }
}

#[derive(Clone)]
pub struct Mem(Box<[u8; 4096]>);

impl Mem {
    pub fn new() -> Self {
        let mut mem = Box::new([0; 4096]);
        Mem(mem)
    }
}

impl std::ops::Index<Addr> for Mem {
    type Output = u8;
    fn index(&self, addr: Addr) -> &Self::Output {
        &self.0[addr.0 as usize]
    }
}

impl std::ops::IndexMut<Addr> for Mem {
    fn index_mut(&mut self, addr: Addr) -> &mut Self::Output {
        &mut self.0[addr.0 as usize]
    }
}
