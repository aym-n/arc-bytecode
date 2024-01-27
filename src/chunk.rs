use crate::value::*;

pub enum OpCode {
    OpConstant,
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<u8>,
    constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_opcode(&mut self, opcode: OpCode) {
        self.write(opcode as u8);
    }

    pub fn free(&mut self) {
        self.code = Vec::new();
        self.constants.free();
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        let index = self.constants.values.len() as u8;
        self.constants.write(value);
        index
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        let instruction = self.code[offset];
        match instruction {
            0 => self.constant_instruction("OpConstant", offset),
            1 => self.simple_instruction("OpReturn", offset),
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{:<16} {:4} '", name, constant);
        self.constants.print_value(constant as usize);
        println!("'");
        offset + 2
    }
}
