/// 12-bit memory address.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Addr(u16);

impl Addr {
    pub fn offset(&self, offset: i32) -> Option<Self> {
        let sum = self.0 as i32 + offset;
        if sum < 0 || sum > 0x0FFF { None } else { Some(Addr(sum as u16)) }
    }
}

impl Default for Addr {
    fn default() -> Self {
        Addr(0x0200)
    }
}

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

impl std::fmt::Display for Addr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:#05X}", self.0)
    }
}

/// Memory address where font data is stored.
pub const FONT_OFFSET: Addr = Addr(0x00_0000);

/// CHIP-8 built-in binary font data.
/// Taken from [@wernsey][0].
///
/// [0]: https://github.com/wernsey/chip8/blob/a2310220ce5c205c74d85bc69125339641ee838a/chip8.c#L56-L74
const FONT: [u8; 80] = [
    /* 0 */ 0xF0, 0x90, 0x90, 0x90, 0xF0,
    /* 1 */ 0x20, 0x60, 0x20, 0x20, 0x70,
    /* 2 */ 0xF0, 0x10, 0xF0, 0x80, 0xF0,
    /* 3 */ 0xF0, 0x10, 0xF0, 0x10, 0xF0,
    /* 4 */ 0x90, 0x90, 0xF0, 0x10, 0x10,
    /* 5 */ 0xF0, 0x80, 0xF0, 0x10, 0xF0,
    /* 6 */ 0xF0, 0x80, 0xF0, 0x90, 0xF0,
    /* 7 */ 0xF0, 0x10, 0x20, 0x40, 0x40,
    /* 8 */ 0xF0, 0x90, 0xF0, 0x90, 0xF0,
    /* 9 */ 0xF0, 0x90, 0xF0, 0x10, 0xF0,
    /* A */ 0xF0, 0x90, 0xF0, 0x90, 0x90,
    /* B */ 0xE0, 0x90, 0xE0, 0x90, 0xE0,
    /* C */ 0xF0, 0x80, 0x80, 0x80, 0xF0,
    /* D */ 0xE0, 0x80, 0x80, 0x80, 0xE0,
    /* E */ 0xF0, 0x80, 0xF0, 0x80, 0xF0,
    /* F */ 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

/// 4096-byte random access memory.
#[derive(Clone)]
pub struct Mem(Box<[u8; 4096]>);

impl Mem {
    pub fn new<B>(program: B) -> Self where B: IntoIterator<Item = u8> {
        let mut mem = Box::new([0; 4096]);
        mem.iter_mut()
            .zip(FONT.iter())
            .for_each(|(lhs, rhs)| *lhs = *rhs);
        mem.iter_mut()
            .skip(0x200)
            .zip(program.into_iter())
            .for_each(|(lhs, rhs)| *lhs = rhs);
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
