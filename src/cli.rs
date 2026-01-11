use anyhow::Result;
use clap::Parser;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use format::Format;
use syllable::Syllable;
use token::Token;

use crate::error::SiphonError;

pub mod format;
pub mod syllable;
pub mod token;

#[derive(Parser, Debug, Clone)]
#[command(
    author = "Buqian LI <buqian.li@outlook.com>",
    version = "1.6.0",
    about = "A CLI tool box for Chinese phonological conversion (SInoPHONe).",
    long_about = None,
    help_template= "\
    {name} {version}
  {author}
  {about}

USAGE: {usage}

INPUT: {positionals}

OPTIONS:
{options}
    "
)]
pub struct Siphon {
    /// Transcription format of the output text
    /// (aliases: toneformat, textformat)
    #[arg(
        value_enum,
        short = 'f',
        long = "format",
        alias = "toneformat",
        alias = "textformat",
        default_value = "dia",
        ignore_case = true
    )]
    format: Format,
    /// Custom LaTeX wrapper command (aliases: wrapper, latex, latexwrapper, latex-wrapper)
    /// [Note] Only the command name part will be replaced.
    #[arg(
        short = 'r',
        long = "wrap",
        alias = "wrapper",
        alias = "latex",
        alias = "latexwrapper",
        alias = "latex-wrapper",
        default_value = "textsuperscript",
        verbatim_doc_comment
    )]
    latex_wrapper: String,
    /// Text in Pinyin to convert
    /// [Attention]
    /// For the phoneme ü, you can input either 'v', the decomposed form 'ü' (u +  ̈), or the precomposed form 'ü'.
    /// The output will always use the decomposed form, as most fonts prioritize support for decomposed characters
    /// over precomposed ones. This ensures consistent rendering across systems.
    #[arg(
        name = "INPUT",
        trailing_var_arg = true,
        allow_hyphen_values = true,
        verbatim_doc_comment
    )]
    text: Vec<String>, // NOTE: must be this type to be able to receive continous args
    /// Print debug info
    #[arg(short = 'd', long = "debug", default_value_t = false)]
    debug: bool,
}

impl Default for Siphon {
    fn default() -> Self {
        Self {
            format: Format::PinyinDiacritic,
            latex_wrapper: String::from("textsuperscript"),
            text: vec![],
            debug: false,
        }
    }
}

impl Siphon {
    pub fn new(text: &str) -> Self {
        let text: Vec<String> = text
            .split_whitespace()
            .map(|value| value.to_string())
            .collect();
        Self {
            text,
            ..Default::default()
        }
    }

    /// Set conversion format
    ///
    /// Possible formats:
    ///     - PinyinDiacritic
    ///     - PinyinLaTeX
    ///     - IPALaTeX
    ///     - IPASuperscript
    pub fn format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    /// Set input pinyin text
    pub fn text(mut self, text: String) -> Self {
        self.text = text
            .split_whitespace()
            .map(|value| value.to_string())
            .collect();
        self
    }

    /// Show debug info
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Set latex wrapper command name
    /// only valid for `Format::PinyinLaTeX` and `Format::IPALaTeX`
    pub fn wrapper(mut self, wrapper: &str) -> Self {
        self.latex_wrapper = wrapper.to_string();
        self
    }

    pub fn get_format(&self) -> &Format {
        &self.format
    }

    pub fn get_text(&self) -> String {
        self.text.join(" ").clone()
    }

    pub fn get_debug(&self) -> bool {
        self.debug
    }

    pub fn get_latex_wrapper(&self) -> &str {
        &self.latex_wrapper
    }

    pub fn set_format(&mut self, format: Format) {
        self.format = format
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug
    }

    pub fn set_latex_wrapper(&mut self, wrapper: &str) {
        self.latex_wrapper = wrapper.to_string()
    }

    /// Normalize the input text using NFC to handle combining diacritics
    fn normalize_input_to_unicode(&self) -> String {
        let input: &String = &self.text.join(" ");
        let normalized_text: String = input.nfc().collect::<String>();
        if *input != normalized_text {
            println!("Input text has been normalized as -> {:?}", normalized_text);
        }
        normalized_text
    }

