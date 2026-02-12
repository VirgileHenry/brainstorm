#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct SagaLayout {
    pub mana_cost: Option<crate::ability_tree::terminals::ManaCost>,
    pub card_type: crate::card::card_type::CardType,
    pub chapters: arrayvec::ArrayVec<crate::ability_tree::ability::saga_chapter::SagaChapter, 4>,
}

impl super::LayoutImpl for SagaLayout {
    fn card_types(&self) -> arrayvec::ArrayVec<mtg_data::CardType, 4> {
        self.card_type.card_types()
    }

    fn mana_value(&self) -> usize {
        self.mana_cost.as_ref().map(|cost| cost.mana_value()).unwrap_or(0)
    }

    fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String> {
        use std::str::FromStr;

        let ability_tree = match raw_card.oracle_text.as_ref() {
            Some(oracle_text) => crate::AbilityTree::from_oracle_text(oracle_text, &raw_card.name)
                .map_err(|e| format!("Failed to parse oracle text to ability tree: {e}"))?,
            None => crate::AbilityTree::empty(),
        };
        let mut chapters = arrayvec::ArrayVec::new();
        for ability in ability_tree.abilities.into_iter() {
            match ability {
                crate::ability_tree::ability::Ability::SagaChapter(chapter) => chapters.push(chapter),
                other => {
                    return Err(format!("Invalid ability for Saga: expected Chapter, found {other:?}"));
                }
            }
        }
        Ok(SagaLayout {
            mana_cost: match raw_card.mana_cost.as_ref() {
                Some(mana_cost) => Some(
                    crate::ability_tree::terminals::ManaCost::from_str(mana_cost)
                        .map_err(|e| format!("Failed to parse mana cost: {e}"))?,
                ),
                None => None,
            },
            card_type: crate::card::card_type::CardType::parse(&raw_card.type_line, raw_card)
                .map_err(|e| format!("Failed to parse card type: {e}"))?,
            chapters,
        })
    }

    fn display<W: std::io::Write>(&self, _output: &mut W) -> std::io::Result<()> {
        unimplemented!()
    }
}
