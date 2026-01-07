use crate::{cli::Format, INITIAL_MAP, RHYME_MAP, TONE_DIACRITIC_MAP};
use anyhow::Result;

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

    pub fn full_syllable(mut self, syllable: &str) -> Self {
        self.full = syllable.to_string();
        self
    }

    pub fn onset(mut self, onset: Option<&str>) -> Self {
        self.initial = onset.map(|v| v.to_string());
        self
    }

    pub fn rhyme(mut self, rhyme: String) -> Self {
        self.rhyme = rhyme;
        self
    }

    pub fn tone(mut self, tone: Option<usize>) -> Self {
        self.tone = tone;
        self
    }

    /// Fully convert pinyin into IPA, but with the optional format:
    /// - LaTeX:
    ///     - \UP{} or \superscript{}
    /// - Unicode
    ///     - normal or superscript numbers
    pub fn convert_to_ipa(&self, format: &Format, latex_wrapper: &str) -> Result<(String, String)> {
        // initial part
        let onset: String = if let Some(initial) = &self.initial {
            INITIAL_MAP
                .get(&initial.to_lowercase())
                .copied()
                .ok_or_else(|| anyhow::anyhow!("The initial is not valid: {}", self.full))?
                .to_string()
        } else {
            String::new()
        };

        // rhyme part
        let rhyme: String = RHYME_MAP
            .get(&self.rhyme)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("The rhyme is not valid : {}", self.full))?
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
                    return Err(anyhow::anyhow!(
                        "There are tones messed up in your input! -> {}",
                        self.full
                    ))
                }
            }
        } else {
            ""
        };

        let tone_transformed: String = match &format {
            Format::IPASuperscript => self.tone_to_superscript(tone_to_transform)?,
            Format::IPALaTeX => {
                if !tone_to_transform.is_empty() {
                    format!(r"\{latex_wrapper}{{{tone_to_transform}}}")
                } else {
                    Default::default()
                }
            }
            _ => String::new(), // never reachable
        };

        Ok((onset + &rhyme, tone_transformed))
    }

    /// Convert pinyin with numbers into pinyin with diacritics
    pub fn convert_to_pinyin(&self, format: &Format, wrap: &str) -> Result<(String, String)> {
        // initialization

        let onset = self.initial.clone().unwrap_or_default();

        let mut word_transformed = String::new();
        let mut tone_transformed = String::new();

        match format {
            // keep the word, but change the tone
            Format::PinyinLaTeX => {
                // treat words
                let word = onset.clone() + &self.rhyme;
                word_transformed = word.replace("v", "ü");
                // treat tones
                let tone_to_transform: &str = self.transpose_tone_value(&word)?;
                tone_transformed = if tone_to_transform.is_empty() {
                    String::new()
                } else {
                    format!(r"\{wrap}{{{tone_to_transform}}}")
                };
            }
            // change tone to diacritic or superscript
            Format::PinyinDiacritic => {
                word_transformed = onset + &self.tone_to_diacritics();
                // leave tone_transformed to be empty
                // tone_transformed = "".to_string();
            }
            Format::PinyinSuperscript => {
                word_transformed = onset + &self.rhyme;
                let tone = self.transpose_tone_value(&word_transformed)?;
                tone_transformed = self.tone_to_superscript(tone)?;
                // tone_transformed = "".to_string();
            }
            _ => {}
        };

        Ok((word_transformed, tone_transformed))
    }

    fn transpose_tone_value(&self, word: &str) -> Result<&str> {
        match self.tone {
            Some(t) => match t {
                0 => Ok("0"),
                1 => Ok("55"),
                2 => Ok("35"),
                3 => Ok("214"),
                4 => Ok("51"),
                5 => Ok("0"),
                _ => Err(anyhow::anyhow!(
                    "There are tones messed up in your input! -> {word}{t}"
                )),
            },
            None => Ok(""),
        }
    }

    /// transform 1-4 tones into actual value in superscript
    pub fn tone_to_superscript(&self, tone: &str) -> Result<String> {
        let out: String = tone
            .chars()
            .map(|c| {
                if let Some(digit) = c.to_digit(10) {
                    ['⁰', '¹', '²', '³', '⁴', '⁵'][digit as usize]
                } else {
                    c // Keep non-numeric characters unchanged
                }
            })
            .collect::<String>();
        Ok(out)
    }

    /// transform 1-4 tones into diacritics on relevant vowels
    pub fn tone_to_diacritics(&self) -> String {
        // Determine the tone index (0-4) or return the original rhyme if invalid
        let vowels_with_diacritics_index = match self.tone {
            Some(real_tone) if (1..=5).contains(&real_tone) => real_tone - 1,
            _ => return self.rhyme.clone(),
        };

        // Extract vowels directly from TONE_MAPPING
        let vowels: Vec<&str> = TONE_DIACRITIC_MAP.iter().map(|&(vowel, _)| vowel).collect();

        // Define priority and fallback vowels
        let priority_vowels: &[&str] = &vowels[0..3]; // 'a', 'e', 'o'
        let fallback_vowels: &[&str] = &vowels[3..]; // 'i', 'u', 'ü' and 'v'

        // Find the target vowel based on priority
        // Handle special cases for "iu"
        let target_vowel: Option<&str> = if self.rhyme.contains("iu") {
            Some("u")
        } else {
            priority_vowels
                .iter()
                .chain(fallback_vowels.iter())
                .find(|&&v| self.rhyme.contains(v))
                .cloned()
        };

        // Replace the target vowel with the diacritic version
        if let Some(vowel) = target_vowel {
            if let Some(pos) = self.rhyme.find(vowel) {
                // Look up the TONE_DIACRITIC_MAP
                let diacritic = TONE_DIACRITIC_MAP
                    .iter()
                    .find(|&&(vowel_row, _)| vowel_row == vowel) // find the
                    // correct row
                    .map(|&(_, vowels_with_diacritics)| {
                        vowels_with_diacritics[vowels_with_diacritics_index]
                    }) // find the
                    // correct cell
                    .unwrap_or("");
                let mut word_with_diacritic = self.rhyme.clone();
                word_with_diacritic.replace_range(pos..pos + vowel.len(), diacritic);
                return word_with_diacritic;
            }
        }

        self.rhyme.clone() // Return the original word if no vowel is found (target_vowel == None)
    }
}
