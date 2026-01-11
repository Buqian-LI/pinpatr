use thiserror::Error;

#[derive(Error, Debug)]
pub enum SiphonError {
    // #[error("Database connection error: {0}")]
    // Conn(#[from] diesel::ConnectionError),

    // #[error("Database error: {0}")]
    // Database(#[from] diesel::result::Error),
    //
    #[error("Pinyin conversion error: {0}")]
    Conversion(#[from] anyhow::Error),

    #[error("Could not get regex expression: {0}")]
    Regex(#[from] regex::Error),

    #[error("Missing vowels in the input text")]
    VowelMissing,

    #[error("Missing vowels in the input text")]
    RhymeNotFound,

    #[error("There are tones messed up in your input! -> {0}")]
    TonConversionFail(String),

    #[error("The initial is not valid: {0}")]
    InvalidInitial(String),

    #[error("The rhyme is not valid: {0}")]
    InvalidRhyme(String),
}
