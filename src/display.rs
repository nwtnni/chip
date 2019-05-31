/// Width
pub const W: usize = 64;

/// Height
pub const H: usize = 32;

#[derive(Clone, Debug)]
pub struct Display {
    grid: [u64; H],
    draw: std::cell::Cell<bool>,
}

impl Display {
    pub fn clear(&mut self) {
        self.grid.iter_mut().for_each(|p| *p = 0);
        self.draw.set(true);
    }

    pub fn toggle(&mut self, x: usize, y: usize) -> u8 {
        let (x, y) = (x % W, y % H);
        let bit = 1 << (W - x);
        let hit = (self.grid[y] & bit) >> (W - x);
        self.grid[y] ^= bit;
        self.draw.set(true);
        hit as u8
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.draw.get() { return Ok(()) }
        write!(fmt, "{}", termion::clear::All)?;
        for row in &self.grid {
            let mut col = 1 << (W - 1);
            for _ in 0..W {
                write!(fmt, "{}", if row & col != 0 { "█" } else { " " })?;
                col >>= 1;
            }
            write!(fmt, "\n")?;
        }
        self.draw.set(false);
        Ok(())
    }
}
