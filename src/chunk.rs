pub enum OpCode {
    OpReturn,
    Add,
}

pub struct Chunk {
    pub code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn free(&mut self) {
        self.code = Vec::new();
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
            0 => self.simple_instruction("OpReturn", offset),
            1 => self.simple_instruction("OpAdd", offset),
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
}