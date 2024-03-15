use std::u32;
pub struct Assembler {
    startaddr: u32,
}
impl Assembler {
    pub fn new() -> Self {
        Self { startaddr: 0 }
    }
    pub fn assemble(&mut self, code: String) {
        for line in code.lines() {
            match line.trim_start().chars().next() {
                Some(character) => match character {
                    '.' => Self::directive(self, line.trim_start()),
                    _ => Self::code(line.trim_start()),
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
                _ => println!("Unknown directive: {}", token),
            },
            None => println!("Empty directive?"),
        }
    }
    fn code(line: &str) {}
}
