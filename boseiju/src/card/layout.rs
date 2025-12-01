/// All the layouts of Magic: The Gathering for playable cards.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Layout {
    Normal {
        mana_cost: Option<crate::ability_tree::terminals::ManaCost>,
        card_type: crate::card::card_type::CardType,
        abilities: crate::AbilityTree,
    },
    Split {},
    Flip {},
    Transform {},
    ModalDfc {},
    Meld {},
    Leveler {},
    Class {},
    Case {},
    Saga {
        mana_cost: Option<crate::ability_tree::terminals::ManaCost>,
        card_type: crate::card::card_type::CardType,
        chapters: arrayvec::ArrayVec<crate::ability_tree::ability::saga_chapter::SagaChapter, 4>,
    },
    Adventure {},
    Mutate {},
    Prototype {},
    Battle {},
    Planar {},
    Scheme {},
    Vanguard {},
    Token {
        card_type: crate::card::card_type::CardType,
        abilities: crate::AbilityTree,
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
                abilities,
            } => {
                writeln!(output, "│ ╰─ Normal:")?;
                if let Some(mana_cost) = mana_cost {
                    writeln!(output, "│    ├─ Mana Cost: {mana_cost}")?;
                }
                writeln!(output, "│    ├─ Type Line: {card_type}")?;
                write!(output, "│    ╰─ Abilities: ")?;
                abilities.display_from_root(output, "│       ")?;
                writeln!(output, "")?;
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

        match raw_card.layout.as_str() {
            "normal" => Ok(Layout::Normal {
                mana_cost: match raw_card.mana_cost.as_ref() {
                    Some(mana_cost) => Some(
                        crate::ability_tree::terminals::ManaCost::from_str(mana_cost)
                            .map_err(|e| format!("Failed to parse mana cost: {e}"))?,
                    ),
                    None => None,
                },
                card_type: crate::card::card_type::CardType::parse(&raw_card.type_line, raw_card)
                    .map_err(|e| format!("Failed to parse card type: {e}"))?,
                abilities: match raw_card.oracle_text.as_ref() {
                    Some(oracle_text) => crate::AbilityTree::from_oracle_text(oracle_text, &raw_card.name)
                        .map_err(|e| format!("Failed to parse oracle text to ability tree: {e}"))?,
                    None => crate::AbilityTree::empty(),
                },
            }),
            "token" => Ok(Layout::Token {
                card_type: crate::card::card_type::CardType::parse(&raw_card.type_line, raw_card)
                    .map_err(|e| format!("Failed to parse card type: {e}"))?,
                abilities: match raw_card.oracle_text.as_ref() {
                    Some(oracle_text) => crate::AbilityTree::from_oracle_text(oracle_text, &raw_card.name)
                        .map_err(|e| format!("Failed to parse oracle text to ability tree: {e}"))?,
                    None => crate::AbilityTree::empty(),
                },
            }),
            "saga" => {
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
                Ok(Layout::Saga {
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
            other => Err(format!("Invalid layout in card: {other}")),
        }
    }
}
