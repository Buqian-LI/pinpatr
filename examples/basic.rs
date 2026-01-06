use pinpatr::{Pinpatr, Token};

fn main() {
    let converter: Pinpatr = Pinpatr::new().text(vec!["ni3".to_string(), "hao3".to_string()]);
    println!("[Input] ni3 hao3");

    let tokens: Vec<Token> = converter.tokenize().unwrap();
    let result: String = converter.transform(tokens).unwrap();
    println!("[Output] {}", result);
}
