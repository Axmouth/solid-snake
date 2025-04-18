use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{
    executor::interpreted::opcode_decoder::{initialize_dispatch_table, SIZE_OF_REGISTER},
    opcodes::OpCode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum JumpWithLabel<'a> {
    Jump(&'a str),
    JumpIf(&'a str, u8),
    JumpIfFalse(&'a str, u8),
    CallFunction(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProcessedLinePassOne<'a> {
    Jump(JumpWithLabel<'a>),
    Compiled(Vec<u8>),
    Label(Cow<'a, str>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProcessedLinePassTwo<'a> {
    Jump(JumpWithLabel<'a>),
    Compiled(Vec<u8>),
}

pub fn parse_byte_code_from_txt(code: &str) -> (Vec<u8>, Vec<(String, usize)>) {
    let dispatch_table = initialize_dispatch_table();
    // remove comments
    let code: String = code
        .lines()
        .map(|line| {
            if let Some((code, _comment)) = line.split_once(';') {
                code
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut labels_to_line: HashMap<&str, u64> = HashMap::new();

    let labels: HashSet<&str> = code
        .lines()
        .filter_map(|line| {
            let tokens: Vec<&str> = line
                .split_whitespace()
                .filter(|token| token.ends_with(':'))
                .collect();
            tokens.first().copied()
        })
        .collect();

    let mut first_pass: Vec<(ProcessedLinePassOne, usize)> = Vec::new();

    let mut byte_count = 0;
    let mut lined_code: Vec<(String, usize)> = Vec::new();
    let mut counter = 0;
    for (line_idx, line) in code.lines().enumerate() {
        lined_code.push((line.to_string(), byte_count));
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if labels.contains((tokens[0].to_string()).as_str()) {
            first_pass.push((
                ProcessedLinePassOne::Label((tokens[0].to_string()).into()),
                line_idx,
            ));
            tokens = tokens[1..].to_vec();
        }

        if tokens.is_empty() {
            continue;
        }

        let opcode_str = tokens[0];

        let opcode =
            OpCode::from_str(opcode_str).unwrap_or_else(|_| panic!("Wrong opcode {opcode_str}"));
        byte_count += size_of::<u16>() + dispatch_table[opcode as usize].1;

        if (opcode_str.starts_with("Jump") || opcode_str.starts_with("CallFunction"))
            && labels.contains((tokens[1].to_string() + ":").as_str())
        {
            match opcode {
                OpCode::Jump => {
                    if tokens.len() > 2 {
                        panic!("too many Jump arguments")
                    }
                    first_pass.push((
                        ProcessedLinePassOne::Jump(JumpWithLabel::Jump(tokens[1])),
                        line_idx,
                    ));
                }
                OpCode::JumpIf => {
                    if tokens.len() > 3 {
                        panic!("too many JumpIf arguments")
                    }
                    first_pass.push((
                        ProcessedLinePassOne::Jump(JumpWithLabel::JumpIf(
                            tokens[1],
                            u8::from_str(tokens[2].strip_prefix('R').unwrap()).unwrap(),
                        )),
                        line_idx,
                    ));
                }
                OpCode::JumpIfFalse => {
                    if tokens.len() > 3 {
                        panic!("too many JumpIfFalse arguments")
                    }
                    first_pass.push((
                        ProcessedLinePassOne::Jump(JumpWithLabel::JumpIfFalse(
                            tokens[1],
                            u8::from_str(tokens[2].strip_prefix('R').unwrap()).unwrap(),
                        )),
                        line_idx,
                    ));
                }
                OpCode::CallFunction => {
                    if tokens.len() > 2 {
                        panic!("too many CallFunction arguments")
                    }
                    first_pass.push((
                        ProcessedLinePassOne::Jump(JumpWithLabel::CallFunction(tokens[1])),
                        line_idx,
                    ));
                }

                _ => {}
            }

            continue;
        }

        tokens = tokens[1..].to_vec();

        // TODO: work with hex
        // TODO: handle cases where the type is not shown by the instruction, like memory handling
        let num_encoder = |unparsed: &str| {
            if opcode_str.ends_with("I8") {
                parse_int_with_radix::<i8>(unparsed).to_be_bytes().to_vec()
            } else if opcode_str.ends_with("I16") {
                parse_int_with_radix::<i16>(unparsed).to_be_bytes().to_vec()
            } else if opcode_str.ends_with("I32") {
                parse_int_with_radix::<i32>(unparsed).to_be_bytes().to_vec()
            } else if opcode_str.ends_with("I64") {
                parse_int_with_radix::<i64>(unparsed).to_be_bytes().to_vec()
            } else if opcode_str.ends_with("U8") {
                parse_int_with_radix::<u8>(unparsed).to_be_bytes().to_vec()
            } else if opcode_str.ends_with("U16") {
                parse_int_with_radix::<u16>(unparsed).to_be_bytes().to_vec()
            } else if opcode_str.ends_with("U32") {
                parse_int_with_radix::<u32>(unparsed).to_be_bytes().to_vec()
            } else if opcode_str.ends_with("U64") {
                parse_int_with_radix::<u64>(unparsed).to_be_bytes().to_vec()
            } else if opcode_str.ends_with("F32") {
                f32::from_str(unparsed).unwrap().to_be_bytes().to_vec()
            } else if opcode_str.ends_with("F64") {
                f64::from_str(unparsed).unwrap().to_be_bytes().to_vec()
            } else {
                panic!("Unhandled: {}", opcode_str);
            }
        };

        let mut byte_code = Vec::new();
        byte_code.extend_from_slice(&(opcode as u16).to_be_bytes());
        for token in tokens {
            if let Some(stripped) = token.strip_prefix('R') {
                let reg_num = u8::from_str(stripped).unwrap();
                byte_code.extend_from_slice(&reg_num.to_be_bytes());
            } else {
                byte_code.extend_from_slice(&num_encoder(token))
            }
        }

        first_pass.push((ProcessedLinePassOne::Compiled(byte_code), line_idx));
    }

    lined_code.push((String::new(), byte_count));

    let mut program_counter_second_pass = 0;
    let second_pass: Vec<(ProcessedLinePassTwo, usize)> = first_pass
        .iter()
        .filter_map(|(line, line_idx)| {
            match line {
                ProcessedLinePassOne::Compiled(bc) => {
                    counter += 1;
                    program_counter_second_pass += bc.len();
                    Some((ProcessedLinePassTwo::Compiled(bc.clone()), *line_idx))
                }
                ProcessedLinePassOne::Jump(jump_with_label) => {
                    // Add enough instructions for the jumps + args
                    match &jump_with_label {
                        JumpWithLabel::Jump(_) | JumpWithLabel::CallFunction(_) => {
                            program_counter_second_pass += size_of::<u16>() + size_of::<u64>();
                        }
                        JumpWithLabel::JumpIf(_, _) | JumpWithLabel::JumpIfFalse(_, _) => {
                            program_counter_second_pass +=
                                size_of::<u16>() + size_of::<u64>() + SIZE_OF_REGISTER;
                        }
                    }

                    Some((
                        ProcessedLinePassTwo::Jump(jump_with_label.clone()),
                        *line_idx,
                    ))
                }
                ProcessedLinePassOne::Label(label) => {
                    labels_to_line.insert(label, program_counter_second_pass as u64);
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    let bc = second_pass
        .into_iter()
        .flat_map(|(line, _line_idx)| {
            // dbg!(&line, _line_idx);

            match line {
                ProcessedLinePassTwo::Jump(jump_with_label) => {
                    let mut bc = Vec::new();

                    match jump_with_label {
                        JumpWithLabel::Jump(label) => {
                            bc.extend_from_slice(&(OpCode::Jump as u16).to_be_bytes());
                            bc.extend_from_slice(
                                &labels_to_line[(label.to_string() + ":").as_str()].to_be_bytes(),
                            );
                        }
                        JumpWithLabel::JumpIf(label, if_reg) => {
                            bc.extend_from_slice(&(OpCode::JumpIf as u16).to_be_bytes());
                            bc.extend_from_slice(
                                &labels_to_line[(label.to_string() + ":").as_str()].to_be_bytes(),
                            );
                            bc.extend_from_slice(&if_reg.to_be_bytes());
                        }
                        JumpWithLabel::JumpIfFalse(label, if_reg) => {
                            bc.extend_from_slice(&(OpCode::JumpIfFalse as u16).to_be_bytes());
                            bc.extend_from_slice(
                                &labels_to_line[(label.to_string() + ":").as_str()].to_be_bytes(),
                            );
                            bc.extend_from_slice(&if_reg.to_be_bytes());
                        }
                        JumpWithLabel::CallFunction(label) => {
                            bc.extend_from_slice(&(OpCode::CallFunction as u16).to_be_bytes());
                            bc.extend_from_slice(
                                &labels_to_line[(label.to_string() + ":").as_str()].to_be_bytes(),
                            );
                        }
                    }

                    bc.into_iter()
                }
                ProcessedLinePassTwo::Compiled(items) => items.into_iter(),
            }
        })
        .collect();

    (bc, lined_code)
}
fn parse_int_with_radix<T: FromStrRadix + FromStr>(s: &str) -> T where <T as std::str::FromStr>::Err: std::fmt::Debug {
    if let Some(stripped) = s.strip_prefix("0x") {
        T::from_str_radix(stripped, 16).unwrap()
    } else if let Some(stripped) = s.strip_prefix("0b") {
        T::from_str_radix(stripped, 2).unwrap()
    } else if let Some(stripped) = s.strip_prefix("0o") {
        T::from_str_radix(stripped, 8).unwrap()
    } else {
        s.parse().unwrap()
    }
}

trait FromStrRadix: Sized + FromStr {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, <Self as std::str::FromStr>::Err>;
}

macro_rules! impl_from_str_radix {
    ($($t:ty),+) => {
        $(impl FromStrRadix for $t {
            fn from_str_radix(s: &str, radix: u32) -> Result<Self, <Self as std::str::FromStr>::Err> {
                <$t>::from_str_radix(s, radix)
            }
        })+
    };
}

impl_from_str_radix!(u8, u16, u32, u64, i8, i16, i32, i64);