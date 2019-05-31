/// Width
pub const W: u8 = 64;

/// Height
pub const H: u8 = 32;

/// Leftmost bit for masking pixels
const MSB: u64 = 0x8000_0000_0000_0000;

#[derive(Clone, Debug, Default)]
pub struct Display {
    grid: [u64; H as usize],
    draw: std::cell::Cell<bool>,
}

impl Display {
    pub fn clear(&mut self) {
        self.grid.iter_mut().for_each(|p| *p = 0);
        self.draw.set(true);
    }

    pub fn toggle(&mut self, x: u8, y: u8) -> u8 {
        let (x, y) = (x % W, y % H);
        let bit = MSB >> x;
        let hit = self.grid[y as usize] & bit > 0;
        self.grid[y as usize] ^= bit;
        self.draw.set(true);
        hit as u8
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.draw.get() { return Ok(()) }
        write!(fmt, "{}", termion::clear::All)?;
        for row in &self.grid {
            let mut col = MSB;
            for _ in 0..W {
                write!(fmt, "{}", if row & col > 0 { "█" } else { " " })?;
                col >>= 1;
            }
            write!(fmt, "\r\n")?;
        }
        self.draw.set(false);
        Ok(())
    }
}
