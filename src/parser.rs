use crate::op_code::{LabelOpCode, OpCode, SegmentOpCode};
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

            if instruction.starts_with("label") {
                let parts = instruction.splitn(2, " ").collect::<Vec<&str>>();
                let label_op_code = LabelOpCode { label: parts[1] };
                instructions.push(OpCode::Label(label_op_code));
                continue;
            }

            if instruction.starts_with("goto") {
                let parts = instruction.splitn(2, " ").collect::<Vec<&str>>();
                let goto_op_code = LabelOpCode { label: parts[1] };
                instructions.push(OpCode::Goto(goto_op_code));
                continue;
            }

            if instruction.starts_with("if-goto") {
                let parts = instruction.splitn(2, " ").collect::<Vec<&str>>();
                let if_op_code = LabelOpCode { label: parts[1] };
                instructions.push(OpCode::If(if_op_code));
                continue;
            }

            if instruction.starts_with("call") {
                let parts = instruction.splitn(3, " ").collect::<Vec<&str>>();
                let call_op_code = OpCode::Call {
                    func_name: parts[1],
                    num_args: parts[2].parse::<u8>().unwrap(),
                };
                instructions.push(call_op_code);
                continue;
            }

            if instruction.starts_with("function") {
                let parts = instruction.splitn(3, " ").collect::<Vec<&str>>();
                let func_op_code = OpCode::Function {
                    func_name: parts[1],
                    num_locals: parts[2].parse::<u8>().unwrap(),
                };
                instructions.push(func_op_code);
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
                "return" => OpCode::Return,
                _ => {
                    println!("Invalid instruction >>> `{}`", instruction);
                    panic!("Invalid instruction")
                }
            };

            instructions.push(op_code)
        }

        instructions
    }
}
