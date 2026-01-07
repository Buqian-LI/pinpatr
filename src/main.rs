use anyhow::Result;
use clap::Parser;
use siphon::{Siphon, Token};

fn main() -> Result<()> {
    let siphon: Siphon = Siphon::parse();
    let tokens: Vec<Token> = siphon.tokenize()?;

    if siphon.get_debug() {
        println!("[args]\n{:#?}", siphon);
        println!("[tokenized text]\n{:?}", tokens);
    }

    let output: String = siphon.transform(tokens)?;
    println!("{}", output);

    Ok(())
}
