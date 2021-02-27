use std::{collections::BTreeMap, fmt::{self, Debug, Formatter}};
use crate::vm::*;
pub struct Compiler {
    output: Vec<u8>,
    input: String,
    labels: BTreeMap<String, u16>,
    errors: CompilerErrors,
}
#[derive(Clone)]
pub struct CompilerErrors(Vec<String>);

impl Compiler {
    pub fn new<T>(input: T) -> Self
    where T: ToString {
        Self {
            output: vec![],
            input: input.to_string(),
            labels: BTreeMap::new(),
            errors: CompilerErrors(vec![]),
        }
    }

    fn add_error(&mut self, line: usize, message: &str) {
        let cloned = self.input.clone();
        self.errors.0.push(
            format!("{} | {}\nError: {}", line + 1, cloned.lines().nth(line).unwrap(), message)
        )
    }

    fn bi_reg(&mut self, l: usize, splited: &Vec<&str>, current: &mut usize, name: &str, opcode: u8, custom: Option<&str>) {
        if splited.len() < 3 {
            self.add_error(l, &format!("Usage: {} <register_a> <register_b>.", name));
        } else {
            *current += 1;
            let lhs = match parse_number(splited[*current]) {
                Some(u) => u,
                None => {
                    self.add_error(l, "Invalid register, expected a natural number between 0 and 31");
                    *current += 1;
                    *current += 1;
                    return;
                }
            } as u8;
            *current += 1;
            let rhs = match parse_number(splited[*current]) {
                Some(u) => u,
                None => {
                    if custom.is_none() {
                        self.add_error(l, "Invalid register, expected a natural number between 0 and 31");
                    } else {
                        self.add_error(l, &format!("Invalid {}, expected a natural number between 0 and 255", custom.unwrap()));
                    }
                    *current += 1;
                    return;
                }
            } as u8;
            *current += 1;

            self.output.push(opcode);
            self.output.push(lhs);
            self.output.push(rhs);
        }
    }

    fn bi_sixteen(&mut self, l: usize, splited: &Vec<&str>, current: &mut usize, name: &str, opcode: u8) {
        if splited.len() < 3 {
            self.add_error(l, &format!("Usage: {} <register> <value>.", name));
        } else {
            *current += 1;
            let register = match parse_number(splited[*current]) {
                Some(u) => u,
                None => {
                    self.add_error(l, "Invalid register, expected a natural number between 0 and 31");
                    *current += 1;
                    *current += 1;
                    return;
                }
            } as u8;
            *current += 1;
            let value = match parse_number(splited[*current]) {
                Some(u) => u,
                None => {
                    self.add_error(l, "Invalid value, expected a natural number between 0 and 65535");
                    *current += 1;
                    return;
                }
            };
            *current += 1;

            self.output.push(opcode);
            self.output.push(register);
            self.output.push((value >> 8) as u8);
            self.output.push((value & 0xFF) as u8);
        }
    }

    fn jmps(&mut self, l: usize, splited: &Vec<&str>, current: &mut usize, name: &str, opcode: u8) {
        if splited.len() < 2 {
            self.add_error(l, &format!("Usage: {} <value>.", name));
        } else {
            *current += 1;
            let value = if splited[*current].starts_with(":") {

                if self.labels.contains_key(splited[*current]) {
                    self.labels[splited[*current]]
                } else {
                    self.add_error(l, &format!("`{}` is not a valid label.", splited[*current]));
                    *current += 1;
                    return;
                }
            } else {
                match parse_number(splited[*current]) {
                    Some(u) => u,
                    None => {
                        self.add_error(l, "Invalid value, expected a natural number between 0 and 65535");
                        *current += 1;
                        return;
                    }
                }
            };
            *current += 1;

            self.output.push(opcode);
            self.output.push((value >> 8) as u8);
            self.output.push((value & 0xFF) as u8);
        }
    }

