use siphon::{Siphon, Token};

fn main() {
    println!("[Input] ni3 hao3");

    let converter: Siphon = Siphon::new("ni3 hao3");
    let tokens: Vec<Token> = converter.tokenize().unwrap();
    let result: String = converter.transform(tokens).unwrap();

    println!("[Output] {}", result);
}
