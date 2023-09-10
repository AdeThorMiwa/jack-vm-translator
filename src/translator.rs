use crate::{code_writer::CodeWriter, op_code::OpCode, parser::Parser};
use anyhow::{Context, Ok, Result};
use std::{fs::File, io::BufWriter, path::Path};

pub struct Translator {
    input_filepath: String,
    output_filepath: String,
    bootstrap: bool,
}

impl Translator {
    pub fn new(input_filepath: String, output_filepath: String, bootstrap: bool) -> Self {
        Self {
            input_filepath,
            output_filepath,
            bootstrap,
        }
    }

    pub fn translate(&mut self) -> Result<()> {
        let input_files = self.get_path_files(&self.input_filepath);

        let output_file =
            File::create(self.output_filepath.as_str()).context("Error creating output file")?;
        let mut writer = BufWriter::new(output_file);
        let mut code_writer = CodeWriter::new(&mut writer, self.bootstrap);

        for file in input_files {
            let file = Path::new(&file);
            code_writer.set_current_filename(
                &file
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".vm", ""),
            );
            let input_file = File::open(file).unwrap();
            let parser = Parser::new(&input_file);

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
                    OpCode::Label(op_code) => code_writer.write_label(&op_code.label),
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
        }

        code_writer
            .flush()
            .context("Error flushing writer contents")?;

        Ok(())
    }

    fn get_path_files(&self, raw_path: &str) -> Vec<String> {
        let path = Path::new(raw_path);
        if path.is_file() {
            vec![path.to_str().unwrap().to_owned()]
        } else {
            let mut files = vec![];

            for entry in path.read_dir().expect("unable to read directory") {
                if let std::result::Result::Ok(entry) = entry {
                    if entry.path().to_str().unwrap().ends_with("vm") {
                        files.push(entry.path().to_str().unwrap().to_owned())
                    }
                }
            }

            files
        }
    }
}
