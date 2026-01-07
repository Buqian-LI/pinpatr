use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum Format {
    /// in Pinyin with diacritics (i.e. zhě)
    ///    (aliases: pydia, pinyindia, diacritic, pinyindiacritic)
    #[value(
        name = "dia",
        alias = "pydia",
        alias = "pinyindia",
        alias = "diacritic",
        alias = "pinyindiacritic",
        verbatim_doc_comment
    )]
    PinyinDiacritic,
    /// in Pinyin with numbers wrapped in LaTeX command (i.e. zhe²¹⁴)
    ///    (aliases: sup, pysup, pysuper, pinyinsuper, pysuperscript, pinyinsuperscript)
    #[value(
        name = "sup",
        alias = "pysup",
        alias = "pysuper",
        alias = "pinyinsuper",
        alias = "pysuperscript",
        alias = "pinyinsuperscript",
        verbatim_doc_comment
    )]
    PinyinSuperscript,
    /// in Pinyin with numbers wrapped in LaTeX command (i.e. zhe\textsuperscript{214})
    ///    (aliases: pynum, number, pylatex, pinyinlatex)
    #[value(
        name = "pytex",
        alias = "number",
        alias = "pynum",
        alias = "pylatex",
        alias = "pinyinlatex",
        verbatim_doc_comment
    )]
    PinyinLaTeX,
    /// in IPA with number wrapped in LaTeX command (i.e. tʂɤ\textsuperscript{214})
    ///    (aliases: ipa, ipatex, ipalatex, tex, latex)
    #[value(
        name = "ipa",
        alias = "tex",
        alias = "latex",
        alias = "ipatex",
        alias = "ipalatex",
        verbatim_doc_comment
    )]
    IPALaTeX,
    /// in IPA with superscript number (i.e. tʂɤ²¹⁴)
    ///    (aliases: ipasup, super, ipasuper, ipasuperscript)
    #[value(
        name = "ipasup",
        alias = "super",
        alias = "ipasuper",
        alias = "ipasuperscript",
        verbatim_doc_comment
    )]
    IPASuperscript,
}
