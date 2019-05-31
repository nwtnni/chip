use crate::ram;

/// 8-bit stack address
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Addr(u8);

impl Addr {
    pub fn dec(&mut self) {
        debug_assert!(self.0 > 0x00, "Stack address cannot be below 0x00");
        self.0 -= 1
    }

    pub fn inc(&mut self) {
        debug_assert!(self.0 < 0x0F, "Stack address cannot be above 0x0F");
        self.0 += 1
    }
}

/// 64-byte stack memory
#[derive(Copy, Clone, Debug, Default)]
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
