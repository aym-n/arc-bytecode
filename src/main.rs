mod chunk;
use chunk::*;

mod value;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_opcode(OpCode::OpConstant);
    chunk.write(constant);

    chunk.write(OpCode::OpReturn as u8);
    chunk.disassemble("test chunk");
    chunk.free();
}
