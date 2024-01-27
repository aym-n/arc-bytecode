mod chunk;
use chunk::*;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn as u8);
    chunk.disassemble("test chunk");
    chunk.free();
}
