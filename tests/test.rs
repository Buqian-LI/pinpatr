#[cfg(test)]
mod syllabel_test {
    use siphon::cli::format::Format;
    use siphon::cli::syllable::Syllable;

    #[test]
    fn test_tone_to_diacritics() {
        let mut syllable = Syllable {
            full: String::from("zhe1"),
            initial: Some(String::from("zh")),
            rhyme: String::from("e"),
            tone: Some(1),
        };

        let test_cases = [(0, "zhe"), (1, "zhē"), (2, "zhé"), (3, "zhě"), (4, "zhè")];

        for &(tone, output) in &test_cases {
            let onset = syllable.initial.as_ref().unwrap().clone();
            syllable.tone = Some(tone);
            assert_eq!(onset + &syllable.tone_to_diacritics(), output);
        }
    }

    #[test]
    fn test_tone_to_superscript() {
        let syllable = Syllable::default();

        assert_eq!(syllable.tone_to_superscript("0").unwrap(), "⁰");
        assert_eq!(syllable.tone_to_superscript("55").unwrap(), "⁵⁵");
        assert_eq!(syllable.tone_to_superscript("35").unwrap(), "³⁵");
        assert_eq!(syllable.tone_to_superscript("214").unwrap(), "²¹⁴");
        assert_eq!(syllable.tone_to_superscript("51").unwrap(), "⁵¹");
    }

    #[test]
    fn test_convert_to_pinyin() {
        let mut syllable = Syllable {
            full: String::from("diu1"),
            initial: Some(String::from("d")),
            rhyme: String::from("iu"),
            tone: None, // Start with no tone
        };

        let test_cases = [(0, "diu"), (1, "diū"), (2, "diú"), (3, "diǔ"), (4, "diù")];

        for &(tone_value, expected) in &test_cases {
            syllable.tone = Some(tone_value);
            let (word, tone) = syllable
                .convert_to_pinyin(&Format::PinyinDiacritic, "UP")
                .unwrap();
            assert_eq!(word + &tone, expected);
        }
    }
}

#[cfg(test)]
mod cli_test {
    use siphon::cli::format::Format;
    use siphon::cli::syllable::Syllable;
    use siphon::cli::token::Token;
    use siphon::cli::Siphon;
    use unicode_normalization::UnicodeNormalization;

    #[test]
    fn test_default_cli() {
        let args = Siphon::default();

        assert!(!args.get_debug());
        assert_eq!(args.get_format(), &Format::PinyinDiacritic);
        assert_eq!(args.get_latex_wrapper(), "textsuperscript");
    }

    #[test]
    fn test_new_cli() {
        let mut args = Siphon::new("\"xi1'an1?\"");

        args.set_format(Format::IPASuperscript);
        args.set_debug(true);

        assert_eq!(args.get_format(), &Format::IPASuperscript);
        assert!(args.get_debug());
    }

    #[test]
    fn test_cli_builder() {
        let builder = Siphon::new("\"xi1'an1?\"")
            .format(Format::IPASuperscript)
            .wrapper("UP");

        assert_eq!(builder.get_text(), String::from("\"xi1'an1?\""));
        assert_eq!(builder.get_format(), &Format::IPASuperscript);
        assert_eq!(builder.get_latex_wrapper(), "UP");
    }

    #[test]
    fn test_cli_tokenization() {
        let builder = Siphon::new("\"xi1'an1=chang2'an1?\"")
            .format(Format::IPASuperscript)
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
}

#[cfg(test)]
mod core_test {
    use siphon::{cli::format::Format, Siphon, Token};

    #[test]
    fn test_convertion_ipasuperscript() {
        let builder =
            Siphon::new("zhe4 shi4 yi2ge0 ce4shi4, yi ya yang yu yue yuan, zher4 shi4 nar3")
                .format(Format::IPASuperscript)
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

    #[test]
    fn test_convertion_pinyinsuperscript() {
        let builder =
            Siphon::new("zhe4 shi4 yi2ge0 ce4shi4, yi ya yang yu yue yuan, zher4 shi4 nar3")
                .format(Format::PinyinSuperscript)
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
            "zhe⁵¹ shi⁵¹ yi³⁵ge⁰ ce⁵¹shi⁵¹, yi ya yang yu yue yuan, zher⁵¹ shi⁵¹ nar²¹⁴"
                .to_string()
        );
    }
}
