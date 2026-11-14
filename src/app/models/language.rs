use std::fmt::Display;
use toasty::Embed;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Embed)]
pub enum Language {
    De,
    En
}

impl Default for Language {
    fn default() -> Self {
        Language::De
    }
}

impl From<String> for Language {
    fn from(s: String) -> Self {
        match s.as_str() {
            "de" => Language::De,
            "en" => Language::En,
            _ => Language::De,
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Language::De => "de".to_string(),
            Language::En => "en".to_string(),
        };
        write!(f, "{}", str)
    }
}