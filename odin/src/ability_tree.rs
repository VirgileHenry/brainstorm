pub mod ability;
pub mod imperative;
pub mod object;
pub mod statement;
pub mod terminals;
pub mod zone;

/// One or more abilities.
/// This is the root of the Magic: the Gathering texts,
/// and can represent on its own the full text box of a card.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct AbilityTree {
    pub abilities: Vec<ability::Ability>,
}

impl AbilityTree {
    pub fn from_oracle_text(
        oracle_text: &str,
        card_name: &str,
    ) -> Result<AbilityTree, crate::error::OdinError> {
        let preprocessed = crate::lexer::preprocess(card_name, oracle_text);
        let tokens = crate::lexer::lex(&preprocessed)?;
        let result = crate::parser::parse(&tokens).0?;
        Ok(result)
    }

    pub fn display<W: std::io::Write>(&self, output: &mut W, prefix: &str) -> std::io::Result<()> {
        use std::io::Write;
        let mut tree_formatter = crate::utils::TreeFormatter::new(output, 64, prefix);

        write!(tree_formatter, "Ability Tree:")?;
        for ability in self
            .abilities
            .iter()
            .take(self.abilities.len().saturating_sub(1))
        {
            tree_formatter.push_inter_branch()?;
            ability.display(&mut tree_formatter)?;
            tree_formatter.pop_branch();
        }
        if let Some(ability) = self.abilities.last() {
            tree_formatter.push_final_branch()?;
            ability.display(&mut tree_formatter)?;
            tree_formatter.pop_branch();
        }
        writeln!(tree_formatter, "")?; /* newline */

        Ok(())
    }

    pub fn empty() -> AbilityTree {
        AbilityTree {
            abilities: Vec::with_capacity(0),
        }
    }

    pub fn example() -> AbilityTree {
        AbilityTree {
            abilities: vec![ability::Ability::Triggered(
                ability::triggered::TriggeredAbility {
                    condition:
                        ability::triggered::trigger_cond::TriggerCondition::ObjectDoesAction {
                            object: object::ObjectReference::SelfReferencing,
                            action: terminals::CardActions::Dies,
                        },
                    effect: statement::Statement::Imperative(imperative::Imperative::Put {
                        amount: terminals::Number::Number(1),
                        of: terminals::Counter::PlusOnePlusOne,
                        on: object::ObjectReference::SpecifiedObj {
                            amount: terminals::CountSpecifier::All,
                            specifier: object::ObjectSpecifier::And(
                                Box::new(object::ObjectSpecifier::And(
                                    Box::new(object::ObjectSpecifier::Control(
                                        terminals::ControlSpecifier::YouControl,
                                    )),
                                    Box::new(object::ObjectSpecifier::Kind(
                                        object::ObjectKind::Type(mtg_data::CardType::Creature),
                                    )),
                                )),
                                Box::new(object::ObjectSpecifier::Color(mtg_data::Color::White)),
                            ),
                        },
                    }),
                },
            )],
        }
    }
}

pub trait AbilityTreeImpl {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()>;
}
