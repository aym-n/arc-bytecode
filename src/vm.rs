use crate::chunk::*;
use crate::value::*;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>, // Stacks are implemented as Vecs in Rust
}

pub enum InterpretResult {
    Ok,
    // CompileError,
    // RuntimeError,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn free(&mut self) {
        self.chunk.free();
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.ip = 0;
        self.run(&chunk)
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                for slot in &self.stack {
                    print!("[ {} ]", slot);
                }
                println!();
                chunk.disassemble_instruction(self.ip);
            }

            let instruction = self.read_byte(chunk);
            match instruction {
                OpCode::OpReturn => {
                    println!("{}", self.stack.pop().unwrap());
                    return InterpretResult::Ok;
                }
                OpCode::OpConstant => {
                    let constant = self.read_constant(chunk);
                    self.stack.push(constant);
                }
            }
        }
    }

    fn read_byte(&mut self, chunk: &Chunk) -> OpCode {
        let value = chunk.read(self.ip);
        self.ip += 1;
        value.into()
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let value = chunk.read(self.ip);
        self.ip += 1;
        chunk.get_constant(value)
    }
}
