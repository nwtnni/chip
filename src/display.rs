/// Width
pub const W: u8 = 64;

/// Height
pub const H: u8 = 32;

/// Leftmost bit for masking pixels
const MSB: u64 = 0x8000_0000_0000_0000;

#[derive(Clone, Debug, Default)]
pub struct Display {
    /// Vertical offset
    dy: u16,

    /// Horizontal offset
    dx: u16,

    /// Pixel grid
    grid: [u64; H as usize],

    /// Dirty pixels
    dirt: std::cell::RefCell<Vec<(u8, u8)>>,
}

impl Display {
    pub fn clear(&mut self) {
        let mut dirt = self.dirt.borrow_mut();
        dirt.clear();
        for (y, row) in self.grid.iter_mut().enumerate() {
            let mut col = MSB;
            for x in 0..W {
                if *row & col > 0 { dirt.push((x, y as u8)); }
                col >>= 1;
            }
            *row = 0;
        }
    }

    pub fn toggle(&mut self, x: u8, y: u8) -> u8 {
        let (x, y) = (x % W, y % H);
        let bit = MSB >> x;
        let hit = self.grid[y as usize] & bit > 0;
        self.grid[y as usize] ^= bit;
        self.dirt.borrow_mut().push((x, y));
        hit as u8
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (x, y) in self.dirt.borrow_mut().drain(..) {
            let set = (self.grid[y as usize] & (MSB >> x)) > 0;
            let bit = if set { '█' } else { ' ' };
            let go = termion::cursor::Goto(x as u16 + self.dx, y as u16 + self.dy);
            write!(fmt, "{}{}", go, bit)?;
        }
        Ok(())
    }
}
