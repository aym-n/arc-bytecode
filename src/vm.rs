use crate::chunk;
use crate::chunk::*;
use crate::compiler::*;
use crate::value::*;
use std::collections::HashMap;
pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

macro_rules! BinaryOp {
    ($self:ident, $op:tt) => {
        if !($self.peek(0).is_number() && $self.peek(1).is_number()) {
            $self.runtime_error("Operands must be two numbers.");
            return InterpretResult::RuntimeError;
        }

        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();
        $self.stack.push(a $op b);
    };
}

macro_rules! BinaryCompOp {
    ($self:ident, $op:tt) => {
        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();
        $self.stack.push(Value::Boolean(a $op b));
    };
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn free(&mut self) {
        self.chunk.free();
        self.stack.clear();
        self.globals.clear();
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(&mut chunk);
        compiler.compile(source);
        self.ip = 0;
        self.run(&chunk)
    }

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack.len() - 1 - distance].clone()
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
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
                OpCode::OpDefineGlobal => {
                    let constant = self.read_constant().clone();
                    if let Value::Str(s) = constant {
                        let p = self.stack.pop().unwrap();
                        println!("Inserting {} into globals {}", s, p);
                        self.globals.insert(s, p.clone());
                    } else {
                        panic!("Unable to read constant from table");
                    }
                }

                OpCode::OpGetGlobal => {
                    let constant = self.read_constant().clone();
                    if let Value::Str(s) = constant {
                        if let Some(v) = self.globals.get(&s) {
                            self.stack.push(v.clone())
                        } else {
                            return InterpretResult::RuntimeError;
                        }
                    } else {
                        panic!("Unable to read constant from table");
                    }
                }

                OpCode::OpSetGlobal => {
                    let constant = self.read_constant().clone();
                    if let Value::Str(s) = constant {
                        let p = self.stack.pop().unwrap();
                        println!("Inserting {} into globals {}", s, p);
                        self.globals.insert(s, p.clone());
                    } else {
                        panic!("Unable to read constant from table");
                    }
                }
                
                OpCode::OpReturn => {
                    return InterpretResult::Ok;
                }
                OpCode::OpConstant => {
                    let constant = self.read_constant();
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
                    if self.peek(0).is_string() && self.peek(1).is_string() {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        let a = a.to_string();
                        let b = b.to_string();
                        self.stack.push(Value::Str(format!("{}{}", a, b)));
                    } else {
                        BinaryOp!(self, +);
                    }
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

                OpCode::OpEqual => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(Value::Boolean(a == b));
                }

                OpCode::OpGreater => {
                    BinaryCompOp!(self, >);
                }

                OpCode::OpLess => {
                    BinaryCompOp!(self, <);
                }

                OpCode::OpPrint => {
                    println!("{}", self.stack.pop().unwrap());
                }

                OpCode::OpPop => {
                    self.stack.pop();
                }
            }
        }
    }

    fn read_string(&mut self, chunk: &Chunk) -> String {
        self.read_constant().to_string()
    }

    fn read_byte(&mut self, chunk: &Chunk) -> OpCode {
        let value = chunk.read(self.ip);
        self.ip += 1;
        value.into()
    }

    fn read_constant(&mut self) -> Value {
        let value = self.chunk.read(self.ip) as usize;
        self.ip += 1;
        self.chunk.get_constant(value)
    }

    fn runtime_error(&mut self, message: &str) {
        println!("{}", message);
        let instruction = self.ip - 1;
        let line = self.chunk.lines[instruction];
        println!("[line {}] in script", line);

        self.stack.clear();
    }
}
