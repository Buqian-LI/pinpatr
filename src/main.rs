use anyhow::Result;
use clap::Parser;
use pinpatr::{Pinpatr, Token};

fn main() -> Result<()> {
    let args: Pinpatr = Pinpatr::parse();
    let tokens: Vec<Token> = args.tokenize()?;

    if args.get_debug() {
        println!("[args]\n{:#?}", args);
        println!("[tokenized text]\n{:?}", tokens);
    }

    let output: String = args.transform(tokens)?;
    println!("{}", output);

    Ok(())
}
