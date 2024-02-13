use crate::chunk::*;
use crate::compiler::*;
use crate::value::*;
pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

macro_rules! BinaryOp {
    ($self:ident, $op:tt) => {
        if !($self.peek(0).is_number() && $self.peek(1).is_number()) {
            $self.runtime_error("Operands must be numbers.");
            return InterpretResult::RuntimeError;
        }
        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();
        $self.stack.push(a $op b);
    };
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

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(&mut chunk);
        compiler.compile(source);
        self.ip = 0;
        self.run(&chunk)
    }

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack.len() - 1 - distance]
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
                    println!("{}", self.stack.pop().unwrap_or(Value::Number(0f64)));
                    return InterpretResult::Ok;
                }
                OpCode::OpConstant => {
                    let constant = self.read_constant(chunk);
                    self.stack.push(constant);
                }
                OpCode::OpNegate => {
                    if !self.peek(0).is_number() {
                        self.runtime_error("Operand must be a number.");
                        return InterpretResult::RuntimeError;
                    }

                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
                OpCode::OpAdd => {
                    BinaryOp!(self, +);
                }

                OpCode::OpSubtract => {
                    BinaryOp!(self, -);
                }

                OpCode::OpMultiply => {
                    BinaryOp!(self, *);
                }

                OpCode::OpDivide => {
                    BinaryOp!(self, /);
                }

                OpCode::OpNil => self.stack.push(Value::Nil),

                OpCode::OpTrue => self.stack.push(Value::Boolean(true)),

                OpCode::OpFalse => self.stack.push(Value::Boolean(false)),

                OpCode::OpNot => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(value.is_falsey()));
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

    fn runtime_error(&mut self, message: &str) {
        println!("{}", message);
        let instruction = self.ip - 1;
        let line = self.chunk.lines[instruction];
        println!("[line {}] in script", line);

        self.stack.clear();
    }
}
