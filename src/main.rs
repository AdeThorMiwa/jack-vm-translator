use anyhow::{Context, Result};
use clap::Parser;
use vm_translator::translator::Translator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input: String,
}

fn main() -> Result<()> {
    let cfg = Args::parse();
    let mut translator = Translator::new(&cfg.input).context("Translator initialization failed")?;
    translator.translate().context("Error during translation")?;
    Ok(())
}
