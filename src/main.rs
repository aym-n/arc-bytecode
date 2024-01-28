mod chunk;
use chunk::*;

mod vm;

mod value;

fn main() {
    let mut vm = vm::VM::new();
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_opcode(OpCode::OpConstant, 123);
    chunk.write(constant, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write_opcode(OpCode::OpConstant, 123);
    chunk.write(constant, 123);

    chunk.write_opcode(OpCode::OpAdd, 123);
    
    let constant = chunk.add_constant(5.6);
    chunk.write_opcode(OpCode::OpConstant, 123);
    chunk.write(constant, 123);

    chunk.write_opcode(OpCode::OpDivide, 123);

    chunk.write_opcode(OpCode::OpReturn, 123);

    vm.interpret(&chunk);

    chunk.free();
    vm.free();
}