    /// Regex pattern to match:
    /// 1. A sequence of letters followed by an optional number (e.g., zhe4, shi)
    /// 2. Keep spaces and punctuation in order to reproduce the same final text
    fn get_regex(&self) -> Result<Regex, SiphonError> {
        // (?x) to make # xxxx to be ignored
        Ok(Regex::new(
            r#"(?x)
            (?i: # case-insensitive
                (?<syllable>
                    (?<initial>zh|ch|sh|[bpmfdtnlgkhjqxrzcs]?)     # Optional initial (excluding y and w)
                    (?<rime>(?:y|w)?[aeiouüv]{1,3}(?:ng|n)?(?:r)?) # Required rime
                    (?<tone>\d?)                                   # Optional tone
                )
            )
            (?-i)
            |(?<space>\s+)
            |(?<quote>['])
            |(?<punctuation>[,!?.\-:"=])
            "#,
        )?)
    }

    /// Correct rhyme parsing
    fn normalize_rhyme(&self, onset: Option<&str>, rhyme: String) -> String {
        let rhyme = match onset {
            Some("j" | "q" | "x" | "J" | "Q" | "X") => rhyme.replace("u", "ü"),
            Some("zh" | "ch" | "sh" | "r" | "Zh" | "Ch" | "Sh" | "R") => rhyme.replace("i", "r"),
            Some("z" | "c" | "s" | "Z" | "S" | "C") => rhyme.replace("i", "z"),
            _ => rhyme,
        };

        rhyme
            .replace("yu", "ü")
            .replace("y", "i")
            .replace("ii", "i")
            .replace("w", "u")
            .replace("uu", "u")
    }

    /// Convert text from String to Vec<Token> using Regex
    pub fn tokenize(&self) -> Result<Vec<Token>, SiphonError> {
        let mut tokens: Vec<Token> = Vec::new();
        let text: String = self.normalize_input_to_unicode();
        let regex: Regex = self.get_regex()?;

        for captures in regex.captures_iter(&text) {
            if let Some(syllable) = captures.name("syllable") {
                let onset: Option<&str> = captures
                    .name("initial")
                    .filter(|m| !m.as_str().is_empty())
                    .map(|on| on.as_str());

                let mut rhyme: String =
                    match captures.name("rime").filter(|m| !m.as_str().is_empty()) {
                        Some(value) => value.as_str().to_string(),
                        None => return Err(SiphonError::RhymeNotFound),
                    };

                let tone: Option<usize> =
                    captures.name("tone").and_then(|t| t.as_str().parse().ok());

                if matches!(self.format, Format::IPALaTeX)
                    | matches!(self.format, Format::IPASuperscript)
                {
                    rhyme = self.normalize_rhyme(onset, rhyme);
                }

                let token = Token::Syllable(
                    Syllable::new()
                        .full_syllable(syllable.as_str())
                        .onset(onset)
                        .rhyme(rhyme)
                        .tone(tone),
                );
                tokens.push(token);
            } else if captures.name("space").is_some() {
                tokens.push(Token::Space);
            } else if captures.name("quote").is_some() {
                tokens.push(Token::Separator);
            } else if let Some(punct) = captures.name("punctuation") {
                tokens.push(Token::Punctuation(punct.as_str().to_string()));
            }
        }

        Ok(tokens)
    }

    pub fn transform(&self, tokens: Vec<Token>) -> Result<String, SiphonError> {
        let transformed: Vec<String> = tokens
            .iter()
            .map(|tok| {
                let (word_transformed, tone_transformed) = match tok {
                    Token::Syllable(syl) => match self.format {
                        Format::PinyinLaTeX
                        | Format::PinyinDiacritic
                        | Format::PinyinSuperscript => {
                            syl.convert_to_pinyin(self.get_format(), self.get_latex_wrapper())?
                        }
                        Format::IPALaTeX | Format::IPASuperscript => {
                            syl.convert_to_ipa(self.get_format(), self.get_latex_wrapper())?
                        }
                    },
                    Token::Separator => match self.format {
                        // keep the separator
                        Format::PinyinDiacritic => (String::from("'"), String::new()),
                        // remove the separator
                        Format::PinyinSuperscript
                        | Format::PinyinLaTeX
                        | Format::IPALaTeX
                        | Format::IPASuperscript => (String::new(), String::new()),
                    },
                    Token::Punctuation(p) => (p.clone(), String::new()),
                    Token::Space => (String::from(" "), String::new()),
                };

                // Combine the transformed word and tone
                Ok(format!("{}{}", word_transformed, tone_transformed))
            })
            .collect::<Result<Vec<String>>>()?;

        Ok(transformed.join(""))
    }
}
