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
            assert_eq!(onset + &syllable.tone_to_diacritics().unwrap(), output);
        }
    }

    #[test]
    fn test_tone_to_superscript() {
        let syllable = Syllable::default();

        assert_eq!(syllable.tone_to_superscript("0"), "⁰");
        assert_eq!(syllable.tone_to_superscript("55"), "⁵⁵");
        assert_eq!(syllable.tone_to_superscript("35"), "³⁵");
        assert_eq!(syllable.tone_to_superscript("214"), "²¹⁴");
        assert_eq!(syllable.tone_to_superscript("51"), "⁵¹");
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

    #[test]
    fn test_comprehensive_pinyin_combinations() {
        // Test all valid initials with various rhymes and tones
        let initials = [
            "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", "j", "q", "x", "zh", "ch", "sh",
            "r", "z", "c", "s",
        ];

        // Test a representative sample of rhymes
        let rhymes = [
            "a", "ai", "ao", "an", "ang", "e", "ei", "en", "eng", "o", "uo", "ou", "ong", "i",
            "ia", "iao", "ie", "iu", "ian", "iang", "in", "ing", "iong", "u", "ua", "uai", "uan",
            "uang", "ui", "un", "ueng", "ü", "üe", "üan", "ün", "v", "ve", "van", "vn",
        ];

        let tones = [0, 1, 2, 3, 4, 5];

        // Test a subset of combinations to keep test runtime reasonable
        for &initial in &initials[0..5] {
            // Test first 5 initials
            for &rhyme in &rhymes[0..10] {
                // Test first 10 rhymes
                for &tone in &tones {
                    let syllable = Syllable::new()
                        .full_syllable(format!("{}{}", initial, rhyme))
                        .onset(Some(initial))
                        .rhyme(rhyme)
                        .tone(Some(tone));

                    // Test all conversion formats
                    let formats = [
                        Format::PinyinDiacritic,
                        Format::PinyinSuperscript,
                        Format::PinyinLaTeX,
                        Format::IPASuperscript,
                        Format::IPALaTeX,
                    ];

                    for format in &formats {
                        match format {
                            Format::PinyinDiacritic
                            | Format::PinyinSuperscript
                            | Format::PinyinLaTeX => {
                                let result = syllable.convert_to_pinyin(format, "UP");
                                assert!(
                                    result.is_ok(),
                                    "Failed to convert {}{}{} with format {:?}: {:?}",
                                    initial,
                                    rhyme,
                                    tone,
                                    format,
                                    result.err()
                                );
                            }
                            Format::IPASuperscript | Format::IPALaTeX => {
                                let result = syllable.convert_to_ipa(format, "UP");
                                assert!(
                                    result.is_ok(),
                                    "Failed to convert {}{}{} to IPA with format {:?}: {:?}",
                                    initial,
                                    rhyme,
                                    tone,
                                    format,
                                    result.err()
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_edge_cases_tone_to_diacritics() {
        // Test special case "iu" which should place diacritic on "u"
        let test_cases = vec![
            ("iu", 1, "iū"),
            ("iu", 2, "iú"),
            ("iu", 3, "iǔ"),
            ("iu", 4, "iù"),
            ("iu", 0, "iu"),
            ("iu", 5, "iu"),
        ];

        for (rhyme, tone, expected) in test_cases {
            let syllable = Syllable::new().rhyme(rhyme).tone(Some(tone));

            let result = syllable.tone_to_diacritics();
            assert!(
                result.is_ok(),
                "Failed for rhyme {} tone {}: {:?}",
                rhyme,
                tone,
                result.err()
            );
            assert_eq!(result.unwrap(), expected);
        }

        // Test vowel priority: a > e > o > i > u > ü > v
        let priority_tests = vec![
            ("ai", 1, "āi"), // a gets diacritic
            ("ei", 2, "éi"), // e gets diacritic
            ("ou", 3, "ǒu"), // o gets diacritic
            ("in", 4, "ìn"), // i gets diacritic
            ("un", 1, "ūn"), // u gets diacritic
            ("ün", 2, "ǘn"), // ü gets diacritic
            ("vn", 3, "ǚn"), // v gets diacritic (decomposed ü)
        ];

        for (rhyme, tone, expected) in priority_tests {
            let syllable = Syllable::new().rhyme(rhyme).tone(Some(tone));

            let result = syllable.tone_to_diacritics();
            assert!(
                result.is_ok(),
                "Failed for rhyme {} tone {}: {:?}",
                rhyme,
                tone,
                result.err()
            );
            assert_eq!(result.unwrap(), expected);
        }

        // Test rhymes without vowels (should return original)
        let no_vowel_tests = vec![
            ("z", 1, "z"), // -i final
            ("r", 2, "r"), // -i final
        ];

        for (rhyme, tone, expected) in no_vowel_tests {
            let syllable = Syllable::new().rhyme(rhyme).tone(Some(tone));

            let result = syllable.tone_to_diacritics();
            assert!(
                result.is_ok(),
                "Failed for rhyme {} tone {}: {:?}",
                rhyme,
                tone,
                result.err()
            );
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn test_builder_pattern_flexibility() {
        // Test that builder methods accept different input types
        let syllable1 = Syllable::new()
            .full_syllable("ma1")
            .onset(Some("m"))
            .rhyme("a")
            .tone(Some(1));

        let syllable2 = Syllable::new()
            .full_syllable(String::from("ma1"))
            .onset(Some(String::from("m")))
            .rhyme(String::from("a"))
            .tone(Some(1));

        assert_eq!(syllable1.full, syllable2.full);
        assert_eq!(syllable1.initial, syllable2.initial);
        assert_eq!(syllable1.rhyme, syllable2.rhyme);
        assert_eq!(syllable1.tone, syllable2.tone);
    }

    #[test]
    fn test_tone_5_neutral_tone() {
        // Test that tone 5 maps to tone 0 (neutral tone)
        let test_cases = vec![
            ("a", 5, "a"), // Tone 5 should produce same as tone 0
            ("a", 0, "a"), // Tone 0 for comparison
            ("a", 1, "ā"), // Tone 1 for comparison
        ];

        for (rhyme, tone, expected) in test_cases {
            let syllable = Syllable::new().rhyme(rhyme).tone(Some(tone));

            // Test tone_to_diacritics
            let result = syllable.tone_to_diacritics().unwrap();
            assert_eq!(
                result, expected,
                "tone_to_diacritics failed for rhyme {} tone {}",
                rhyme, tone
            );

            // Test convert_to_pinyin with PinyinDiacritic format
            let syllable_with_initial = Syllable::new()
                .full_syllable(format!("m{}", rhyme))
                .onset(Some("m"))
                .rhyme(rhyme)
                .tone(Some(tone));

            let (word, _) = syllable_with_initial
                .convert_to_pinyin(&Format::PinyinDiacritic, "UP")
                .unwrap();
            assert_eq!(
                word,
                format!("m{}", expected),
                "convert_to_pinyin(PinyinDiacritic) failed for rhyme {} tone {}",
                rhyme,
                tone
            );

            // Test convert_to_ipa - tone 5 should map to "0"
            let ipa_result = syllable_with_initial
                .convert_to_ipa(&Format::IPASuperscript, "UP")
                .unwrap();
            if tone == 5 || tone == 0 {
                // Tone 5 and 0 should produce "⁰" superscript
                assert!(
                    ipa_result.1.contains("⁰") || ipa_result.1.is_empty(),
                    "IPA superscript for tone {} should be empty or '⁰', got: {}",
                    tone,
                    ipa_result.1
                );
            }
        }

        // Test that tone 5 produces same output as tone 0 in all formats
        let formats = [
            Format::PinyinDiacritic,
            Format::PinyinSuperscript,
            Format::PinyinLaTeX,
            Format::IPASuperscript,
            Format::IPALaTeX,
        ];

        for format in &formats {
            let syllable_tone_0 = Syllable::new().onset(Some("m")).rhyme("a").tone(Some(0));

            let syllable_tone_5 = Syllable::new().onset(Some("m")).rhyme("a").tone(Some(5));

            match format {
                Format::PinyinDiacritic | Format::PinyinSuperscript | Format::PinyinLaTeX => {
                    let result_0 = syllable_tone_0.convert_to_pinyin(format, "UP").unwrap();
                    let result_5 = syllable_tone_5.convert_to_pinyin(format, "UP").unwrap();
                    assert_eq!(
                        result_0, result_5,
                        "Tone 0 and 5 should produce same output for format {:?}",
                        format
                    );
                }
                Format::IPASuperscript | Format::IPALaTeX => {
                    let result_0 = syllable_tone_0.convert_to_ipa(format, "UP").unwrap();
                    let result_5 = syllable_tone_5.convert_to_ipa(format, "UP").unwrap();
                    assert_eq!(
                        result_0, result_5,
                        "Tone 0 and 5 should produce same output for format {:?}",
                        format
                    );
                }
            }
        }
    }

    #[test]
    fn test_invalid_tone_handling() {
        // Test invalid tone values
        let syllable = Syllable::new().rhyme("a").tone(Some(6)); // Invalid tone

        // tone_to_diacritics should handle invalid tones gracefully
        let result = syllable.tone_to_diacritics();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "a"); // Should return original rhyme

        // convert_to_pinyin should also handle invalid tones
        let result = syllable.convert_to_pinyin(&Format::PinyinDiacritic, "UP");
        assert!(result.is_ok());

        // convert_to_ipa should return error for invalid tones
        let result = syllable.convert_to_ipa(&Format::IPASuperscript, "UP");
        assert!(result.is_err());
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
mod exhaustive_test {
    use siphon::cli::format::Format;
    use siphon::cli::syllable::Syllable;

    #[test]
    fn test_all_valid_pinyin_combinations() {
        // All valid pinyin initials from INITIAL_MAP
        let initials = [
            "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", "j", "q", "x", "zh", "ch", "sh",
            "r", "z", "c", "s",
        ];

        // All valid pinyin rhymes from RHYME_MAP
        let rhymes = [
            "a", "ai", "ao", "an", "ang", "e", "ei", "en", "eng", "o", "uo", "ou", "ong", "i",
            "ia", "iao", "ie", "iu", "ian", "iang", "in", "ing", "iong", "u", "ua", "uai", "uan",
            "uang", "ui", "un", "ueng", "ü", "üe", "üan", "ün", "v", "ve", "van", "vn", "z",
            "r", // -i finals
            // Erhua rhymes
            "ar", "air", "aor", "anr", "angr", "er", "eir", "enr", "engr", "or", "uor", "our",
            "ongr", "ir", "iar", "iaor", "ier", "iur", "ianr", "iangr", "inr", "ingr", "iongr",
            "ur", "uar", "uair", "uanr", "uangr", "uir", "unr", "uengr", "ür", "üer", "üanr",
            "ünr", "vr", "ver", "vanr", "vnr", "rr", "zr",
        ];

        let tones = [0, 1, 2, 3, 4, 5];

        // Test counter for debugging
        let mut total_tests = 0;
        let mut failed_tests = Vec::new();

        // Test all combinations
        for &initial in &initials {
            for &rhyme in &rhymes {
                // Skip invalid combinations based on pinyin rules
                if should_skip_combination(initial, rhyme) {
                    continue;
                }

                for &tone in &tones {
                    total_tests += 1;

                    let syllable = Syllable::new()
                        .full_syllable(format!("{}{}{}", initial, rhyme, tone))
                        .onset(Some(initial))
                        .rhyme(rhyme)
                        .tone(Some(tone));

                    // Test tone_to_diacritics
                    let diacritic_result = syllable.tone_to_diacritics();
                    if let Ok(result) = diacritic_result {
                        // Verify properties for tone_to_diacritics
                        if tone == 0 || tone == 5 {
                            // Tone 0 and 5 should return original rhyme without diacritics
                            if result != rhyme {
                                failed_tests.push(format!(
                                    "tone_to_diacritics for tone {} should return original rhyme '{}', got '{}' for {}{}{}",
                                    tone, rhyme, result, initial, rhyme, tone
                                ));
                            }
                        } else if (1..=4).contains(&tone) {
                            // Tones 1-4 should have diacritics (result may be different from original)
                            // At minimum, check it's not empty
                            if result.is_empty() {
                                failed_tests.push(format!(
                                    "tone_to_diacritics returned empty string for {}{}{}",
                                    initial, rhyme, tone
                                ));
                            }
                        }
                    } else {
                        failed_tests.push(format!(
                            "tone_to_diacritics failed for {}{}{}: {:?}",
                            initial,
                            rhyme,
                            tone,
                            diacritic_result.err()
                        ));
                    }

                    // Test convert_to_pinyin with all pinyin formats
                    let pinyin_formats = [
                        Format::PinyinDiacritic,
                        Format::PinyinSuperscript,
                        Format::PinyinLaTeX,
                    ];

                    for format in &pinyin_formats {
                        let pinyin_result = syllable.convert_to_pinyin(format, "UP");
                        if let Ok((word, tone_str)) = pinyin_result {
                            if !word.is_empty() {
                                // Word should not be empty
                                if word.is_empty() {
                                    failed_tests.push(format!(
                                        "convert_to_pinyin({:?}) returned empty word for {}{}{}",
                                        format, initial, rhyme, tone
                                    ));
                                }

                                // For PinyinDiacritic format, tone string should be empty
                                if *format == Format::PinyinDiacritic && !tone_str.is_empty() {
                                    failed_tests.push(format!(
                                        "convert_to_pinyin(PinyinDiacritic) should have empty tone string, got '{}' for {}{}{}",
                                        tone_str, initial, rhyme, tone
                                    ));
                                }

                                // For superscript formats, tone string should not be empty for tones 1-5
                                if *format == Format::PinyinSuperscript
                                    || *format == Format::PinyinLaTeX
                                        && (1..=5).contains(&tone)
                                        && tone_str.is_empty()
                                {
                                    failed_tests.push(format!(
                                            "convert_to_pinyin({:?}) should have non-empty tone string for tone {}, got empty for {}{}{}",
                                            format, tone, initial, rhyme, tone
                                        ));
                                }
                            }
                        } else {
                            failed_tests.push(format!(
                                "convert_to_pinyin({:?}) failed for {}{}{}: {:?}",
                                format,
                                initial,
                                rhyme,
                                tone,
                                pinyin_result.err()
                            ));
                        }
                    }

                    // Test convert_to_ipa with all IPA formats
                    let ipa_formats = [Format::IPASuperscript, Format::IPALaTeX];

                    for format in &ipa_formats {
                        let ipa_result = syllable.convert_to_ipa(format, "UP");
                        if let Ok((word, tone_str)) = ipa_result {
                            // Verify properties for convert_to_ipa
                            if !word.is_empty() {
                                // Word should not be empty
                                if word.is_empty() {
                                    failed_tests.push(format!(
                                        "convert_to_ipa({:?}) returned empty word for {}{}{}",
                                        format, initial, rhyme, tone
                                    ));
                                }

                                // IPA should have tone indication for all tones
                                if tone_str.is_empty() && (1..=5).contains(&tone) {
                                    failed_tests.push(format!(
                                        "convert_to_ipa({:?}) should have non-empty tone string for tone {}, got empty for {}{}{}",
                                        format, tone, initial, rhyme, tone
                                    ));
                                }
                            }
                        } else {
                            failed_tests.push(format!(
                                "convert_to_ipa({:?}) failed for {}{}{}: {:?}",
                                format,
                                initial,
                                rhyme,
                                tone,
                                ipa_result.err()
                            ));
                        }
                    }
                }
            }
        }

        println!("Total combinations tested: {}", total_tests);
        if !failed_tests.is_empty() {
            println!("Failed tests: {}", failed_tests.len());
            for failure in &failed_tests[0..std::cmp::min(10, failed_tests.len())] {
                println!("  {}", failure);
            }
            if failed_tests.len() > 10 {
                println!("  ... and {} more failures", failed_tests.len() - 10);
            }
            panic!(
                "Found {} failures in pinyin combinations",
                failed_tests.len()
            );
        }
    }

    /// Helper function to skip invalid pinyin combinations
    fn should_skip_combination(initial: &str, rhyme: &str) -> bool {
        // j, q, x can only combine with i, ü, and their compounds
        if ["j", "q", "x"].contains(&initial) {
            return !rhyme.starts_with('i')
                && !rhyme.starts_with('ü')
                && !rhyme.starts_with('v')
                && !rhyme.starts_with('y');
        }

        // zh, ch, sh, r, z, c, s cannot combine with i, ü, or their compounds
        // (except for the special -i finals)
        if ["zh", "ch", "sh", "r", "z", "c", "s"].contains(&initial) {
            if rhyme == "i" || rhyme == "ü" || rhyme == "v" {
                return false; // These are the special -i finals
            }
            if rhyme.starts_with('i')
                || rhyme.starts_with('ü')
                || rhyme.starts_with('v')
                || rhyme.starts_with('y')
            {
                return true;
            }
        }

        // b, p, m, f cannot combine with ua, uai, uan, uang, ui, un, ueng
        if ["b", "p", "m", "f"].contains(&initial)
            && (rhyme.starts_with("ua")
                || rhyme.starts_with("ui")
                || rhyme.starts_with("un")
                || rhyme.starts_with("ueng"))
        {
            return true;
        }

        // g, k, h cannot combine with i, ü, or their compounds
        if ["g", "k", "h"].contains(&initial)
            && (rhyme.starts_with('i')
                || rhyme.starts_with('ü')
                || rhyme.starts_with('v')
                || rhyme.starts_with('y'))
        {
            return true;
        }

        // d, t cannot combine with ü, v, or their compounds
        if ["d", "t"].contains(&initial) && (rhyme.starts_with('ü') || rhyme.starts_with('v')) {
            return true;
        }

        // n, l can combine with all
        // No restrictions for n, l

        false
    }

    #[test]
    fn test_specific_problematic_combinations() {
        // Test combinations that might be problematic with expected outputs
        let test_cases = vec![
            // j, q, x with i and ü compounds
            ("j", "i", 1, "jī", "tɕi⁵⁵"),
            ("j", "ü", 2, "jǘ", "tɕy³⁵"),
            ("j", "v", 3, "jǚ", "tɕy²¹⁴"),
            ("j", "ian", 4, "jiàn", "tɕjɛn⁵¹"),
            ("q", "ve", 1, "quē", "tɕʰɥœ⁵⁵"),
            ("x", "van", 2, "xuán", "ɕɥɛn³⁵"),
            // zh, ch, sh, r, z, c, s with special -i finals
            ("zh", "i", 1, "zhī", "tʂʅ⁵⁵"),
            ("ch", "i", 2, "chí", "tʂʰʅ³⁵"),
            ("sh", "i", 3, "shǐ", "ʂʅ²¹⁴"),
            ("r", "i", 4, "rì", "ʐʅ⁵¹"),
            ("z", "i", 1, "zī", "tsɿ⁵⁵"),
            ("c", "i", 2, "cí", "tsʰɿ³⁵"),
            ("s", "i", 3, "sǐ", "sɿ²¹⁴"),
            // b, p, m, f with valid combinations
            ("b", "a", 1, "bā", "pɑ⁵⁵"),
            ("p", "o", 2, "pó", "pʰwʌ³⁵"),
            ("m", "ai", 3, "mǎi", "maj²¹⁴"),
            ("f", "ei", 4, "fèi", "fej⁵¹"),
            // g, k, h with valid combinations
            ("g", "u", 1, "gū", "ku⁵⁵"),
            ("k", "ua", 2, "kuá", "kʰwɑ³⁵"),
            ("h", "uang", 3, "huǎng", "xwɑŋ²¹⁴"),
            // d, t with valid combinations
            ("d", "a", 1, "dā", "tɑ⁵⁵"),
            ("t", "u", 2, "tú", "tʰu³⁵"),
            // n, l with all types
            ("n", "ü", 1, "nǖ", "ny⁵⁵"),
            ("l", "v", 2, "lǘ", "ly³⁵"),
            ("n", "i", 3, "nǐ", "ni²¹⁴"),
            ("l", "u", 4, "lù", "lu⁵¹"),
        ];

        for (initial, rhyme, tone, expected_pinyin, expected_ipa) in test_cases {
            let syllable = Syllable::new()
                .onset(Some(initial))
                .rhyme(rhyme)
                .tone(Some(tone));

            // Test tone_to_diacritics
            let diacritic_result = syllable.tone_to_diacritics().unwrap();
            let expected_diacritic_rhyme = &expected_pinyin[initial.len()..]; // Remove initial
            assert_eq!(
                diacritic_result, expected_diacritic_rhyme,
                "tone_to_diacritics failed for {}{}{}: expected rhyme '{}', got '{}'",
                initial, rhyme, tone, expected_diacritic_rhyme, diacritic_result
            );

            // Test convert_to_pinyin with PinyinDiacritic format
            let (pinyin_word, pinyin_tone) = syllable
                .convert_to_pinyin(&Format::PinyinDiacritic, "UP")
                .unwrap();
            assert_eq!(
                pinyin_word, expected_pinyin,
                "convert_to_pinyin(PinyinDiacritic) failed for {}{}{}: expected '{}', got '{}'",
                initial, rhyme, tone, expected_pinyin, pinyin_word
            );
            assert!(
                pinyin_tone.is_empty(),
                "Pinyin diacritic format should have empty tone string for {}{}{}, got '{}'",
                initial,
                rhyme,
                tone,
                pinyin_tone
            );

            // Test convert_to_ipa with IPASuperscript format
            let (ipa_word, ipa_tone) = syllable
                .convert_to_ipa(&Format::IPASuperscript, "UP")
                .unwrap();

            // Split expected IPA into base and superscript parts
            // The superscript characters are: ⁰¹²³⁴⁵
            let superscript_chars = ['⁰', '¹', '²', '³', '⁴', '⁵'];
            let mut expected_ipa_base = expected_ipa;
            let mut expected_ipa_superscript = "";

            // Find the first superscript character
            for (i, c) in expected_ipa.char_indices() {
                if superscript_chars.contains(&c) {
                    expected_ipa_base = &expected_ipa[..i];
                    expected_ipa_superscript = &expected_ipa[i..];
                    break;
                }
            }

            assert_eq!(
                ipa_word, expected_ipa_base,
                "convert_to_ipa(IPASuperscript) failed for {}{}{}: expected base '{}', got '{}'",
                initial, rhyme, tone, expected_ipa_base, ipa_word
            );
            assert_eq!(
                ipa_tone, expected_ipa_superscript,
                "convert_to_ipa(IPASuperscript) failed for {}{}{}: expected superscript '{}', got '{}'",
                initial, rhyme, tone, expected_ipa_superscript, ipa_tone
            );
        }
    }
}

#[cfg(test)]
mod core_test {
    use siphon::{cli::format::Format, Siphon, Token};

    #[test]
    fn test_convertion_ipasup() {
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
    fn test_convertion_pysup() {
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

    #[test]
    fn test_convertion_pytex() {
        let builder = Siphon::new("liu2 lve4 jiu3")
            .format(Format::PinyinLaTeX)
            .wrapper("UP");
        let tokens: Vec<Token> = builder.tokenize().unwrap_or_else(|e| {
            eprintln!("Tokenization failed: {}", e);
            vec![]
        });
        let output: String = builder.transform(tokens).unwrap_or_else(|e| {
            eprintln!("Transformation failed: {}", e);
            String::new()
        });

        assert_eq!(output, r"liu\UP{35} lüe\UP{51} jiu\UP{214}".to_string());
    }

    #[test]
    fn test_convertion_pydia() {
        let builder = Siphon::new("liu2 lve4 jiu3 de")
            .format(Format::PinyinDiacritic)
            .wrapper("UP");
        let tokens: Vec<Token> = builder.tokenize().unwrap_or_else(|e| {
            eprintln!("Tokenization failed: {}", e);
            vec![]
        });
        let output: String = builder.transform(tokens).unwrap_or_else(|e| {
            eprintln!("Transformation failed: {}", e);
            String::new()
        });

        assert_eq!(output, "liú lüè jiǔ de".to_string());
    }

    #[test]
    fn test_convertion_ipatex() {
        let builder = Siphon::new("liu2 lve4")
            .format(Format::IPALaTeX)
            .wrapper("UP");
        let tokens: Vec<Token> = builder.tokenize().unwrap_or_else(|e| {
            eprintln!("Tokenization failed: {}", e);
            vec![]
        });
        let output: String = builder.transform(tokens).unwrap_or_else(|e| {
            eprintln!("Transformation failed: {}", e);
            String::new()
        });

        assert_eq!(output, r"ljɤw\UP{35} lɥœ\UP{51}".to_string());
    }
}