    fn tri_reg(&mut self, l: usize, splited: &Vec<&str>, current: &mut usize, name: &str, opcode: u8) {
        if splited.len() < 4 {
            self.add_error(l, &format!("Usage: {} <register_a> <register_b> <register_c>.", name));
        } else {
            *current += 1;
            let a = match parse_number(splited[*current]) {
                Some(u) => u,
                None => {
                    self.add_error(l, "Invalid register, expected a natural number between 0 and 31");
                    *current += 1;
                    *current += 1;
                    *current += 1;
                    return;
                }
            } as u8;
            *current += 1;
            let b = match parse_number(splited[*current]) {
                Some(u) => u,
                None => {
                    self.add_error(l, "Invalid register, expected a natural number between 0 and 31");
                    *current += 1;
                    *current += 1;
                    return;
                }
            } as u8;
            *current += 1;
            let c = match parse_number(splited[*current]) {
                Some(u) => u,
                None => {
                    self.add_error(l, "Invalid register, expected a natural number between 0 and 31");
                    *current += 1;
                    return;
                }
            } as u8;
            *current += 1;

            self.output.push(opcode);
            self.output.push(a);
            self.output.push(b);
            self.output.push(c);
        }
    }

    pub fn compile(&mut self) -> std::result::Result<Vec<u8>, CompilerErrors> {
        let cloned = self.input.clone();
        for (l, line) in cloned.lines().enumerate() {
            let splited = line.split(|c| {
                c == ' ' || c == '\t'
            }).filter(|el| *el != "").collect::<Vec<_>>();
            let mut current = 0;
            if splited.len() != 0 {
                if line.starts_with(":") {
                    self.labels.insert(splited[0].to_string(), self.output.len() as u16);
                    continue;
                }
                match splited[current] {
                    "load" => self.bi_sixteen(l, &splited, &mut current, "load", LOAD),
                    "move" => self.bi_reg(l, &splited, &mut current, "move", MOVE, None),

                    "add" => self.bi_reg(l, &splited, &mut current, "add", ADD, None),
                    "sub" => self.bi_reg(l, &splited, &mut current, "sub", SUB, None),
                    "mul" => self.bi_reg(l, &splited, &mut current, "mul", MUL, None),
                    "div" => self.bi_reg(l, &splited, &mut current, "div", DIV, None),
                    "mod" => self.bi_reg(l, &splited, &mut current, "mod", MOD, None),

                    "eq" => self.bi_reg(l, &splited, &mut current, "eq", EQ, None),
                    "neq" => self.bi_reg(l, &splited, &mut current, "neq", NEQ, None),
                    "geq" => self.bi_reg(l, &splited, &mut current, "geq", GEQ, None),
                    "ge" => self.bi_reg(l, &splited, &mut current, "ge", GE, None),
                    "leq" => self.bi_reg(l, &splited, &mut current, "leq", LEQ, None),
                    "le" => self.bi_reg(l, &splited, &mut current, "le", LE, None),
                    "not" => self.bi_reg(l, &splited, &mut current, "not", NOT, None),
                    "and" => self.bi_reg(l, &splited, &mut current, "and", AND, None),
                    "or" => self.bi_reg(l, &splited, &mut current, "or", OR, None),
                    "xor" => self.bi_reg(l, &splited, &mut current, "xor", XOR, None),

                    "rqm" => self.bi_sixteen(l, &splited, &mut current, "rqm", RQM),
                    "setb" => self.bi_reg(l, &splited, &mut current, "setb", SETB, Some("byte")),
                    "getb" => self.bi_reg(l, &splited, &mut current, "getb", GETB, None),
                    "mset" => self.tri_reg(l, &splited, &mut current, "mset", MSET),
                    "mmov" => self.tri_reg(l, &splited, &mut current, "mmov", MMOV),
                    "free" => self.bi_reg(l, &splited, &mut current, "free", FREE, None),
                    "ascii" => self.ascii(l, &splited, &mut current),

                    "jmp" => self.jmps(l, &splited, &mut current, "jmp", JMP),
                    "jmpeq" => self.jmps(l, &splited, &mut current, "jmpeq", JMPEQ),
                    "rjmp" => self.jmps(l, &splited, &mut current, "rjmp", RJMP),

                    "hlt" => {
                        self.output.push(HLT);
                        current += 1;
                    }
                    "drg" => {
                        if splited.len() < 2 {
                            self.add_error(l, "Usage: drg <register>.");
                        } else {
                            current += 1;
                            match parse_number(splited[current]) {
                                Some(u) => {
                                    self.output.push(DRG);
                                    self.output.push(u as u8);
                                },
                                None => {
                                    self.add_error(l, "Invalid register, expected a natural number between 0 and 31");
                                }
                            };
                            current += 1;
                        }
                    }
                    "dsp" => {
                        if splited.len() < 2 {
                            self.add_error(l, "Usage: dsp <register>.");
                        } else {
                            current += 1;
                            match parse_number(splited[current]) {
                                Some(u) => {
                                    self.output.push(DSP);
                                    self.output.push(u as u8);
                                },
                                None => {
                                    self.add_error(l, "Invalid register, expected a natural number between 0 and 31");
                                }
                            };
                            current += 1;
                        }
                    }
                    

                    _ => self.add_error(l, &format!("Unknown opcode: `{}`.", splited[current])),
                }
            }
        }

        if self.errors.0.is_empty() {
            Ok(
                self.output.clone()
            )
        } else {
            Err(
                self.errors.clone()
            )
        }
    }

