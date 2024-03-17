use crate::opcodes::Opcodes;
use std::u32;
// Registers are u8
// Memory addresses are u32
#[derive(Debug)]
enum Instruction {
    Jmp(JmpInstr),
    Goto(LabelJmpInstr),
    Math(MathInstr),
    Label(LabelInstr),
}
#[derive(Debug)]
struct JmpInstr {
    op_type: Opcodes,
    target: u32,
}
#[derive(Debug)]
struct LabelJmpInstr {
    op_type: Opcodes,
    target: LabelInstr,
}
#[derive(Debug)]
struct MathInstr {
    op_type: Opcodes,
    target: u8,
    src1: u8,
    src2: u8,
}
#[derive(Debug)]
struct LabelInstr {
    name: String,
}
#[derive(Debug)]
pub struct Assembler {
    startaddr: u32,
    instructions: Vec<Instruction>,
}
impl Assembler {
    pub fn new() -> Self {
        Self {
            startaddr: 0,
            instructions: vec![],
        }
    }
    pub fn assemble(&mut self, code: String) {
        for line in code.lines() {
            match line.trim_start().chars().next() {
                Some(character) => match character {
                    '#' => println!("Comment: {}", line),
                    '.' => {
                        println!("Directive: {}", line);
                        Self::directive(self, line.trim_start())
                    }
                    _ => {
                        println!("Code: {}", line);
                        Self::code(self, line.trim_start())
                    }
                },
                None => {
                    println!("Empty line.")
                }
            }
        }
    }
    fn directive(&mut self, line: &str) {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some(token) => match token {
                ".start" => {
                    let straddr = tokens.next().expect("No address for .start!");
                    let addr = u32::from_str_radix(
                        match straddr.strip_prefix("0x") {
                            Some(stripped) => stripped,
                            None => straddr,
                        },
                        16,
                    )
                    .expect("Error parsing address!");
                    self.startaddr = addr;
                }
                ".label" => {
                    let name = tokens.next().expect("No name for .label!");
                    self.instructions.push(Instruction::Label(LabelInstr {
                        name: String::from(name),
                    }))
                }
                _ => println!("Unknown directive: {}", token),
            },
            None => println!("Empty directive?"),
        }
    }
    fn code(&mut self, line: &str) {
        let mut op = line.splitn(2, ' ');
        let op_type = op
            .next()
            .expect("Empty lines should already be handled. THIS IS A BUG.");
        match op_type {
            "goto" => {
                let label = op.next().expect("No address for goto!");
                self.instructions.push(Instruction::Goto(LabelJmpInstr {
                    op_type: Opcodes::Goto,
                    target: LabelInstr {
                        name: String::from(label),
                    },
                }))
            }
            "jmp" => {
                let straddr = op.next().expect("No address for jmp!");
                let addr = u32::from_str_radix(
                    match straddr.strip_prefix("0x") {
                        Some(stripped) => stripped,
                        None => straddr,
                    },
                    16,
                )
                .expect("Error parsing address!");
                self.instructions.push(Instruction::Jmp(JmpInstr {
                    op_type: Opcodes::Goto,
                    target: addr,
                }));
            }

            _ => println!("Unknown instruction: {}", op_type),
        }
    }
}
