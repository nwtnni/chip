use termion::cursor;
use termion::event;

use crate::cpu;
use crate::asm;
use crate::ram;
use crate::stack;
use crate::display;

/// Main emulator logic.
pub struct Chip {
    /// CHIP-8 CPU
    cpu: cpu::CPU,

    /// Random access memory
    ram: ram::Mem,

    /// Last-pressed key
    key: Option<u8>,

    /// Stack memory
    stack: stack::Mem,

    /// Terminal-backed display
    display: display::Display,
}

impl Chip {
    /// Create a new emulator running `program`.
    pub fn new<B>(program: B) -> Self where B: IntoIterator<Item = u8> {
        Chip {
            cpu: cpu::CPU::default(),
            ram: ram::Mem::new(program),
            key: None,
            stack: stack::Mem::default(),
            display: display::Display::default(),
        }
    }

    /// Decrement the sound and delay timers.
    pub fn tick(&mut self) {
        self.cpu.st = self.cpu.st.saturating_sub(1);
        self.cpu.dt = self.cpu.dt.saturating_sub(1);
    }

    /// Update the latest pressed key.
    pub fn set_key(&mut self, event: event::Key) {
        use event::Key::*;
        self.key = match event {
        | Char('1') => Some(0x01),
        | Char('2') => Some(0x02),
        | Char('3') => Some(0x03),
        | Char('4') => Some(0x0C),
        | Char('q') => Some(0x04),
        | Char('w') => Some(0x05),
        | Char('e') => Some(0x06),
        | Char('r') => Some(0x0D),
        | Char('a') => Some(0x07),
        | Char('s') => Some(0x08),
        | Char('d') => Some(0x09),
        | Char('f') => Some(0x0E),
        | Char('z') => Some(0x0A),
        | Char('x') => Some(0x00),
        | Char('c') => Some(0x0B),
        | Char('v') => Some(0x0F),
        | _ => None,
        };
    }

