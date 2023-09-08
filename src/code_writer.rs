use crate::op_code::{LabelOpCode, OpCode, SegmentOpCode};
use anyhow::{Ok, Result};
use std::io::Write;

// const SP: u8 = 0; // stores the memory address of the topmost stack value
// const LCL: u8 = 1; // stores the base address of the local virtual segment
// const ARG: u8 = 2; // stores the base address of the argument virtual segment
// const THIS: u8 = 3; // stores the base of the current this segment within the heap
// const THAT: u8 = 4; // stores the base of the current that segment within the heap
// const TEMP: u8 = 5; // 5-12 holds temp segments
// const STACK: u16 = 256;
// static variables are going to be XXX.i where XXX is the name of the Generated file

/**
 * 0-15 virtual registers
 * 16-255 static variables
 * 256-2047 stack
 * 2048-16483 Heap (store objects and arrays)
 * 16384-24575 Memory mapped I/O
 *
 *
 * Virtual Segment
 * 1. argument
 * 2. local
 * 3. this
 * 4. that
 * 5. pointer
 * 6. static
 * 7. temp
 * 8. constant
 */

pub struct CodeWriter<'a> {
    writer: &'a mut dyn Write,
    lines_written: u32,
    static_prefix: &'a str,
    func_name: Option<String>,
}

impl<'a> CodeWriter<'a> {
    pub fn new(writer: &'a mut dyn Write, static_prefix: &'a str) -> Self {
        let mut code_writer = Self {
            writer,
            lines_written: 0,
            static_prefix,
            func_name: None,
        };

        // VM Initialization
        code_writer.write_init();

        code_writer
    }

    fn write_init(&mut self) {
        // bootstrap code
        // this must be placed at the beginning of the output file
    }

    pub fn set_current_func(&mut self, name: Option<String>) {
        self.func_name = name
    }

    fn get_current_func(&self) -> &str {
        if self.func_name.is_some() {
            self.func_name.as_ref().unwrap()
        } else {
            "Main"
        }
    }

    pub fn write_arithmetic(&mut self, op_code: &OpCode) {
        match op_code {
            OpCode::Add => {
                self.write_double_operand();
                self.write("D=D+M")
            }
            OpCode::Sub => {
                self.write_double_operand();
                self.write("D=M-D")
            }
            OpCode::Neg => {
                self.write_single_operand();
                self.write("D=-D")
            }
            OpCode::Eq => self.write_conditional("D;JEQ"),
            OpCode::Gt => self.write_conditional("D;JGT"),
            OpCode::Lt => self.write_conditional("D;JLT"),
            OpCode::And => {
                self.write_double_operand();
                self.write("D=D&M")
            }
            OpCode::Or => {
                self.write_double_operand();
                self.write("D=D|M")
            }
            OpCode::Not => {
                self.write_single_operand();
                self.write("D=!D")
            }
            _ => self.comment("Invalid Opcode"),
        }

        // push result back unto stack
        self.write("@SP");
        self.write("A=M");
        self.write("M=D");

        // increment SP
        self.write("@SP");
        self.write("M=M+1");
    }

    pub fn write_push(&mut self, op_code: &SegmentOpCode) {
        // retrieve value from segment and store in D Register
        if op_code.is_scoped_segment() {
            match op_code.segment {
                "local" => self.write("@LCL"),
                "argument" => self.write("@ARG"),
                "this" => self.write("@THIS"),
                "that" => self.write("@THAT"),
                _ => {}
            }

            self.write("D=M");
            self.write(&format!("@{}", op_code.offset));
            self.write("A=D+A");
            self.write("D=M");
        } else if op_code.segment == "constant" {
            self.write(&format!("@{}", op_code.offset));
            self.write("D=A");
        } else {
            match op_code.segment {
                "temp" => self.write(&format!("@{}", 5 + op_code.offset)),
                "pointer" => self.write(&format!("@{}", 3 + op_code.offset)),
                "static" => self.write(&format!("@{}.{}", self.static_prefix, op_code.offset)),
                _ => {}
            }

            self.write("D=M");
        }

        // push value of D Register to stack
        self.write("@SP");
        self.write("A=M");
        self.write("M=D");

        // increment SP
        self.write("@SP");
        self.write("M=M+1");
    }

    pub fn write_pop(&mut self, op_code: &SegmentOpCode) {
        if op_code.is_scoped_segment() {
            // set segment
            match op_code.segment {
                "local" => self.write("@LCL"),
                "argument" => self.write("@ARG"),
                "this" => self.write("@THIS"),
                "that" => self.write("@THAT"),
                _ => {}
            }

            // get address
            self.write("D=M");
            self.write(&format!("@{}", op_code.offset));
            self.write("D=D+A");
        } else {
            match op_code.segment {
                "temp" => self.write(&format!("@{}", 5 + op_code.offset)),
                "pointer" => self.write(&format!("@{}", 3 + op_code.offset)),
                "static" => self.write(&format!("@{}.{}", self.static_prefix, op_code.offset)), // TODO: recheck if filename should be for individual compiled files or output file
                _ => {}
            }

            self.write("D=A");
        }

        // store address in @R13
        self.write("@R13");
        self.write("M=D");

        // pop value
        self.write("@SP");
        self.write("AM=M-1");
        self.write("D=M");
        self.write("M=0");

        // update address to popped value
        self.write("@R13");
        self.write("A=M");
        self.write("M=D");
    }

    pub fn write_label(&mut self, op_code: &LabelOpCode) {
        let func_name = self.get_current_func();
        self.label(&format!("({}__{})", func_name, op_code.label))
    }

    pub fn write_goto(&mut self, op_code: &LabelOpCode) {
        let func_name = self.get_current_func();
        self.write(&format!("@{}__{}", func_name, op_code.label));
        self.write("0;JMP")
    }

    pub fn write_if(&mut self, op_code: &LabelOpCode) {
        // get the value on top of the stack
        self.write_single_operand();

        let func_name = self.get_current_func();
        self.write(&format!("@{}__{}", func_name, op_code.label));
        self.write("D;JNE")
    }

    pub fn write_call(&mut self, func_name: &str, num_args: u8) {
        let return_address = format!("");
        // push return-address
        // push LCL
        // push ARG
        // push THIS
        // push THAT
        // ARG = SP-n-5
        // LCL = SP
        // goto f
        // (return-address)
    }

    pub fn write_return(&mut self) {}

    pub fn write_function(&mut self, func_name: &str, num_locals: u8) {
        self.set_current_func(Some(func_name.to_string()))
    }

    pub fn comment(&mut self, comment: &str) {
        write!(&mut self.writer, "// {}\n", comment).unwrap();
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }

    fn write(&mut self, line: &str) {
        write!(&mut self.writer, "\t{}\n", line).unwrap();
        self.lines_written += 1;
    }

    fn label(&mut self, line: &str) {
        write!(&mut self.writer, "{}\n", line).unwrap();
    }

    fn write_double_operand(&mut self) {
        // get first operand
        self.write_single_operand();

        // get second operand
        self.write("@SP");
        self.write("AM=M-1");
    }

    fn write_single_operand(&mut self) {
        // get first operand
        self.write("@SP");
        self.write("AM=M-1");
        self.write("D=M");
        self.write("M=0");
    }

    fn write_conditional(&mut self, condition: &str) {
        self.write_double_operand();
        self.write("D=M-D");
        self.write("M=0");
        self.write(&format!("@{}", self.lines_written + 5));
        self.write(condition);
        self.write("D=0");
        self.write(&format!("@{}", self.lines_written + 3));
        self.write("0;JMP");
        self.write("D=-1")
    }
}
