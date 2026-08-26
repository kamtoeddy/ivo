use std::{hash::Hash, sync::LazyLock};

use regex::Regex;
use unicode_normalization::UnicodeNormalization;

// 1. Define the Type-Safe Newtype (equivalent to Nominal in TS)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SlugifiedString(String);

// Optional: Implement Display so it prints like a regular string
impl std::fmt::Display for SlugifiedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Hash for SlugifiedString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<&str> for SlugifiedString {
    fn from(value: &str) -> Self {
        slugify(value)
    }
}

impl From<String> for SlugifiedString {
    fn from(value: String) -> Self {
        slugify(&value)
    }
}

impl From<SlugifiedString> for String {
    fn from(value: SlugifiedString) -> Self {
        value.0
    }
}

static RE_ACCENTS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\u{0300}-\u{036f}]").unwrap());
static RE_SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
static RE_SPECIAL_CHARS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^a-zA-Z0-9_~\s]").unwrap());
static RE_MINUSES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-+").unwrap());

pub fn slugify(s: &str) -> SlugifiedString {
    // .toLowerCase() & .normalize('NFD')
    let normalized: String = s.to_lowercase().nfd().collect();
    // let normalized: String = s.to_lowercase();

    // .replace(/[\u0300-\u036f]/g, '')
    let no_accents = RE_ACCENTS.replace_all(&normalized, "");

    // .replace(regexToCaptureAllSpaces, ' ')
    let space_cleaned = RE_SPACES.replace_all(&no_accents, " ");

    // .replace(regexToCaptureAllSpecialCharsExceptSpaces, ' ')
    let special_cleaned = RE_SPECIAL_CHARS.replace_all(&space_cleaned, " ");

    // .trim()
    let trimmed = special_cleaned.trim();

    // .replace(regexToCaptureAllSpaces, '-')
    let hiphenated = RE_SPACES.replace_all(trimmed, "-");

    // .replace(regexToCaptureAllMinuses, '-')
    let final_slug = RE_MINUSES.replace_all(&hiphenated, "-");

    // Return wrapped in our type-safe struct
    SlugifiedString(final_slug.into_owned())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slugify() {
        let data = vec![
            ("John doe", "john-doe".into()),
            (" ?Crème  Brûlée & Cafe!!!? #*", "creme-brulee-cafe".into()),
        ];

        for (input, slug) in data.iter() {
            assert_eq!(slugify(input), *slug);
        }

        assert!(data.len() == 2)
    }
}
