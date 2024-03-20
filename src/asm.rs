use crate::opcodes::Opcodes;
use std::{collections::HashMap, u32};
// Registers are u8
// Memory addresses are u32
#[derive(Debug)]
enum Instruction {
    Jmp(JmpInstr),
    Goto(GotoInstr),
    Op(OpInstr),
    Label(LabelInstr),
}
#[derive(Debug)]
struct JmpInstr {
    op_type: Opcodes,
    target: u32,
}
#[derive(Debug)]
struct GotoInstr {
    op_type: Opcodes,
    target: LabelInstr,
}
#[derive(Debug)]
struct OpInstr {
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
                    let straddr = tokens.next().expect("FATAL: No address for .start!");
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
        let op_type = op.next().unwrap();
        match op_type {
            "goto" => {
                let label = op.next().expect("FATAL: No label for goto!");
                self.instructions.push(Instruction::Goto(GotoInstr {
                    op_type: Opcodes::Jmp,
                    target: LabelInstr {
                        name: String::from(label),
                    },
                }))
            }
            "jmp" => {
                let straddr = op.next().expect("FATAL: No address for jmp!");
                let addr = u32::from_str_radix(
                    match straddr.strip_prefix("0x") {
                        Some(stripped) => stripped,
                        None => straddr,
                    },
                    16,
                )
                .expect("FATAL: Error parsing address for jmp!");
                self.instructions.push(Instruction::Jmp(JmpInstr {
                    op_type: Opcodes::Jmp,
                    target: addr,
                }));
            }

            _ => println!("Unknown instruction: {}", op_type),
        }
    }
    pub fn emit(self) -> Vec<u8> {
        let mut instructions: Vec<Vec<u8>> = vec![];
        let mut labels_srcs: HashMap<usize, String> = HashMap::new();
        let mut labels_targets: HashMap<String, usize> = HashMap::new();
        for instr in self.instructions {
            match instr {
                Instruction::Label(label_instr) => {
                    instructions.push(vec![]);
                    labels_targets.insert(label_instr.name, instructions.len());
                }
                Instruction::Op(op_instr) => instructions.push(vec![
                    op_instr.op_type as u8,
                    op_instr.target,
                    op_instr.src1,
                    op_instr.src2,
                ]),
                Instruction::Jmp(jmp_instr) => {
                    let mut code = vec![jmp_instr.op_type as u8];
                    code.append(&mut jmp_instr.target.to_le_bytes().to_vec());
                    instructions.push(code);
                }
                Instruction::Goto(goto_instr) => {
                    // push placeholder
                    instructions.push(vec![goto_instr.op_type as u8, 255, 255, 255, 255]);
                    // add index of goto to table
                    labels_srcs.insert(instructions.len() - 1, goto_instr.target.name);
                }
            }
        }
        // a bit ugly, but makes rust happy
        let mut mutations = Vec::new();
        // resolve gotos
        for (index, mut instr) in instructions.iter().enumerate() {
            // check for placeholder
            if instr == &vec![Opcodes::Jmp as u8, 255, 255, 255, 255] {
                let label_name = labels_srcs.get(&index);
                match label_name {
                    Some(label) => {
                        let label_target =
                            labels_targets.get(label).expect("FATAL: Unknown label!");
                        let mut code = vec![Opcodes::Jmp as u8];
                        let target_addr: u32 = instructions[..*label_target]
                            .iter()
                            .map(|v| v.len())
                            .sum::<usize>() as u32;
                        code.append(&mut target_addr.to_le_bytes().to_vec());
                        mutations.push((index, code));
                    } // if there is no jump target
                    None => println!("Why would you jump to 0xFFFFFFFF?"),
                }
            }
        }
        for (index, code) in mutations {
            instructions[index] = code;
        }
        println!("{:#?}", instructions);
        instructions.into_iter().flatten().collect()
    }
}