    /// Execute a single CPU cycle.
    pub fn step(&mut self) {

        let hi = self.ram[self.cpu.pc] as u16;
        let lo = self.ram[self.cpu.pc + 1] as u16;
        let op = asm::Asm::from(hi << 8 | lo);

        self.cpu.pc += 2;

        use asm::Asm::*;

        match op {
        | SYS(_) => (),
        | CLS => {
            self.display.clear();
        }
        | RET => {
            self.cpu.sp.dec();
            self.cpu.pc = self.stack[self.cpu.sp];
        }
        | JP(addr) => {
            self.cpu.pc = addr;
        }
        | CALL(addr) => {
            self.stack[self.cpu.sp] = self.cpu.pc;
            self.cpu.sp.inc();
            self.cpu.pc = addr;
        }
        | SEC(x, kk) => {
            if self.cpu[x] == kk { self.cpu.pc += 2; }
        }
        | SNEC(x, kk) => {
            if self.cpu[x] != kk { self.cpu.pc += 2; }
        }
        | SER(x, y) => {
            if self.cpu[x] == self.cpu[y] { self.cpu.pc += 2; }
        }
        | SNER(x, y) => {
            if self.cpu[x] != self.cpu[y] { self.cpu.pc += 2; }
        }
        | LDC(x, kk) => {
            self.cpu[x] = kk;
        }
        | ADDC(x, kk) => {
            self.cpu[x] += kk;;
        }
        | LDR(x, y) => {
            self.cpu[x] = self.cpu[y];
        }
        | OR(x, y) => {
            self.cpu[x] |= self.cpu[y];
        }
        | AND(x, y) => {
            self.cpu[x] &= self.cpu[y];
        }
        | XOR(x, y) => {
            self.cpu[x] ^= self.cpu[y];
        }
        | ADDR(x, y) => {
            let z = self.cpu[x] as u16 + self.cpu[y] as u16;
            self.cpu[cpu::VF] = if z > 0xFF { 1 } else { 0 };
            self.cpu[x] = z as u8;
        }
        | SUB(x, y) => {
            let vx = self.cpu[x];
            let vy = self.cpu[y];
            self.cpu[cpu::VF] = if vx > vy { 1 } else { 0 };
            self.cpu[x] = vx.wrapping_sub(vy);
        }
        | SHR(x) => {
            let vx = self.cpu[x];
            self.cpu[cpu::VF] = vx & 0x01;
            self.cpu[x] = vx >> 0x01;
        }
        | SUBN(x, y) => {
            let vx = self.cpu[x];
            let vy = self.cpu[y];
            self.cpu[cpu::VF] = if vy > vx { 1 } else { 0 };
            self.cpu[x] = vy.wrapping_sub(vx);
        }
        | SHL(x) => {
            let vx = self.cpu[x];
            self.cpu[cpu::VF] = if vx & 0x80 != 0x00 { 1 } else { 0 };
            self.cpu[x] = vx << 0x01;
        }
        | LDI(addr) => {
            self.cpu.idx = addr;
        }
        | JO(addr) => {
            self.cpu.pc = addr + self.cpu[cpu::V0] as u16;
        }
        | RND(x, kk) => {
            self.cpu[x] = rand::random::<u8>() & kk;
        }
        | DRW(x, y, n) => {
            self.cpu[cpu::VF] = 0;
            let vx = self.cpu[x];
            let vy = self.cpu[y];
            for dy in 0..n {
                let mut line = self.ram[self.cpu.idx + dy as u16];
                for dx in 0..8 {
                    if line & 0x80 > 0 { self.cpu[cpu::VF] |= self.display.toggle(vx + dx, vy + dy); }
                    line <<= 1;
                }
            }
        }
        | SKP(x) => {
            match self.key.take() {
            | Some(k) if k == self.cpu[x] => { self.cpu.pc += 2; },
            | _ => (),
            }
        }
        | SKNP(x) => {
            match self.key.take() {
            | None => { self.cpu.pc += 2; },
            | Some(k) if k != self.cpu[x] => { self.cpu.pc += 2; },
            | _ => (),
            }
        }
        | LDTR(x) => {
            self.cpu[x] = self.cpu.dt;
        }
        | LDK(x) => {
            match self.key.take() {
            | None => { self.cpu.pc -= 2; }
            | Some(k) => { self.cpu[x] = k },
            }
        }
        | LDRT(x) => {
            self.cpu.dt = self.cpu[x];
        }
        | LDRS(x) => {
            self.cpu.st = self.cpu[x];
        }
        | ADDI(x) => {
            self.cpu.idx += self.cpu[x] as u16;
        }
        | LDS(x) => {
            self.cpu.idx = ram::FONT_OFFSET + (self.cpu[x] as u16 * 5);
        }
        | LDB(x) => {
            let vx = self.cpu[x];
            self.ram[self.cpu.idx + 0] = (vx / 100) % 10;
            self.ram[self.cpu.idx + 1] = (vx / 10) % 10;
            self.ram[self.cpu.idx + 2] = (vx / 1) % 10;
        }
        | WR(x) => {
            for offset in 0..=x {
                self.ram[self.cpu.idx + offset as u16] = self.cpu[cpu::V0 + offset]; 
            }
            self.cpu.idx += x as u16 + 1;
        }
        | RD(x) => {
            for offset in 0..=x {
                self.cpu[cpu::V0 + offset] = self.ram[self.cpu.idx + offset as u16];
            }
            self.cpu.idx += x as u16 + 1;
        }
        };
    }

    /// Draw the current display.
    pub fn draw<W: std::io::Write>(&mut self, dx: u16, dy: u16, out: &mut W) -> std::io::Result<()> {
        self.display.draw(dx, dy, out)?;

        let dy = dy + display::H as u16 + 1;

        for offset in 0x0..=0xF {
            if offset % 4 == 0 {
                let dx = dx + 4;
                let dy = dy + (offset as u16 / 2); 
                write!(out, "{}", cursor::Goto(dx, dy))?;
            }
            let x = cpu::V0 + offset;
            write!(out, "{}: {:#04X}        ", x, self.cpu[x])?;
        }

        let dy = dy + 8;

        write!(
            out,
            "{}   PC: {}    SP: {}    ST: {:#04X}    DT: {:#04X}    I: {}",
            cursor::Goto(dx, dy),
            self.cpu.pc,
            self.cpu.sp,
            self.cpu.st,
            self.cpu.dt,
            self.cpu.idx
        )?;

        Ok(())
    }
}
