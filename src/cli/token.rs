use crate::cli::syllable::Syllable;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Syllable(Syllable),
    Punctuation(String),
    Separator,
    Space,
}
