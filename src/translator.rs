use crate::{code_writer::CodeWriter, op_code::OpCode, parser::Parser};
use anyhow::{Context, Ok, Result};
use std::{fs::File, io::BufWriter, path::Path};

pub struct Translator {
    input_file: File,
    out_filepath: String,
}

impl Translator {
    pub fn new(file_path: &str) -> Result<Self> {
        let input_file = File::open(file_path).unwrap();

        let out_filepath = file_path.replace(".vm", ".asm");

        Ok(Self {
            input_file,
            out_filepath,
        })
    }

    pub fn translate(&mut self) -> Result<()> {
        // TODO: accept dir and parse each .vm file in dir
        // pass same codewriter to each parsing to unify output to one single .asm file
        let parser = Parser::new(&self.input_file);

        let output_file =
            File::create(self.out_filepath.as_str()).context("Error creating output file")?;
        let mut writer = BufWriter::new(output_file);
        let filename = Path::new(&self.out_filepath)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".asm", "");
        let mut code_writer = CodeWriter::new(&mut writer, &filename);

        let op_codes = parser.parse();
        for op_code in op_codes {
            code_writer.comment(&format!("{}", op_code.to_string()));

            match op_code {
                OpCode::Add
                | OpCode::Sub
                | OpCode::Neg
                | OpCode::Eq
                | OpCode::Gt
                | OpCode::Lt
                | OpCode::And
                | OpCode::Or
                | OpCode::Not => code_writer.write_arithmetic(&op_code),
                OpCode::Push(push_op_code) => code_writer.write_push(&push_op_code),
                OpCode::Pop(pop_op_code) => code_writer.write_pop(&pop_op_code),
                OpCode::Label(op_code) => code_writer.write_label(&op_code),
                OpCode::Goto(op_code) => code_writer.write_goto(&op_code),
                OpCode::If(op_code) => code_writer.write_if(&op_code),
                OpCode::Call {
                    func_name,
                    num_args,
                } => code_writer.write_call(func_name, num_args),
                OpCode::Function {
                    func_name,
                    num_locals,
                } => code_writer.write_function(func_name, num_locals),
                OpCode::Return => code_writer.write_return(),
            };
        }

        code_writer
            .flush()
            .context("Error flushing writer contents")?;

        Ok(())
    }
}
