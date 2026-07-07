use std::hash::Hash;

// 1. Define the Type-Safe Newtype (equivalent to Nominal in TS)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlugifiedString(String);

// impl SlugifiedString {
//     pub fn value(&self) -> String {
//         self.0.clone()
//     }
// }

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

pub fn slugify(s: &str) -> SlugifiedString {
    SlugifiedString(s.to_lowercase())
}
