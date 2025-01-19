use std::collections::HashMap;

use crate::cpu::AddressingMode;
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub addressing_mode: AddressingMode,
}

impl Instruction {
    pub fn new(
        opcode: u8,
        mnemonic: &'static str,
        bytes: u8,
        cycles: u8,
        addressing_mode: AddressingMode,
    ) -> Self {
        Self {
            opcode,
            mnemonic,
            bytes,
            cycles,
            addressing_mode,
        }
    }
}

lazy_static! {
    pub static ref CPU_INSTRUCTIONS: Vec<Instruction> = vec![
        // Access
        Instruction::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xBD, "LDA", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteX),
        Instruction::new(0xB9, "LDA", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteY),
        Instruction::new(0xA1, "LDA", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0xB1, "LDA", 2, 5 /* 6 if page crossed */, AddressingMode::IndirectY),

        Instruction::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x9D, "STA", 3, 5, AddressingMode::AbsoluteX),
        Instruction::new(0x99, "STA", 3, 5, AddressingMode::AbsoluteY),
        Instruction::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),

        Instruction::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPageY),
        Instruction::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xBE, "LDX", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteY),

        Instruction::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x96, "STX", 2, 4, AddressingMode::ZeroPageY),
        Instruction::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),

        Instruction::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xBC, "LDY", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteX),

        Instruction::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x94, "STY", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),

        // Transfer
        Instruction::new(0xAA, "TAX", 1, 2, AddressingMode::Implicit),

        Instruction::new(0x8A, "TXA", 1, 2, AddressingMode::Implicit),

        Instruction::new(0xA8, "TAY", 1, 2, AddressingMode::Implicit),

        Instruction::new(0x98, "TYA", 1, 2, AddressingMode::Implicit),

        // Arithmetic
        Instruction::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        Instruction::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x7D, "ADC", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteX),
        Instruction::new(0x79, "ADC", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteY),
        Instruction::new(0x61, "ADC", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0x71, "ADC", 2, 5 /* 6 if page crossed */, AddressingMode::IndirectY),

        Instruction::new(0xE9, "SBC", 2, 2, AddressingMode::Immediate),
        Instruction::new(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0xF5, "SBC", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0xED, "SBC", 3, 4, AddressingMode::Absolute),
        Instruction::new(0xFD, "SBC", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteX),
        Instruction::new(0xF9, "SBC", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteY),
        Instruction::new(0xE1, "SBC", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0xF1, "SBC", 2, 5 /* 6 if page crossed */, AddressingMode::IndirectY),

        Instruction::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage),
        Instruction::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPageX),
        Instruction::new(0xEE, "INC", 3, 6, AddressingMode::Absolute),
        Instruction::new(0xFE, "INC", 3, 7, AddressingMode::AbsoluteX),

        Instruction::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage),
        Instruction::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPageX),
        Instruction::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute),
        Instruction::new(0xDE, "DEC", 3, 7, AddressingMode::AbsoluteX),

        Instruction::new(0xCA, "DEX", 1, 2, AddressingMode::Implicit),

        Instruction::new(0xE8, "INX", 1, 2, AddressingMode::Implicit),

        Instruction::new(0xC8, "INY", 1, 2, AddressingMode::Implicit),

        Instruction::new(0x88, "DEY", 1, 2, AddressingMode::Implicit),

        //Shift
        Instruction::new(0x0A, "ASL", 1, 2, AddressingMode::Accumulator),
        Instruction::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
        Instruction::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPageX),
        Instruction::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute),
        Instruction::new(0x1E, "ASL", 3, 7, AddressingMode::AbsoluteX),

        Instruction::new(0x4A, "LSR", 1, 2, AddressingMode::Accumulator),
        Instruction::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage),
        Instruction::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPageX),
        Instruction::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute),
        Instruction::new(0x5E, "LSR", 3, 7, AddressingMode::AbsoluteX),

        Instruction::new(0x2A, "ROL", 1, 2, AddressingMode::Accumulator),
        Instruction::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),
        Instruction::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPageX),
        Instruction::new(0x2E, "ROL", 3, 6, AddressingMode::Absolute),
        Instruction::new(0x3E, "ROL", 3, 7, AddressingMode::AbsoluteX),

        Instruction::new(0x6A, "ROR", 1, 2, AddressingMode::Accumulator),
        Instruction::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),
        Instruction::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPageX),
        Instruction::new(0x6E, "ROR", 3, 6, AddressingMode::Absolute),
        Instruction::new(0x7E, "ROR", 3, 7, AddressingMode::AbsoluteX),

        // Bitwise
        Instruction::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
        Instruction::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x35, "AND", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x3D, "AND", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteX),
        Instruction::new(0x39, "AND", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteY),
        Instruction::new(0x21, "AND", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0x31, "AND", 2, 5 /* 6 if page crossed */, AddressingMode::IndirectY),

        Instruction::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
        Instruction::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0x0D, "ORA", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x1D, "ORA", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteX),
        Instruction::new(0x19, "ORA", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteY),
        Instruction::new(0x01, "ORA", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0x11, "ORA", 2, 5 /* 6 if page crossed */, AddressingMode::IndirectY),

        Instruction::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
        Instruction::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
        Instruction::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPageX),
        Instruction::new(0x4D, "EOR", 3, 4, AddressingMode::Absolute),
        Instruction::new(0x5D, "EOR", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteX),
        Instruction::new(0x59, "EOR", 3, 4 /* 5 if page crossed */, AddressingMode::AbsoluteY),
        Instruction::new(0x41, "EOR", 2, 6, AddressingMode::IndirectX),
        Instruction::new(0x51, "EOR", 2, 5 /* 6 if page crossed */, AddressingMode::IndirectY),

        // Jump
        Instruction::new(0x00, "BRK", 1, 7, AddressingMode::Implicit),
        // Instruction::new(0x00, "BRK", 2, 7, AddressingMode::Immediate),
    ];

    pub static ref INSTRUCTION_MAP: HashMap<u8, &'static Instruction> = {
        let mut instruction_map = HashMap::new();
        for instruction in CPU_INSTRUCTIONS.iter() {
            instruction_map.insert(instruction.opcode, instruction);
        }
        instruction_map
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_instruction() {
        assert_eq!(
            INSTRUCTION_MAP.get(&0xA9),
            Some(&&Instruction::new(
                0xA9,
                "LDA",
                2,
                2,
                AddressingMode::Immediate
            ))
        );
    }
}
