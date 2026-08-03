impl crate::card::Parse for boseiju_tree::card::layout::TokenLayout {
    type Source = mtg_cardbase::Card;
    type Error = crate::card::layout::LayoutParseError;

    fn parse(from: &Self::Source) -> Result<Self, Self::Error> {
        let type_line = from.type_line.to_ascii_lowercase();

        Ok(Self {
            name: from.name.clone(),
            card_type: boseiju_tree::ability_tree::type_line::TypeLine::parse(&type_line)?,
            color_identity: boseiju_tree::ability_tree::colors::Colors::parse(&from.color_identity)?,
            abilities: match from.oracle_text.as_ref() {
                Some(oracle_text) => {
                    let preprocessed = boseiju_lexer::preprocess(&from.name, oracle_text);
                    let tokens = boseiju_lexer::lex(&preprocessed)?;
                    let abilities = crate::parse_ability_tree(tokens.as_slice())?;
                    abilities
                }
                None => boseiju_tree::AbilityTree::empty(),
            },
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }
}
