pub fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'y' | 'o' | 'u' | 'ø' => true,
        'á' | 'æ' | 'í' | 'ý' | 'ó' | 'ú' => true,
        _ => false,
    }
}

pub fn skerping(vowel: char) -> Option<&'static str> {
    match vowel {
        'ó' | 'ú' => Some("gv"),
        'í' | 'ý' | 'y' | 'i' => Some("ggj"),
        _ => None,
    }
}

pub fn dash(c: char) -> bool {
    matches!(c, '-' | '~')
}