/// All the layouts of Magic: The Gathering for playable cards.
#[derive(Debug, Clone)]
pub enum Layout {
    Normal {
        mana_cost: crate::mana_cost::ManaCost,
        card_type: crate::card_type::CardType,
        abilities: odin::AbilityTree,
    },
    Split {},
    Flip {},
    Transform {},
    ModalDfc {},
    Meld {},
    Leveler {},
    Class {},
    Case {},
    Saga {},
    Adventure {},
    Mutate {},
    Prototype {},
    Battle {},
    Planar {},
    Scheme {},
    Vanguard {},
    Token {},
    DoubleFaced {},
    Emblem {},
}

impl TryFrom<&mtg_cardbase::Card> for Layout {
    type Error = String; // Fixme!
    fn try_from(raw_card: &mtg_cardbase::Card) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        let mana_cost = raw_card
            .mana_cost
            .ok_or_else(|| format!("No mana cost on normal layout card!"))?;

        match raw_card.layout {
            "normal" => Ok(Layout::Normal {
                mana_cost: crate::mana_cost::ManaCost::from_str(mana_cost)
                    .map_err(|e| format!("Failed to parse mana cost: {e}"))?,
                card_type: crate::card_type::CardType::parse(raw_card.type_line, raw_card)
                    .map_err(|e| format!("Failed to parse card type: {e}"))?,
                abilities: odin::AbilityTree::empty(),
            }),
            other => Err(format!("Invalid layout in card: {other}")),
        }
    }
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layout::Normal {
                mana_cost,
                card_type,
                abilities,
            } => {
                writeln!(f, "Normal:")?;
                writeln!(f, "Mana Cost: {mana_cost}")?;
                writeln!(f, "Type Line: {card_type}")?;
                write!(f, "Abilities: TODO")?; // fixme
                Ok(())
            }
            _ => write!(f, "Unimplemented!"),
        }
    }
}
