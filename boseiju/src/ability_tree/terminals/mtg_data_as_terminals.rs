mod color;
mod mana;
mod types;

use crate::ability_tree::terminals::Terminal;

fn from_str_singular_or_plural<T: std::str::FromStr>(source: &str) -> Option<T> {
    if let Ok(value) = T::from_str(source) {
        return Some(value);
    } else if let Some(singular) = source.strip_suffix('s') {
        if let Ok(value) = T::from_str(singular) {
            return Some(value);
        }
    }
    None
}

impl Terminal for mtg_data::KeywordAction {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}

impl Terminal for mtg_data::AbilityWord {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        from_str_singular_or_plural::<Self>(source)
    }
}
