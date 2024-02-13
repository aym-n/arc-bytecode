use std::ops::{Add, Sub, Div, Mul, Neg};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialOrd)]
pub enum Value{
    Boolean(bool),
    Number(f64),
    Nil,
}

impl Value {
    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Boolean(false) => true,
            _ => false,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            _ => panic!("Operands must be two numbers or two booleans."),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => panic!("Operands must be two numbers."),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => panic!("Operands must be two numbers."),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Value::Number(a) => Value::Number(-a),
            _ => panic!("Operand must be a number."),
        }
    }
    
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => panic!("Operands must be two numbers."),
        }
    }
    
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Number(n) => write!(f, "{n}"),
            Value::Nil => write!(f, "nil"),
        }
    }
}
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) -> usize {
        let index = self.values.len();
        self.values.push(value);
        index
    }

    pub fn free(&mut self) {
        self.values = Vec::new();
    }

    pub fn print_value(&self, index: usize) {
        print!("{}", self.values[index]);
    }
}
