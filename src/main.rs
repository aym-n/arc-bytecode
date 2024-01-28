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

    chunk.write(OpCode::OpReturn as u8, 123);
    chunk.disassemble("test chunk");
    vm.interpret(&chunk);

    chunk.free();
    vm.free();
}
