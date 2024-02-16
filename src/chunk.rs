use crate::value::*;

pub enum OpCode {
    OpConstant,
    OpReturn,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpPrint,
    OpPop,
    OpDefineGlobal,
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn read(&self, offset: usize) -> u8 {
        self.code[offset]
    }

    pub fn write_opcode(&mut self, opcode: OpCode, line: usize) {
        self.code.push(opcode.into());
        self.lines.push(line);
    }

    pub fn free(&mut self) {
        self.code = Vec::new();
        self.constants.free();
    }

    pub fn add_constant(&mut self, value: Value) -> Option<u8> {
        let index = self.constants.write(value);
        u8::try_from(index).ok()
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match instruction {
            0 => self.constant_instruction("OpConstant", offset),
            1 => self.simple_instruction("OpReturn", offset),
            2 => self.simple_instruction("OpNegate", offset),
            3 => self.simple_instruction("OpAdd", offset),
            4 => self.simple_instruction("OpSubtract", offset),
            5 => self.simple_instruction("OpMultiply", offset),
            6 => self.simple_instruction("OpDivide", offset),
            7 => self.simple_instruction("OpNil", offset),
            8 => self.simple_instruction("OpTrue", offset),
            9 => self.simple_instruction("OpFalse", offset),
            10 => self.simple_instruction("OpNot", offset),
            11 => self.simple_instruction("OpEqual", offset),
            12 => self.simple_instruction("OpGreater", offset),
            13 => self.simple_instruction("OpLess", offset),
            14 => self.simple_instruction("OpPrint", offset),
            15 => self.simple_instruction("OpPop", offset),
            16 => self.constant_instruction("OpDefineGlobal", offset),
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
        self.constants.print_value(constant.into());
        println!("'");
        offset + 2
    }

    pub fn get_constant(&self, index: u8) -> Value {
        self.constants.values[index as usize].clone()
    }
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::OpConstant,
            1 => OpCode::OpReturn,
            2 => OpCode::OpNegate,
            3 => OpCode::OpAdd,
            4 => OpCode::OpSubtract,
            5 => OpCode::OpMultiply,
            6 => OpCode::OpDivide,
            7 => OpCode::OpNil,
            8 => OpCode::OpTrue,
            9 => OpCode::OpFalse,
            10 => OpCode::OpNot,
            11 => OpCode::OpEqual,
            12 => OpCode::OpGreater,
            13 => OpCode::OpLess,
            14 => OpCode::OpPrint,
            15 => OpCode::OpPop,
            16 => OpCode::OpDefineGlobal,
            _ => panic!("Unknown opcode {}", byte),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(opcode: OpCode) -> Self {
        opcode as u8
    }
}
