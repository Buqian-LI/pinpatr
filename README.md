# `siphon`

A CLI tool for Chinese Pinyin and IPA conversion.

## Arguments

### Format (-f, --format \<FORMAT\>)

#### `dia` in Pinyin with diacritics

(aliases: pydia, pinyindia, diacritic, pinyindiacritic)

for example:

```bash
    zhě
```

#### `num` in Pinyin with numbers wrapped in LaTeX command

(aliases: pynum, pylatex, pinyinlatex, number)

for example:

```bash
    zhe\textsuperscript{214}
```

#### `ipa` in IPA with number wrapped in LaTeX command

- (aliases: ipa, ipatex, ipalatex, tex, latex)

for example:

```bash
    tʂɤ\textsuperscript{214}
```

#### `sup` in IPA with superscript number

- (aliases: ipasup, ipasuper, super, superscript)

for example:

```bash
    tʂɤ²¹⁴
```

### Wrapper (-r, --wrap \<LATEX_WRAPPER\>)

Custom LaTeX wrapper command (aliases: `wrapper`, `latex`, `latexwrapper`, `latex-wrapper`)

default: textsuperscript

>Note: Only the command name part will be replaced.

## Caveat

For the phoneme ü, you can input either 'v', the decomposed form 'ü' (u +  ̈), or the pre-composed form 'ü'.

The output will always use the decomposed form, as most fonts prioritize support for decomposed characters over pre-composed ones. This ensures consistent rendering across systems.
