#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct TokenLayout {
    pub name: String,
    pub card_type: crate::card::card_type::CardType,
    pub abilities: crate::AbilityTree,
}

impl super::LayoutImpl for TokenLayout {
    fn card_types(&self) -> arrayvec::ArrayVec<mtg_data::CardType, 4> {
        self.card_type.card_types()
    }

    fn mana_value(&self) -> usize {
        0
    }

    fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String> {
        Ok(TokenLayout {
            name: raw_card.name.clone(),
            card_type: crate::card::card_type::CardType::parse(&raw_card.type_line, raw_card)
                .map_err(|e| format!("Failed to parse card type: {e}"))?,
            abilities: match raw_card.oracle_text.as_ref() {
                Some(oracle_text) => crate::AbilityTree::from_oracle_text(oracle_text, &raw_card.name)
                    .map_err(|e| format!("Failed to parse oracle text to ability tree: {e}"))?,
                None => crate::AbilityTree::empty(),
            },
        })
    }

    fn display<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()> {
        writeln!(output, "│ ╰─ Token:")?;
        writeln!(output, "│    ├─ Type Line: {}", self.card_type)?;
        write!(output, "│    ╰─ Abilities: ")?;
        self.abilities.display_from_root(output, "│       ")?;
        writeln!(output, "")?;
        Ok(())
    }
}
