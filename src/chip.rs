use crate::cpu;
use crate::asm;
use crate::ram;
use crate::stack;
use crate::display;

pub struct Chip {
    cpu: cpu::CPU,
    ram: ram::Mem,
    stack: stack::Mem,
    display: display::Display,
}

impl Chip {
    fn step(&mut self) {

        let hi = self.ram[self.cpu.pc] as u16;
        let lo = self.ram[self.cpu.pc + 1] as u16;
        let op = asm::Asm::from(hi << 8 | lo);

        self.cpu.pc += 2;

        use asm::Asm::*;

        match op {
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
            unimplemented!() 
        }
        | SKNP(x) => {
            unimplemented!() 
        }
        | LDTR(x) => {
            self.cpu[x] = self.cpu.dt;
        }
        | LDK(x) => {
            unimplemented!() 
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
        | _ => unimplemented!(),
        }
    }
}

impl std::fmt::Display for Chip {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.display)
    }
}
