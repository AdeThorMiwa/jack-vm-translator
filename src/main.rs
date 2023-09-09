use anyhow::{Context, Result};
use clap::Parser;
use vm_translator::translator::Translator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,
}

fn main() -> Result<()> {
    let cfg = Args::parse();
    let mut translator = Translator::new(cfg.input, cfg.output);
    translator.translate().context("Error during translation")?;
    Ok(())
}
