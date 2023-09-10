use crate::op_code::{LabelOpCode, OpCode, SegmentOpCode};
use anyhow::{Ok, Result};
use rand::{distributions::Alphanumeric, Rng};
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

struct FunctionNameStack(Vec<String>);

impl FunctionNameStack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, func_name: &str) {
        self.0.push(func_name.to_owned())
    }

    fn pop(&mut self) {
        self.0.pop();
    }

    fn peek(&self) -> Option<String> {
        self.0.last().map(|i| i.to_owned())
    }
}

pub struct CodeWriter<'a> {
    writer: &'a mut dyn Write,
    lines_written: u32,
    current_filename: String,
    function_name: FunctionNameStack,
}

impl<'a> CodeWriter<'a> {
    pub fn new(writer: &'a mut dyn Write, bootstrap: bool) -> Self {
        let mut code_writer = Self {
            writer,
            lines_written: 0,
            current_filename: "Sys".to_owned(),
            function_name: FunctionNameStack::new(),
        };

        // VM Initialization
        if bootstrap {
            code_writer.write_init();
        }

        code_writer
    }

    fn write_init(&mut self) {
        // bootstrap code
        // this must be placed at the beginning of the output file
        self.write("@256");
        self.write("D=A");
        self.write("@SP");
        self.write_call("Sys.init", 0)
    }

    pub fn set_current_filename(&mut self, filename: &str) {
        self.current_filename = filename.to_owned()
    }

    fn get_current_func(&self) -> String {
        let func_name = self.function_name.peek();
        if func_name.is_some() {
            func_name.unwrap()
        } else {
            "Sys".to_owned()
        }
    }

    fn write_to_stack(&mut self) {
        // push value of D Register to stack
        self.write("@SP");
        self.write("A=M");
        self.write("M=D");

        // increment SP
        self.write("@SP");
        self.write("M=M+1");
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
                self.pop_stack();
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
                self.pop_stack();
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
                "static" => self.write(&format!("@{}.{}", self.current_filename, op_code.offset)),
                _ => {}
            }

            self.write("D=M");
        }

        self.write_to_stack()
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
                "static" => self.write(&format!("@{}.{}", self.current_filename, op_code.offset)),
                _ => {}
            }

            self.write("D=A");
        }

        // store address in @R13
        self.write("@R13");
        self.write("M=D");

        // pop_stack value
        self.write("@SP");
        self.write("AM=M-1");
        self.write("D=M");
        self.write("M=0");

        // update address to popped value
        self.write("@R13");
        self.write("A=M");
        self.write("M=D");
    }

    pub fn write_label(&mut self, label: &str) {
        let func_name = self.get_current_func();
        self.label(&format!("({}__{})", func_name, label))
    }

    pub fn write_goto(&mut self, op_code: &LabelOpCode) {
        let func_name = self.get_current_func();
        self.write(&format!("@{}__{}", func_name, op_code.label));
        self.write("0;JMP")
    }

    pub fn write_if(&mut self, op_code: &LabelOpCode) {
        // get the value on top of the stack
        self.pop_stack();

        let func_name = self.get_current_func();
        self.write(&format!("@{}__{}", func_name, op_code.label));
        self.write("D;JNE")
    }

    pub fn write_call(&mut self, func_name: &str, num_args: u8) {
        // push return-address
        let fn_name = self.get_current_func();
        let return_address = format!("$ret__{}", Self::gen_return_string());
        self.write(&format!("@{}__{}", fn_name, return_address));
        self.write("D=A");
        self.write_to_stack();

        // push LCL
        self.write("@LCL");
        self.write("D=M");
        self.write_to_stack();

        // push ARG
        self.write("@ARG");
        self.write("D=M");
        self.write_to_stack();

        // push THIS
        self.write("@THIS");
        self.write("D=M");
        self.write_to_stack();

        // push THAT
        self.write("@THAT");
        self.write("D=M");
        self.write_to_stack();

        // ARG = SP-n-5
        self.write("@SP");
        self.write("D=M");
        self.write(&format!("@{}", num_args));
        self.write("D=D-A");
        self.write("@5");
        self.write("D=D-A");
        self.write("@ARG");
        self.write("M=D");

        // LCL = SP
        self.write("@SP");
        self.write("D=M");
        self.write("@LCL");
        self.write("M=D");

        // goto f
        self.write(&format!("@{}", func_name));
        self.write("0;JMP");

        // (return-address)
        self.write_label(&return_address);
    }

    pub fn write_return(&mut self) {
        // FRAME = LCL
        self.write("@LCL");
        self.write("D=M");
        self.write("@R6");
        self.write("M=D");

        // RET = *(FRAME-5)
        self.write("@R6");
        self.write("D=M");
        self.write("@5");
        self.write("D=D-A");
        self.write("A=D");
        self.write("D=M");
        self.write("@R7");
        self.write("M=D");

        // *ARG = pop_stack()
        self.pop_stack();
        self.write("@ARG");
        self.write("A=M");
        self.write("M=D");

        // SP = ARG+1
        self.write("@ARG");
        self.write("D=M+1");
        self.write("@SP");
        self.write("M=D");

        // THAT = *(FRAME-1)
        self.write("@R6");
        self.write("D=M");
        self.write("@1");
        self.write("D=D-A");
        self.write("A=D");
        self.write("D=M");
        self.write("@THAT");
        self.write("M=D");

        // THIS = *(FRAME-2)
        self.write("@R6");
        self.write("D=M");
        self.write("@2");
        self.write("D=D-A");
        self.write("A=D");
        self.write("D=M");
        self.write("@THIS");
        self.write("M=D");

        // ARG = *(FRAME-3)
        self.write("@R6");
        self.write("D=M");
        self.write("@3");
        self.write("D=D-A");
        self.write("A=D");
        self.write("D=M");
        self.write("@ARG");
        self.write("M=D");

        // LCL = *(FRAME-4)
        self.write("@R6");
        self.write("D=M");
        self.write("@4");
        self.write("D=D-A");
        self.write("A=D");
        self.write("D=M");
        self.write("@LCL");
        self.write("M=D");

        // GOTO RET
        self.write("@R7");
        self.write("A=M");
        self.write("0;JMP");

        self.function_name.pop();
    }

    pub fn write_function(&mut self, func_name: &str, num_locals: u8) {
        self.function_name.push(&func_name);
        self.label(&format!("({})", func_name));

        for k in 0..num_locals {
            self.write("@LCL");
            self.write("D=M");
            self.write(&format!("@{}", k));
            self.write("D=D+A");

            // store address in @R13
            self.write("@R13");
            self.write("M=D");

            // pop_stack value
            self.write("@0");
            self.write("D=A");

            // update address to popped value
            self.write("@R13");
            self.write("A=M");
            self.write("M=D");
        }
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
        self.pop_stack();

        // get second operand
        self.write("@SP");
        self.write("AM=M-1");
    }

    fn pop_stack(&mut self) {
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

    fn gen_return_string() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect()
    }
}
