use std::io::{Read, stdin};

#[derive(Debug)]
pub struct Interpreter {
    data_pointer: usize,
    data: Vec<u8>,
    program_pointer: usize,
    pub program: Vec<Instruction>,
}

impl Interpreter {
    const VALID_CHARS: &[char] = &['>', '<', '+', '-', '.', ',', '[', ']'];

    pub fn new(program: &str) -> Result<Self, String> {
        let clean_program = Self::clean_program(program);
        if !Self::check_valid(&clean_program) {
            return Err("Invalid program".to_string());
        }
        let program = Self::compile_program(clean_program);
        Ok(Self {
            data_pointer: 0,
            data: vec![0; 30000],
            program_pointer: 0,
            program,
        })
    }
    pub fn compile_program(program: String) -> Vec<Instruction> {
        let mut i = 0;
        let mut insts = Vec::with_capacity(program.len());
        while i < program.len() {
            let inst = match program.get(i..(i + 1)).unwrap() {
                ">" => {
                    let mut c = 1isize;
                    for j in (i + 1)..program.len() {
                        if program.get(j..(j + 1)).unwrap() == ">" {
                            c += 1;
                        } else {
                            break;
                        }
                    }
                    i += c as usize;
                    Instruction::INCP_C(c)
                }
                "<" => {
                    let mut c = 1isize;
                    for j in (i + 1)..program.len() {
                        if program.get(j..(j + 1)).unwrap() == "<" {
                            c += 1;
                        } else {
                            break;
                        }
                    }
                    i += c as usize;
                    Instruction::INCP_C(-c)
                }
                "+" => {
                    let mut c = 1i16;
                    for j in (i + 1)..program.len() {
                        if program.get(j..(j + 1)).unwrap() == "+" {
                            c += 1;
                        } else {
                            break;
                        }
                    }
                    i += c as usize;
                    Instruction::INCV_C(c)
                }
                "-" => {
                    let mut c = 1i16;
                    for j in (i + 1)..program.len() {
                        if program.get(j..(j + 1)).unwrap() == "-" {
                            c += 1;
                        } else {
                            break;
                        }
                    }
                    i += c as usize;
                    Instruction::INCV_C(-c)
                }
                "." => {
                    i += 1;
                    Instruction::PRNT
                }
                "," => {
                    i += 1;
                    Instruction::INPT
                }
                "[" => {
                    i += 1;
                    Instruction::JMPF
                }
                "]" => {
                    i += 1;
                    Instruction::JMPB
                }
                _ => {
                    i += 1;
                    Instruction::NOOP
                }
            };
            insts.push(inst);
        }
        let mut final_inst = Vec::with_capacity(insts.len());
        for i in 0..insts.len() {
            if insts[i] == Instruction::JMPF {
                let mut j = i + 1;
                let mut d = 0;
                while let Some(cur_inst) = insts.get(j) {
                    if d == 0 && cur_inst == &Instruction::JMPB {
                        final_inst.push(Instruction::JUMP_C(j));
                        break;
                    } else if cur_inst == &Instruction::JMPB {
                        d -= 1;
                    } else if cur_inst == &Instruction::JMPF {
                        d += 1;
                    }
                    j += 1;
                }
            } else if insts[i] == Instruction::JMPB {
                let mut j = i - 1;
                let mut d = 0;
                while let Some(cur_inst) = insts.get(j) {
                    if d == 0 && cur_inst == &Instruction::JMPF {
                        final_inst.push(Instruction::JUMP_C(j));
                        break;
                    } else if cur_inst == &Instruction::JMPB {
                        d += 1;
                    } else if cur_inst == &Instruction::JMPF {
                        d -= 1;
                    }
                    j -= 1;
                }
            } else {
                final_inst.push(insts[i]);
            }
        }
        final_inst
    }
    fn clean_program(program: &str) -> String {
        let mut clean = String::new();
        for c in program.chars() {
            if Self::VALID_CHARS.contains(&c) {
                clean.push(c);
            }
        }
        clean
    }

    fn check_valid(program: &str) -> bool {
        let mut depth = 0;
        for c in program.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                _ => {}
            }
            if depth < 0 {
                return false;
            }
        }
        depth == 0
    }
    pub fn run(&mut self) -> Result<(), String> {
        loop {
            if self.program_pointer >= self.program.len() {
                break;
            }
            let instruction = self.program[self.program_pointer];
            match instruction {
                Instruction::PRNT => {
                    print!("{}", self.data[self.data_pointer] as char)
                }
                Instruction::INPT => {
                    let mut buf = vec![0];
                    stdin().read_exact(&mut buf).unwrap();
                    self.data[self.data_pointer] = buf[0];
                }
                Instruction::INCP_C(p) => {
                    self.data_pointer = (self.data_pointer as isize + p) as usize;
                }
                Instruction::INCV_C(p) => {
                    self.data[self.data_pointer] =
                        ((self.data[self.data_pointer] as i16 + 256 + p) % 256) as u8;
                }

                Instruction::JUMP_C(p) => {
                    if (p < self.program_pointer && self.data[self.data_pointer] != 0)
                        || (p > self.program_pointer && self.data[self.data_pointer] == 0)
                    {
                        self.program_pointer = p;
                    }
                }
                Instruction::NOOP => {}
                _ => {}
            }

            self.program_pointer += 1;
        }
        Ok(())
    }
}
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    /// No Op
    NOOP,

    /// Increment the data pointer by the value
    INCP_C(isize),

    /// Increment the value at the data pointer by the value
    INCV_C(i16),

    /// Print the value of the current cell ('.')
    PRNT,

    /// Take 1 byte from input and store in the current cell (',')
    INPT,

    /// Jump to position
    JUMP_C(usize),

    // Used for setting up jumps in compile
    JMPF,
    JMPB,
}
