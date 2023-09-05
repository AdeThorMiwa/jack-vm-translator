use crate::op_code::{OpCode, SegmentOpCode};
use std::io::Read;

pub struct Parser {
    content: String,
}

impl Parser {
    pub fn new(mut stream: impl Read) -> Self {
        let mut content = String::new();
        let _ = stream.read_to_string(&mut content);

        Self { content }
    }

    pub fn parse(&self) -> Vec<OpCode<'_>> {
        let mut instructions: Vec<OpCode<'_>> = Vec::new();

        for line in self.content.lines() {
            // strip comments and empty spaces
            let instruction = line.trim().splitn(2, "/").next().unwrap().trim();
            if instruction.is_empty() {
                continue;
            }

            if instruction.starts_with("push") {
                let parts = instruction.splitn(3, " ").collect::<Vec<&str>>();
                let push_op_code = SegmentOpCode {
                    segment: parts[1],
                    offset: parts[2]
                        .parse::<u32>()
                        .expect("Invalid index pass to push instruction"),
                };
                instructions.push(OpCode::Push(push_op_code));
                continue;
            }

            if instruction.starts_with("pop") {
                let parts = instruction.splitn(3, " ").collect::<Vec<&str>>();
                let pop_op_code = SegmentOpCode {
                    segment: parts[1],
                    offset: parts[2]
                        .parse::<u32>()
                        .expect("Invalid index pass to pop instruction"),
                };
                instructions.push(OpCode::Pop(pop_op_code));
                continue;
            }

            let op_code = match instruction {
                "add" => OpCode::Add,
                "sub" => OpCode::Sub,
                "neg" => OpCode::Neg,
                "eq" => OpCode::Eq,
                "gt" => OpCode::Gt,
                "lt" => OpCode::Lt,
                "and" => OpCode::And,
                "or" => OpCode::Or,
                "not" => OpCode::Not,
                _ => panic!("invalid op code"),
            };

            instructions.push(op_code)
        }

        instructions
    }
}
