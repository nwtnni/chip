/// 12-bit memory address
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Addr(u16);

impl From<u16> for Addr {
    fn from(addr: u16) -> Self {
        Addr(addr & 0x0FFF)
    }
}

impl std::ops::Add<u16> for Addr {
    type Output = Self; 
    fn add(self, rhs: u16) -> Self::Output {
        debug_assert!(self.0 + rhs < 0x1000);
        Addr(self.0 + rhs)
    }
}

impl std::ops::AddAssign<u16> for Addr {
    fn add_assign(&mut self, rhs: u16) {
        debug_assert!(self.0 + rhs < 0x1000);
        self.0 += rhs
    }
}

impl std::ops::Sub<u16> for Addr {
    type Output = Self; 
    fn sub(self, rhs: u16) -> Self::Output {
        debug_assert!(self.0 >= rhs);
        Addr(self.0 - rhs)
    }
}

impl std::ops::SubAssign<u16> for Addr {
    fn sub_assign(&mut self, rhs: u16) {
        debug_assert!(self.0 >= rhs);
        self.0 -= rhs
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
