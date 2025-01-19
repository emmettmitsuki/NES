pub mod instructions;

use bitflags::bitflags;
use instructions::INSTRUCTION_MAP;

const MEMORY_SIZE: usize = 2048;

const PROGRAM_START_ADDRESS: usize = 0x8000;
const PROGRAM_COUNTER_RESET_ADDRESS: u16 = 0xFFFC;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
}

pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    status: u8,
    sp: u8,
    pc: u16,

    memory: [u8; 0xFFFF],
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct StatusFlags: u8 {
        const Carry            = 0b0000_0001;
        const Zero             = 0b0000_0010;
        const InterruptDisable = 0b0000_0100;
        const Decimal          = 0b0000_1000;
        //
        //
        const Overflow         = 0b0100_0000;
        const Negative         = 0b1000_0000;
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            status: 0,
            sp: 0,
            pc: 0,

            memory: [0; 0xFFFF],
        }
    }

    // Access

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.mem_read(addr);

        self.a = value;
        self.update_zero_and_negative_flags(self.a);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_address(mode);
        self.mem_write(addr, self.a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.mem_read(addr);

        self.x = value;
        self.update_zero_and_negative_flags(self.x);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_address(mode);
        self.mem_write(addr, self.x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.mem_read(addr);

        self.y = value;
        self.update_zero_and_negative_flags(self.y);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_address(mode);
        self.mem_write(addr, self.y);
    }

    // Transfer

    fn tax(&mut self) {
        self.x = self.a;
        self.update_zero_and_negative_flags(self.x);
    }

    fn txa(&mut self) {
        self.a = self.x;
        self.update_zero_and_negative_flags(self.a);
    }

    fn tay(&mut self) {
        self.y = self.a;
        self.update_zero_and_negative_flags(self.y);
    }

    fn tya(&mut self) {
        self.a = self.y;
        self.update_zero_and_negative_flags(self.a);
    }

    // Arithmetic

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.mem_read(addr);
        let carry_flag = self.get_carry_flag();

        let (result, overflow) = {
            let (res, ovf1) = self.a.overflowing_add(value);
            let (res, ovf2) = res.overflowing_add(carry_flag);
            (res, ovf1 || ovf2)
        };

        let overflow_flag_value = (result ^ self.a) & (result ^ value) & 0x80;

        self.a = result;

        self.update_overflow_flag(overflow_flag_value);
        self.update_carry_flag(overflow);
        self.update_zero_and_negative_flags(self.a);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        todo!()
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.mem_read(addr);

        let result = value.wrapping_add(1);
        self.mem_write(addr, result);

        self.update_zero_and_negative_flags(result);
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_address(mode);
        let value = self.mem_read(addr);

        let result = value.wrapping_sub(1);
        self.mem_write(addr, result);

        self.update_zero_and_negative_flags(result);
    }

    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.x);
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.x);
    }

    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.y);
    }

    // Other

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.status |= StatusFlags::Zero.bits();
        } else {
            self.status &= !StatusFlags::Zero.bits();
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        if result & StatusFlags::Negative.bits() != 0 {
            self.status |= StatusFlags::Negative.bits();
        } else {
            self.status &= !StatusFlags::Negative.bits();
        }
    }

    fn get_overflow_flag(&self) -> u8 {
        (self.status & StatusFlags::Overflow.bits()) >> 6
    }

    fn get_carry_flag(&self) -> u8 {
        self.status & StatusFlags::Carry.bits()
    }

    // sets overflow flag to 0 if value == 0, else 1
    fn update_overflow_flag(&mut self, value: u8) {
        if value != 0 {
            self.status |= StatusFlags::Overflow.bits();
        } else {
            self.status &= !StatusFlags::Overflow.bits();
        }
    }

    // sets carry flag if overflow occured
    fn update_carry_flag(&mut self, overflow: bool) {
        if overflow {
            self.status |= StatusFlags::Carry.bits();
        } else {
            self.status &= !StatusFlags::Carry.bits();
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;

        (hi << 8) | lo
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[PROGRAM_START_ADDRESS..(PROGRAM_START_ADDRESS + program.len())]
            .copy_from_slice(&program);
        self.mem_write_u16(PROGRAM_COUNTER_RESET_ADDRESS, PROGRAM_START_ADDRESS as u16);
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        // TODO: change reset value
        self.status = 0;

        // TODO: self.stack -= 3;

        self.pc = self.mem_read_u16(PROGRAM_COUNTER_RESET_ADDRESS);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.mem_read(self.pc);
            self.pc += 1;

            let instruction = INSTRUCTION_MAP.get(&opcode).unwrap();

            match opcode {
                // Access
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&instruction.addressing_mode)
                }
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&instruction.addressing_mode)
                }
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&instruction.addressing_mode),
                0x86 | 0x96 | 0x8E => self.stx(&instruction.addressing_mode),
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&instruction.addressing_mode),
                0x84 | 0x94 | 0x8C => self.sty(&instruction.addressing_mode),

                // Transfer
                0xAA => self.tax(),
                0x8A => self.txa(),
                0xA8 => self.tay(),
                0x98 => self.tya(),

                // Arithmetic
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&instruction.addressing_mode);
                }
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    self.sbc(&instruction.addressing_mode);
                }
                0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(&instruction.addressing_mode),
                0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&instruction.addressing_mode),
                0xCA => self.dex(),
                0xE8 => self.inx(),
                0xC8 => self.iny(),

                // Jump
                0x00 => return,
                _ => panic!("opcode '{:X}' not recognised", opcode),
            }
            self.pc += (instruction.bytes - 1) as u16;
        }
    }

    fn get_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Implicit => todo!(),
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage => self.mem_read(self.pc) as u16,
            AddressingMode::ZeroPageX => {
                let arg = self.mem_read(self.pc);
                let addr = arg.wrapping_add(self.x) as u16;
                addr
            }
            AddressingMode::ZeroPageY => {
                let arg = self.mem_read(self.pc);
                let addr = arg.wrapping_add(self.y) as u16;
                addr
            }
            AddressingMode::Absolute => self.mem_read_u16(self.pc),
            AddressingMode::AbsoluteX => {
                let arg = self.mem_read_u16(self.pc);
                let addr = arg.wrapping_add(self.x as u16);
                addr
            }
            AddressingMode::AbsoluteY => {
                let arg = self.mem_read_u16(self.pc);
                let addr = arg.wrapping_add(self.y as u16);
                addr
            }
            AddressingMode::Relative => {
                // TODO: test
                let offset = self.mem_read(self.pc) as u16;
                self.pc + offset
            }
            AddressingMode::Indirect => {
                // TODO: test
                let addr = self.mem_read_u16(self.pc);
                addr
            }
            AddressingMode::IndirectX => {
                let addr = self.mem_read(self.pc).wrapping_add(self.x);
                let lo = self.mem_read(addr as u16);
                let hi = self.mem_read(addr.wrapping_add(1) as u16);

                (hi as u16) << 8 | lo as u16
            }
            AddressingMode::IndirectY => {
                let addr = self.mem_read(self.pc);
                let lo = self.mem_read(addr as u16);
                let hi = self.mem_read(addr.wrapping_add(1) as u16);
                let deref = (hi as u16) << 8 | lo as u16;
                let result = deref.wrapping_add(self.y as u16);
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod instructions {
        use super::*;

        mod access {
            use super::*;

            #[test]
            fn test_0xa9_lda_immediate() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA9, 0x05, 0x00]);
                assert_eq!(cpu.a, 0x05);
                assert_eq!(cpu.status & StatusFlags::Zero.bits(), 0b00);
                assert_eq!(cpu.status & StatusFlags::Negative.bits(), 0b0);
            }

            #[test]
            fn test_0xa9_lda_zero_flag() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
                assert_eq!(cpu.status & StatusFlags::Zero.bits(), 0b10)
            }

            #[test]
            fn test_0xa5_lda_from_memory() {
                let mut cpu = Cpu::new();
                cpu.mem_write(0x10, 0x55);
                cpu.load_and_run(vec![0xA5, 0x10, 0x00]);
                assert_eq!(cpu.a, 0x55);
            }

            #[test]
            fn test_0xa2_ldx_immediate() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA2, 0x10, 0x00]);
                assert_eq!(cpu.x, 0x10);
            }

            #[test]
            fn test_0xa6_ldx_from_memory() {
                let mut cpu = Cpu::new();
                cpu.mem_write(0x11, 0xAB);
                cpu.load_and_run(vec![0xA6, 0x11, 0x00]);
                assert_eq!(cpu.x, 0xAB);
            }

            #[test]
            fn test_0x86_stx() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA2, 0xFF, 0x86, 0x10, 0x00]);
                assert_eq!(cpu.mem_read(0x10), 0xFF);
            }

            #[test]
            fn test_0xa0_ldy_immediate() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA0, 0x13, 0x00]);
                assert_eq!(cpu.y, 0x13);
            }

            #[test]
            fn test_0xa4_ldy_from_memory() {
                let mut cpu = Cpu::new();
                cpu.mem_write(0x03, 0x1F);
                cpu.load_and_run(vec![0xA4, 0x03, 0x00]);
                assert_eq!(cpu.y, 0x1F);
            }

            #[test]
            fn test_0x84_sty() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA0, 0x44, 0x84, 0x01, 0x00]);
                assert_eq!(cpu.mem_read(0x01), 0x44);
            }
        }

        mod transfer {
            use super::*;

            #[test]
            fn test_0xaa_tax() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA9, 0x0A, 0xAA, 0x00]);
                assert_eq!(cpu.x, 0x0A);
            }

            #[test]
            fn test_0x8a_txa() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA2, 0x12, 0x8A, 0x00]);
                assert_eq!(cpu.a, 0x12);
            }

            #[test]
            fn test_0xa8_tay() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA9, 0x01, 0xA8, 0x00]);
                assert_eq!(cpu.y, 0x01);
            }

            #[test]
            fn test_0x98_tya() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA0, 0xAD, 0x98, 0x00]);
                assert_eq!(cpu.a, 0xAD);
            }
        }

        mod arithmetic {
            use super::*;

            #[test]
            fn test_0x69_adc() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x69, 0xC4, 0x00]);
                assert_eq!(cpu.a, 0x84);
            }

            #[test]
            fn test_0x69_adc_carry_flag() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA9, 0xFF, 0x69, 0x01, 0x00]);
                assert_eq!(cpu.a, 0x00);
                assert_eq!(cpu.get_carry_flag(), 1);
            }

            #[test]
            fn test_0x69_adc_overflow_flag() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA9, 0x50, 0x69, 0x50, 0x00]);
                assert_eq!(cpu.a, 0xA0);
                assert_eq!(cpu.get_overflow_flag(), 1);
            }

            #[test]
            fn test_0xe6_inc() {
                let mut cpu = Cpu::new();
                cpu.mem_write(0x10, 0x35);
                cpu.load_and_run(vec![0xE6, 0x10, 0x00]);
                assert_eq!(cpu.mem_read(0x10), 0x36);
            }

            #[test]
            fn test_0xc6_dec() {
                let mut cpu = Cpu::new();
                cpu.mem_write(0x12, 0xEF);
                cpu.load_and_run(vec![0xC6, 0x12, 0x00]);
                assert_eq!(cpu.mem_read(0x12), 0xEE);
            }

            #[test]
            fn test_0xca_dex() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA2, 0x13, 0xCA, 0x00]);
                assert_eq!(cpu.x, 0x12);
            }

            #[test]
            fn test_0xe8_inx_overflow() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA9, 0xFF, 0xAA, 0xE8, 0xE8, 0x00]);

                assert_eq!(cpu.x, 1);
            }

            #[test]
            fn test_0xc8_iny() {
                let mut cpu = Cpu::new();
                cpu.load_and_run(vec![0xA0, 0x01, 0xC8, 0x00]);
                assert_eq!(cpu.y, 0x02);
            }
        }

        #[test]
        fn test_5_ops_0xa9_0xaa_0xe8_0x00() {
            let mut cpu = Cpu::new();
            cpu.x = 0xFF;
            cpu.load_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);
            assert_eq!(cpu.x, 0xC1);
        }
    }
}
