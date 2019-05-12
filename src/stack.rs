use crate::ram;

/// 8-bit stack address
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Addr(u8);

impl std::ops::AddAssign<u8> for Addr {
    fn add_assign(&mut self, rhs: u8) {
        self.0 += rhs
    }
}

impl std::ops::SubAssign<u8> for Addr {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 -= rhs
    }
}

/// 64-byte stack memory
pub struct Mem([ram::Addr; 16]);

impl std::ops::Index<Addr> for Mem {
    type Output = ram::Addr;
    fn index(&self, addr: Addr) -> &Self::Output {
        &self.0[addr.0 as usize]
    }
}

impl std::ops::IndexMut<Addr> for Mem {
    fn index_mut(&mut self, addr: Addr) -> &mut Self::Output {
        &mut self.0[addr.0 as usize]
    }
}