    fn ascii(&mut self, l: usize, splited: &Vec<&str>, current: &mut usize) {
        if splited.len() < 3 {
            self.add_error(l, "Usage: .ascii <register> '<string>'.");
        } else {
            *current += 1;
            let reg = match splited[*current].parse::<u8>() {
                Ok(u) => u,
                Err(_) => {
                    self.add_error(l, "Invalid register, expected a natural number between 0 and 31");
                    *current = splited.len();
                    return;
                }
            };
            *current += 1;

            let to_iter = splited[(*current)..].to_vec();
            
            if !to_iter[0].starts_with("'") {
                self.add_error(l, "Invalid string, strings have to start and finish with a single quotation mark.");
                *current = splited.len();
                return;
            }

            let mut to_push = String::new();

            let mut i = 0;
            while i < to_iter.len() && !to_iter[i].ends_with("'") {
                to_push.push_str(to_iter[i]);
                to_push.push(' ');
                *current += 1;
                i += 1;
            }

            to_push.push_str(to_iter[i]);
            self.output.push(ASCII);
            self.output.push(reg);
            let mut byted = to_push.as_bytes().to_vec();
            
            *(byted.last_mut().unwrap()) = 0x00; // Overwrite closing delimiter with a null terminator.
 
            self.output.extend(
                &byted[1..]
            );
                
        }
    }
}

pub fn parse_number(num: &str) -> Option<u16> {
    if num.starts_with("0x") {
        match u16::from_str_radix(num.trim_start_matches("0x"), 16) {
            Ok(i) => Some(i),
            _ => None,
        }
    } else if num.starts_with("0b") {
        match u16::from_str_radix(num.trim_start_matches("0b"), 2) {
            Ok(i) => Some(i),
            _ => None,
        }
    } else if num.starts_with("%") {
        match u16::from_str_radix(num.trim_start_matches("%"), 8) {
            Ok(i) => Some(i),
            _ => None
        }
    } else {

        match u16::from_str_radix(num, 10) {
            Ok(i) => Some(i),
            _ => None
        }
    }
}

impl Debug for CompilerErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        
        for elem in &self.0 {
            write!(f, "{}\n", elem)?;
        }

        Ok(())
    }
}