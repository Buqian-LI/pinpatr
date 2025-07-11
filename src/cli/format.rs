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
    /// in Pinyin with numbers wrapped in LaTeX command (i.e. zhe\textsuperscript{214})
    ///    (aliases: pynum, pylatex, pinyinlatex, number)
    #[value(
        name = "num",
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
        alias = "ipatex",
        alias = "ipalatex",
        alias = "tex",
        alias = "latex",
        verbatim_doc_comment
    )]
    IPALaTeX,
    /// in IPA with superscript number (i.e. tʂɤ²¹⁴)
    ///    (aliases: ipasup, ipasuper, super, superscript)
    #[value(
        name = "sup",
        alias = "ipasup",
        alias = "ipasuper",
        alias = "super",
        alias = "superscript",
        verbatim_doc_comment
    )]
    IPASuperscript,
}
