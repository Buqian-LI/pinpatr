pub mod cli;

pub use cli::token::Token;
pub use cli::Siphon;

use phf::phf_map;

const TONE_SUPERSCRIPT_DIGITS: [char; 6] = ['⁰', '¹', '²', '³', '⁴', '⁵'];

/// Mapping of vowels to their diacritic versions for each tone
/// 0123243
const TONE_DIACRITIC_MAP: [(&str, [&str; 4]); 8] = [
    ("a", ["ā", "á", "ǎ", "à"]),
    ("e", ["ē", "é", "ě", "è"]),
    ("o", ["ō", "ó", "ǒ", "ò"]),
    ("i", ["ī", "í", "ǐ", "ì"]),
    ("u", ["ū", "ú", "ǔ", "ù"]),
    ("ü", ["ǖ", "ǘ", "ǚ", "ǜ"]), // decompoased 'ü'
    // ("ü", ["ǖ", "ǘ", "ǚ", "ǜ"]), // precomposed 'ü' matching to decompoased variants
    ("ü", ["ǖ", "ǘ", "ǚ", "ǜ"]), // precomposed 'ü' matching to precomposed variants
    ("v", ["ǖ", "ǘ", "ǚ", "ǜ"]), // convenient 'v'
];

pub static INITIAL_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "b"     => "p"    ,
    "p"     => "pʰ"   ,
    "m"     => "m"    ,
    "f"     => "f"    ,
    "d"     => "t"    ,
    "t"     => "tʰ"   ,
    "n"     => "n"    ,
    "l"     => "l"    ,
    "g"     => "k"    ,
    "k"     => "kʰ"   ,
    "h"     => "x"    ,
    "j"     => "tɕ"   ,
    "q"     => "tɕʰ"  ,
    "x"     => "ɕ"    ,
    "zh"    => "tʂ"   ,
    "ch"    => "tʂʰ"  ,
    "sh"    => "ʂ"    ,
    "r"     => "ʐ"    ,
    "z"     => "ts"   ,
    "c"     => "tsʰ"  ,
    "s"     => "s"    ,
};

pub static RHYME_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "a"     => "ɑ"    ,
    "ai"    => "aj"   ,
    "ao"    => "ɑw"   ,
    "an"    => "an"   ,
    "ang"   => "ɑŋ"   ,

    "e"     => "ɤ"    , // ɰʌ
    "ei"    => "ej"   ,
    "en"    => "ən"   ,
    "eng"   => "əŋ"   ,

    "o"     => "wʌ"   , // wɔ
    "uo"    => "wʌ"   , // wɔ
    "ou"    => "ɤw"   ,
    "ong"   => "ʊŋ"   ,

    "i"     => "i"    ,
    "ia"    => "jɑ"   ,
    "iao"   => "jɑw"  ,
    "ie"    => "jɛ"   ,
    "iu"    => "jɤw"  ,
    "iou"   => "jɤw"  ,
    "ian"   => "jɛn"  ,
    "iang"  => "jɑŋ"  ,
    "in"    => "in"   ,
    "ing"   => "iŋ"   ,
    "iong"  => "jʊŋ"  ,

    "u"     => "u"    ,
    "ua"    => "wɑ"   ,
    "uai"   => "waj"  ,
    "uan"   => "wan"  ,
    "uang"  => "wɑŋ"  ,
    "ui"    => "wej"  ,
    "uei"   => "wej"  ,
    "un"    => "wən"  ,
    "uen"   => "wən"  ,
    "ueng"  => "wəŋ"  ,

    "ü"     => "y"    ,
    "v"     => "y"    ,
    "üe"    => "ɥœ"   ,
    "ve"    => "ɥœ"   ,
    "üan"   => "ɥɛn"  ,
    "van"   => "ɥɛn"  ,
    "ün"    => "yn"   ,
    "vn"    => "yn"   ,
    "üen"   => "yn"   ,

    "z"     => "ɿ"    ,
    "r"     => "ʅ"    ,

    // Erhua
    "ar"    => "ɐʵ"   ,
    "air"   => "ɐʵ"   ,
    "aor"   => "ɑʊʵ"  ,
    "anr"   => "ɐʵ"   ,
    "angr"  => "ɑ̃ʵ"   ,

    "er"    => "ɤʵ"   ,
    "eir"   => "ɚ"    ,
    "enr"   => "ɚ"    ,
    "engr"  => "ɤ̃ʵ"   ,

    "or"    => "wɔʵ"   ,
    "uor"   => "wɔʵ"  ,
    "our"   => "ɤʊʵ"  ,
    "ongr"  => "ʊ̃ʵ"   ,

    "ir"    => "jɚ"   ,
    "rr"    => "ɚ"    ,
    "zr"    => "ɚ"    ,
    "iar"   => "jɐʵ"  ,
    "iaor"  => "jɑʊʵ" ,
    "ier"   => "jɛʵ"  ,
    "iur"   => "jɤʊʵ" ,
    "iour"  => "jɤʊʵ" ,
    "ianr"  => "jɐʵ"  ,
    "iangr" => "jɑ̃ʵ"  ,
    "inr"   => "jɚ"   ,
    "ingr"  => "jɤ̃ʵ"  ,
    "iongr" => "jʊ̃ʵ"  ,

    "ur"    => "uʵ"   ,
    "uar"   => "wɐʵ"  ,
    "uair"  => "wɐʵ"  ,
    "uanr"  => "wɐʵ"  ,
    "uangr" => "wɑ̃ʵ"  ,
    "uir"   => "wɚ"   ,
    "ueir"  => "wɚ"   ,
    "unr"   => "wɚ"   ,
    "uenr"  => "wɚ"   ,
    "uengr" => "wɤ̃ʵ"  ,

    "ür"    => "ɥɚ"   ,
    "üer"   => "ɥœʵ"  ,
    "üanr"  => "ɥɐʵ"  ,
    "ünr"   => "ɥɚ"   ,
    "üenr"  => "ɥɚ"   ,

    "vr"    => "ɥɚ"   ,
    "ver"   => "ɥœʵ"  ,
    "vanr"  => "ɥɐʵ"  ,
    "vnr"   => "ɥɚ"   ,
};

