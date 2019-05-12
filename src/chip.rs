use crate::cpu;
use crate::asm;
use crate::ram;
use crate::stack;
use crate::io;

pub struct Chip {
    cpu: cpu::CPU,
    ram: ram::Mem,
    stack: stack::Mem,
}

impl Chip {
    fn step(&mut self) {

        let hi = self.ram[self.cpu.pc] as u16;
        let lo = self.ram[self.cpu.pc + 1] as u16;
        let op = asm::Asm::from(hi << 8 | lo);

        self.cpu.pc += 2;

        use asm::Asm::*;

        match op {
        | CLS => io::clear(),
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
        | _ => unimplemented!(),
        }
    }
}
