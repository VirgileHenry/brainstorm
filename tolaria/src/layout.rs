/// All the layouts of Magic: The Gathering for playable cards.
#[derive(Debug, Clone)]
pub enum Layout {
    Normal {
        mana_cost: Option<crate::mana_cost::ManaCost>,
        card_type: crate::card_type::CardType,
        abilities: odin::AbilityTree,
    },
    Split {},
    Flip {},
    Transform {},
    ModalDfc {},
    Meld {},
    Leveler {},
    Class {
        mana_cost: Option<crate::mana_cost::ManaCost>,
        card_type: crate::card_type::CardType,
        abilities: odin::AbilityTree,
        levels: arrayvec::ArrayVec<odin::ability_tree::imperative::Imperative, 8>,
    },
    Case {},
    Saga {
        mana_cost: Option<crate::mana_cost::ManaCost>,
        card_type: crate::card_type::CardType,
        chapters: arrayvec::ArrayVec<odin::AbilityTree, 4>,
    },
    Adventure {},
    Mutate {},
    Prototype {},
    Battle {},
    Planar {},
    Scheme {},
    Vanguard {},
    Token {
        card_type: crate::card_type::CardType,
        abilities: odin::AbilityTree,
    },
    DoubleFaced {},
    Emblem {},
}

impl Layout {
    pub fn display<W: std::io::Write>(&self, output: &mut W) -> std::io::Result<()> {
        match self {
            Layout::Normal {
                mana_cost,
                card_type,
                ..
            } => {
                writeln!(output, "│ ╰─ Normal:")?;
                if let Some(mana_cost) = mana_cost {
                    writeln!(output, "│    ├─ Mana Cost: {mana_cost}")?;
                }
                writeln!(output, "│    ├─ Type Line: {card_type}")?;
                writeln!(output, "│    ╰─ Abilities: TODO")?; // Todo
                Ok(())
            }
            _ => writeln!(output, "Unimplemented!"),
        }
    }
}

impl TryFrom<&mtg_cardbase::Card> for Layout {
    type Error = String; // Fixme!
    fn try_from(raw_card: &mtg_cardbase::Card) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        match raw_card.layout {
            "normal" => Ok(Layout::Normal {
                mana_cost: match raw_card.mana_cost {
                    Some(mana_cost) => Some(
                        crate::mana_cost::ManaCost::from_str(mana_cost)
                            .map_err(|e| format!("Failed to parse mana cost: {e}"))?,
                    ),
                    None => None,
                },
                card_type: crate::card_type::CardType::parse(raw_card.type_line, raw_card)
                    .map_err(|e| format!("Failed to parse card type: {e}"))?,
                abilities: match raw_card.oracle_text {
                    Some(oracle_text) => {
                        odin::AbilityTree::from_oracle_text(oracle_text, raw_card.name).map_err(
                            |e| format!("Failed to parse oracle text to ability tree: {e}"),
                        )?
                    }
                    None => odin::AbilityTree::empty(),
                },
            }),
            "token" => Ok(Layout::Token {
                card_type: crate::card_type::CardType::parse(raw_card.type_line, raw_card)
                    .map_err(|e| format!("Failed to parse card type: {e}"))?,
                abilities: odin::AbilityTree::empty(),
            }),
            "saga" => {
                let _ability_tree = odin::AbilityTree::empty();
                // Todo: attempt to get out the chapters from the ability tree
                // On failure, it means the saga is badly formatted
                Ok(Layout::Saga {
                    mana_cost: match raw_card.mana_cost {
                        Some(mana_cost) => Some(
                            crate::mana_cost::ManaCost::from_str(mana_cost)
                                .map_err(|e| format!("Failed to parse mana cost: {e}"))?,
                        ),
                        None => None,
                    },
                    card_type: crate::card_type::CardType::parse(raw_card.type_line, raw_card)
                        .map_err(|e| format!("Failed to parse card type: {e}"))?,
                    chapters: arrayvec::ArrayVec::new(),
                })
            }
            "class" => {
                let _ability_tree = odin::AbilityTree::empty();
                // Todo: attempt to get out the static ability and levels from the ability tree
                // On failure, it means the class is badly formatted
                Ok(Layout::Class {
                    mana_cost: match raw_card.mana_cost {
                        Some(mana_cost) => Some(
                            crate::mana_cost::ManaCost::from_str(mana_cost)
                                .map_err(|e| format!("Failed to parse mana cost: {e}"))?,
                        ),
                        None => None,
                    },
                    card_type: crate::card_type::CardType::parse(raw_card.type_line, raw_card)
                        .map_err(|e| format!("Failed to parse card type: {e}"))?,
                    abilities: odin::AbilityTree::empty(),
                    levels: arrayvec::ArrayVec::new(),
                })
            }
            other => Err(format!("Invalid layout in card: {other}")),
        }
    }
}
