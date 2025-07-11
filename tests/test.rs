use pinpatr::cli::format::Format;
use pinpatr::cli::syllable::Syllable;
use pinpatr::cli::token::Token;
use pinpatr::cli::Pinpatr;
use unicode_normalization::UnicodeNormalization;

#[test]
fn test_default_cli() {
    let args = Pinpatr::default();

    assert!(!args.get_debug());
    assert_eq!(args.get_format(), &Format::PinyinDiacritic);
    assert_eq!(args.get_latex_wrapper(), "textsuperscript");
}

#[test]
fn test_new_cli() {
    let mut args = Pinpatr::new();

    args.set_text(vec!["\"xi1'an1?\"".to_string()]);
    args.set_format(Format::IPASuperscript);
    args.set_debug(true);

    assert_eq!(args.get_format(), &Format::IPASuperscript);
    assert!(args.get_debug());
}

#[test]
fn test_cli_builder() {
    let builder = Pinpatr::new()
        .format(Format::IPASuperscript)
        .text(vec![String::from("\"xi1'an1?\"")])
        .wrapper("UP");

    assert_eq!(builder.get_text(), &vec![String::from("\"xi1'an1?\"")]);
    assert_eq!(builder.get_format(), &Format::IPASuperscript);
    assert_eq!(builder.get_latex_wrapper(), "UP");
}

#[test]
fn test_cli_tokenization() {
    let builder = Pinpatr::new()
        .format(Format::IPASuperscript)
        .text(vec![String::from("\"xi1'an1=chang2'an1?\"")])
        .wrapper("UP");

    let text: Vec<Token> = builder.tokenize().unwrap_or_else(|e| {
        eprintln!("Tokenization failed: {}", e);
        vec![]
    });

    assert_eq!(
        text,
        vec![
            Token::Punctuation("\"".to_string()),
            Token::Syllable(
                Syllable::new()
                    .full_syllable("xi1")
                    .onset(Some("x"))
                    .rhyme(String::from("i"))
                    .tone(Some(1))
            ),
            Token::Separator,
            Token::Syllable(Syllable {
                full: "an1".to_string(),
                initial: None,
                rhyme: String::from("an"),
                tone: Some(1)
            }),
            Token::Punctuation(String::from("=")),
            Token::Syllable(
                Syllable::new()
                    .full_syllable("chang2")
                    .onset(Some("ch"))
                    .rhyme(String::from("ang"))
                    .tone(Some(2))
            ),
            Token::Separator,
            Token::Syllable(Syllable {
                full: "an1".to_string(),
                initial: None,
                rhyme: String::from("an"),
                tone: Some(1)
            }),
            Token::Punctuation("?".to_string()),
            Token::Punctuation("\"".to_string()),
        ]
    );
}

#[test]
fn test_cli_parser() {
    let decomposed_u = "ǔ";
    let compose = decomposed_u.nfc().collect::<String>();
    assert_eq!("ǔ", &compose[..]);
}

#[test]
fn test_cli_function() {
    let builder = Pinpatr::new()
        .format(Format::IPASuperscript)
        .text(vec![
            String::from("zhe4 shi4 yi2ge0 ce4shi4,"),
            String::from("yi ya yang yu yue yuan,"),
            String::from("zher4 shi4 nar3"),
        ])
        .wrapper("UP");
    let tokens: Vec<Token> = builder.tokenize().unwrap_or_else(|e| {
        eprintln!("Tokenization failed: {}", e);
        vec![]
    });
    let output: String = builder.transform(tokens).unwrap_or_else(|e| {
        eprintln!("Transformation failed: {}", e);
        String::new()
    });

    assert_eq!(
        output,
        "tʂɤ⁵¹ ʂʅ⁵¹ i³⁵kɤ⁰ tsʰɤ⁵¹ʂʅ⁵¹, i jɑ jɑŋ y ɥœ ɥɛn, tʂɤʵ⁵¹ ʂʅ⁵¹ nɐʵ²¹⁴".to_string()
    );
}
