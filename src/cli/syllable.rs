use crate::{cli::Format, INITIAL_MAP, RHYME_MAP, TONE_DIACRITIC_MAP, TONE_SUPERSCRIPT_DIGITS};
use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Syllable {
    pub full: String,
    pub initial: Option<String>,
    pub rhyme: String,
    pub tone: Option<usize>,
}

impl Syllable {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn full_syllable(mut self, syllable: impl Into<String>) -> Self {
        self.full = syllable.into();
        self
    }

    pub fn onset(mut self, onset: Option<impl Into<String>>) -> Self {
        self.initial = onset.map(|v| v.into());
        self
    }

    pub fn rhyme(mut self, rhyme: impl Into<String>) -> Self {
        self.rhyme = rhyme.into();
        self
    }

    pub fn tone(mut self, tone: Option<usize>) -> Self {
        self.tone = tone;
        self
    }

    /// Fully convert pinyin into IPA, but with the optional format:
    /// - LaTeX:
    ///     - \superscript{} (default) or any other customable wrapper
    /// - Unicode
    ///     - superscript numbers
    pub fn convert_to_ipa(&self, format: &Format, latex_wrapper: &str) -> Result<(String, String)> {
        // initial part
        let onset: String = if let Some(initial) = &self.initial {
            INITIAL_MAP
                .get(&initial.to_lowercase())
                .copied()
                .ok_or_else(|| anyhow!("The initial is not valid: {}", self.full))?
                .to_string()
        } else {
            String::new()
        };

        // rhyme part
        let rhyme: String = RHYME_MAP
            .get(&self.rhyme)
            .copied()
            .ok_or_else(|| anyhow!("The rhyme is not valid : {}", self.full))?
            .to_owned();

        // tone part
        let tone_to_transform: &str = if let Some(t) = self.tone {
            match t {
                0 | 5 => "0",
                1 => "55",
                2 => "35",
                3 => "214",
                4 => "51",
                _ => {
                    return Err(anyhow!(
                        "There are tones messed up in your input! -> {}",
                        self.full
                    ))
                }
            }
        } else {
            ""
        };

        let tone_transformed: String = match format {
            Format::IPASuperscript => self.tone_to_superscript(tone_to_transform),
            Format::IPALaTeX => {
                if !tone_to_transform.is_empty() {
                    format!(r"\{latex_wrapper}{{{tone_to_transform}}}")
                } else {
                    String::new()
                }
            }
            _ => String::new(), // never reachable
        };

        Ok((onset + &rhyme, tone_transformed))
    }

    /// Convert pinyin with numbers into pinyin with diacritics
    pub fn convert_to_pinyin(&self, format: &Format, wrapper: &str) -> Result<(String, String)> {
        match format {
            // keep the word, but change the tone
            Format::PinyinLaTeX => {
                let onset = self.initial.as_deref().unwrap_or_default();
                let mut input_word = format!("{}{}", onset, self.rhyme);
                if self.rhyme.contains("v") {
                    input_word = input_word.replace("v", "ü");
                }

                let word_transformed = input_word.replace("v", "ü");
                let tone_to_transform = self.transpose_tone_value(&input_word)?;
                let tone_transformed = if tone_to_transform.is_empty() {
                    String::new()
                } else {
                    format!(r"\{wrapper}{{{tone_to_transform}}}")
                };
                Ok((word_transformed, tone_transformed))
            }
            // change tone to diacritic or superscript
            Format::PinyinDiacritic => {
                let onset = self.initial.as_deref().unwrap_or_default();
                let word_transformed = format!("{}{}", onset, self.tone_to_diacritics(&self.full)?);
                Ok((word_transformed, String::new()))
            }
            Format::PinyinSuperscript => {
                let onset = self.initial.as_deref().unwrap_or_default();
                let word_transformed = format!("{}{}", onset, self.rhyme.replace("v", "ü"));
                let tone = self.transpose_tone_value(&word_transformed)?;
                let tone_transformed = self.tone_to_superscript(tone);
                Ok((word_transformed, tone_transformed))
            }
            _ => unreachable!(),
        }
    }

    fn transpose_tone_value(&self, word: &str) -> Result<&str> {
        match self.tone {
            Some(t) => match t {
                0 | 5 => Ok("0"),
                1 => Ok("55"),
                2 => Ok("35"),
                3 => Ok("214"),
                4 => Ok("51"),
                _ => Err(anyhow!(
                    "There are tones messed up in your input! -> {word}{t}"
                )),
            },
            None => Ok(""),
        }
    }

    /// transform 1-4 tones into actual value in superscript
    pub fn tone_to_superscript(&self, tone: &str) -> String {
        tone.chars()
            .map(|c| {
                if let Some(digit) = c.to_digit(10) {
                    TONE_SUPERSCRIPT_DIGITS[digit as usize]
                } else {
                    c // Keep non-numeric characters unchanged
                }
            })
            .collect()
    }

    /// transform 1-4 tones into diacritics on relevant vowels
    pub fn tone_to_diacritics(&self, word: &str) -> Result<String> {
        // Determine the tone index (0-3) or return the original rhyme if invalid
        let tone_index = match self.tone {
            Some(real_tone) if (1..=4).contains(&real_tone) => real_tone - 1,
            Some(real_tone) if real_tone == 0 || real_tone == 5 => return Ok(self.rhyme.clone()),
            _ => {
                return Err(anyhow!(
                    "There are tones messed up in your input! -> {}",
                    word,
                ))
            } // Tone 0, 5, or invalid: no diacritic
        };

        // Handle special case for "iu" first
        if self.rhyme.contains("iu") {
            return self.replace_vowel_with_diacritic("u", tone_index);
        }

        // Check priority vowels: a, e, o
        for vowel in ["a", "e", "o"] {
            if self.rhyme.contains(vowel) {
                return self.replace_vowel_with_diacritic(vowel, tone_index);
            }
        }

        // Check fallback vowels: i, u, ü, v
        for vowel in ["i", "u", "ü", "v"] {
            if self.rhyme.contains(vowel) {
                return self.replace_vowel_with_diacritic(vowel, tone_index);
            }
        }

        Ok(self.rhyme.clone())
    }

    /// Helper method to replace a vowel with its diacritic version
    fn replace_vowel_with_diacritic(&self, vowel: &str, tone_index: usize) -> Result<String> {
        if let Some(pos) = self.rhyme.find(vowel) {
            // Search through the array for the vowel
            for &(vowel_row, diacritics) in TONE_DIACRITIC_MAP.iter() {
                if vowel_row == vowel {
                    let diacritic = diacritics[tone_index];
                    let mut result = self.rhyme.clone();
                    result.replace_range(pos..pos + vowel.len(), diacritic);
                    return Ok(result);
                }
            }
        }
        Ok(self.rhyme.clone())
    }
}