// const INITIAL_MAPPING: [(&str, &str); 21] = [
//     ("b", "p"),
//     ("p", "pʰ"),
//     ("m", "m"),
//     ("f", "f"),
//     ("d", "t"),
//     ("t", "tʰ"),
//     ("n", "n"),
//     ("l", "l"),
//     ("g", "k"),
//     ("k", "kʰ"),
//     ("h", "x"),
//     ("j", "tɕ"),
//     ("q", "tɕʰ"),
//     ("x", "ɕ"),
//     ("zh", "tʂ"),
//     ("ch", "tʂʰ"),
//     ("sh", "ʂ"),
//     ("r", "ʐ"),
//     ("z", "ts"),
//     ("c", "tsʰ"),
//     ("s", "s"),
// ];
//
// const RHYME_MAPPING: [(&str, &str); 90] = [
//     // A
//     ("a", "ɑ"),
//     ("ai", "aj"),
//     ("ao", "ɑw"),
//     ("an", "an"),
//     ("ang", "ɑŋ"),
//     // E
//     ("e", "ɤ"), // ɰʌ
//     ("ei", "ej"),
//     ("en", "ən"),
//     ("eng", "əŋ"),
//     // O
//     ("o", "wʌ"),  // wɔ
//     ("uo", "wʌ"), // wɔ
//     ("ou", "ɤw"),
//     ("ong", "ʊŋ"),
//     // I
//     ("i", "i"),
//     ("ia", "jɑ"),
//     ("iao", "jɑw"),
//     ("ie", "jɛ"),
//     ("iu", "jɤw"),
//     ("iou", "jɤw"),
//     ("ian", "jɛn"),
//     ("iang", "jɑŋ"),
//     ("in", "in"),
//     ("ing", "iŋ"),
//     ("iong", "jʊŋ"),
//     // U
//     ("u", "u"),
//     ("ua", "wɑ"),
//     ("uai", "waj"),
//     ("uan", "wan"),
//     ("uang", "wɑŋ"),
//     ("ui", "wej"),
//     ("uei", "wej"),
//     ("un", "wən"),
//     ("uen", "wən"),
//     ("ueng", "wəŋ"),
//     // Ü
//     ("ü", "y"),
//     ("üe", "ɥœ"),
//     ("üan", "ɥɛn"),
//     ("ün", "yn"),
//     ("üen", "yn"),
//     ("v", "y"),
//     ("ve", "ɥœ"),
//     ("van", "ɥɛn"),
//     ("vn", "yn"),
//     // -I > z/r
//     ("z", "ɿ"),
//     ("r", "ʅ"),
//     // Erhua
//     // A
//     ("ar", "ɐʵ"),
//     ("air", "ɐʵ"),
//     ("aor", "ɑʊʵ"),
//     ("anr", "ɐʵ"),
//     ("angr", "ɑ̃ʵ"),
//     // E
//     ("er", "ɤʵ"),
//     ("eir", "ɚ"),
//     ("enr", "ɚ"),
//     ("engr", "ɤ̃ʵ"),
//     // O
//     ("or", "wɔʵ"),
//     ("uor", "wɔʵ"),
//     ("our", "ɤʊʵ"),
//     ("ongr", "ʊ̃ʵ"),
//     // I
//     ("ir", "jɚ"),
//     ("iar", "jɐʵ"),
//     ("iaor", "jɑʊʵ"),
//     ("ier", "jɛʵ"),
//     ("iur", "jɤʊʵ"),
//     ("iour", "jɤʊʵ"),
//     ("ianr", "jɐʵ"),
//     ("iangr", "jɑ̃ʵ"),
//     ("inr", "jɚ"),
//     ("ingr", "jɤ̃ʵ"),
//     ("iongr", "jʊ̃ʵ"),
//     // U
//     ("ur", "uʵ"),
//     ("uar", "wɐʵ"),
//     ("uair", "wɐʵ"),
//     ("uanr", "wɐʵ"),
//     ("uangr", "wɑ̃ʵ"),
//     ("uir", "wɚ"),
//     ("ueir", "wɚ"),
//     ("unr", "wɚ"),
//     ("uenr", "wɚ"),
//     ("uengr", "wɤ̃ʵ"),
//     // Ü
//     ("ür", "ɥɚ"),
//     ("üer", "ɥœʵ"),
//     ("üanr", "ɥɐʵ"),
//     ("ünr", "ɥɚ"),
//     ("üenr", "ɥɚ"),
//     ("vr", "ɥɚ"),
//     ("ver", "ɥœʵ"),
//     ("vanr", "ɥɐʵ"),
//     ("vnr", "ɥɚ"),
//     // -I > zr/rr
//     ("rr", "ɚ"),
//     ("zr", "ɚ"),
// ];
//
