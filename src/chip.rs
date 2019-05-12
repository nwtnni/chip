use crate::cpu;
use crate::asm;
use crate::ram;
use crate::stack;

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

        match op {
        | _ => unimplemented!(),
        }
    }
}
