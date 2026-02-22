use crate::ability_tree::terminals::Terminal;

impl Terminal for mtg_data::KeywordAction {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        use std::str::FromStr;
        Self::from_str(source).ok()
    }
}

impl Terminal for mtg_data::AbilityWord {
    #[cfg(feature = "lexer")]
    fn try_from_str(source: &str) -> Option<Self> {
        use std::str::FromStr;
        Self::from_str(source).ok()
    }
}
