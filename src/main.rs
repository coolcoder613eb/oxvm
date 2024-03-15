use std::env::args;
use std::fs::read_to_string;
mod asm;
use asm::Assembler;
mod opcodes;
/*
 * oxvm asm prog.asm prog.bin
 * oxvm prog.bin
 */

fn main() {
    let mut arguments = args();
    arguments.next();
    match arguments.next() {
        Some(filename) => {
            println!("Filename: {}", filename);
            let mut asmb = Assembler::new();
            asmb.assemble(read_to_string(filename).expect("Failed to read file!"));
        }
        None => println!("No file specified"),
    }
}
