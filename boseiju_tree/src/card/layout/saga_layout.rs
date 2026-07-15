#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SagaLayout {
    pub mana_cost: Option<crate::ability_tree::terminals::ManaCost>,
    pub card_type: crate::ability_tree::type_line::TypeLine,
    pub chapters: crate::utils::HeapArrayVec<SagaChapter, 4>,
    #[cfg(feature = "spanned_tree")]
    pub span: crate::ability_tree::span::TreeSpan,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SagaChapter {
    chapter: usize,
    effect: crate::ability_tree::ability::spell::SpellAbility,
}

impl super::LayoutImpl for SagaLayout {
    fn card_types(&self) -> crate::ability_tree::type_line::SimplifiedCardTypes {
        (&self.card_type).into()
    }

    fn mana_value(&self) -> usize {
        self.mana_cost.as_ref().map(|cost| cost.mana_value()).unwrap_or(0)
    }

    #[cfg(feature = "parser")]
    fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String> {
        // let ability_tree = match raw_card.oracle_text.as_ref() {
        //     Some(oracle_text) => crate::AbilityTree::from_oracle_text(oracle_text, &raw_card.name)
        //         .map_err(|e| format!("Failed to parse oracle text to ability tree: {e}"))?,
        //     None => crate::AbilityTree::empty(),
        // };

        use crate::lexer::IntoToken;
        let chapters = crate::utils::HeapArrayVec::new();

        /* Fixme: the parser shall be able to parse into any target node I think ? That would be cool */
        /* And then the SagaChapters node shall exist, and for the saga layout we can use this ? */

        let type_line_span = crate::lexer::Span {
            start: 0,
            length: raw_card.type_line.len(),
            text: raw_card.type_line.as_str(),
        };

        Ok(SagaLayout {
            mana_cost: match raw_card.mana_cost.as_ref() {
                Some(mana_cost) => {
                    use crate::lexer::IntoToken;

                    let mana_cost_span = crate::lexer::Span {
                        start: 0,
                        length: mana_cost.len(),
                        text: mana_cost,
                    };
                    Some(
                        crate::ability_tree::terminals::ManaCost::try_from_span(&mana_cost_span)
                            .ok_or_else(|| format!("Failed to parse mana cost from: {mana_cost}"))?,
                    )
                }
                None => None,
            },

            card_type: crate::ability_tree::type_line::TypeLine::try_from_span(&type_line_span)
                .ok_or_else(|| format!("Failed to parse card type: {}", raw_card.type_line))?,
            chapters,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        })
    }

    fn layout_debug_display<W: std::io::Write>(&self, _output: &mut W) -> std::io::Result<()> {
        unimplemented!()
    }
}
