use std::fmt::Display;

pub struct SegmentOpCode<'a> {
    pub segment: &'a str,
    pub offset: u32,
}

pub enum OpCode<'a> {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Push(SegmentOpCode<'a>),
    Pop(SegmentOpCode<'a>),
}

impl Display for OpCode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::Add => "add".to_owned(),
            Self::Sub => "sub".to_owned(),
            Self::Neg => "neg".to_owned(),
            Self::Eq => "eq".to_owned(),
            Self::Gt => "gt".to_owned(),
            Self::Lt => "lt".to_owned(),
            Self::And => "and".to_owned(),
            Self::Or => "or".to_owned(),
            Self::Not => "not".to_owned(),
            Self::Push(op) => format!("push {} {}", op.segment, op.offset),
            Self::Pop(op) => format!("pop {} {}", &op.segment, &op.offset),
        };

        write!(f, "{}", v)
    }
}
