use std::fmt::Display;

pub struct SegmentOpCode<'a> {
    pub segment: &'a str,
    pub offset: u32,
}

impl<'a> SegmentOpCode<'a> {
    pub fn is_scoped_segment(&self) -> bool {
        ["local", "argument", "this", "that"].contains(&self.segment)
    }
}

pub struct LabelOpCode<'a> {
    pub label: &'a str,
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
    Label(LabelOpCode<'a>),
    Goto(LabelOpCode<'a>),
    If(LabelOpCode<'a>),
    Call { func_name: &'a str, num_args: u8 },
    Return,
    Function { func_name: &'a str, num_locals: u8 },
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
            Self::Pop(op) => format!("pop {} {}", op.segment, op.offset),
            Self::Label(op) => format!("label {}", op.label),
            Self::Goto(op) => format!("goto {}", op.label),
            Self::If(op) => format!("if-goto {}", op.label),
            Self::Call {
                func_name,
                num_args,
            } => format!("call {} {}", func_name, num_args),
            Self::Function {
                func_name,
                num_locals,
            } => format!("function {} {}", func_name, num_locals),
            Self::Return => "return".to_owned(),
        };

        write!(f, "{}", v)
    }
}
