#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct NormalLayout {
    pub mana_cost: Option<crate::ability_tree::terminals::ManaCost>,
    pub card_type: crate::ability_tree::type_line::TypeLine,
    pub abilities: crate::AbilityTree,
}

impl super::LayoutImpl for NormalLayout {
    fn card_types(&self) -> arrayvec::ArrayVec<mtg_data::CardType, 4> {
        self.card_type.card_types()
    }

    fn mana_value(&self) -> usize {
        self.mana_cost.as_ref().map(|cost| cost.mana_value()).unwrap_or(0)
    }

    #[cfg(feature = "parser")]
    fn from_raw_card(raw_card: &mtg_cardbase::Card) -> Result<Self, String> {
        use std::str::FromStr;

        Ok(NormalLayout {
            mana_cost: match raw_card.mana_cost.as_ref() {
                Some(mana_cost) => Some(
                    crate::ability_tree::terminals::ManaCost::from_str(mana_cost)
                        .map_err(|e| format!("Failed to parse mana cost: {e}"))?,
                ),
                None => None,
            },
            card_type: crate::ability_tree::type_line::TypeLine::parse(&raw_card.type_line, raw_card)
                .map_err(|e| format!("Failed to parse card type: {e}"))?,
            abilities: match raw_card.oracle_text.as_ref() {
                Some(oracle_text) => crate::AbilityTree::from_oracle_text(oracle_text, &raw_card.name)
                    .map_err(|e| format!("Failed to parse oracle text to ability tree: {e}"))?,
                None => crate::AbilityTree::empty(),
            },
        })
    }

    fn layout_debug_display<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()> {
        writeln!(output, "│ ╰─ Normal:")?;
        if let Some(mana_cost) = self.mana_cost.as_ref() {
            writeln!(output, "│    ├─ Mana Cost: {mana_cost}")?;
        }
        writeln!(output, "│    ├─ Type Line: {}", self.card_type)?;
        write!(output, "│    ╰─ Abilities: ")?;
        self.abilities.display_from_root(output, "│       ")?;
        writeln!(output, "")?;
        Ok(())
    }
}
