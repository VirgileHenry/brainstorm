use super::ParserNode;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<statement>" is a spell ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Statement {
                statement: Default::default(),
            }
            .id()]),
            merged: ParserNode::SpellAbility {
                ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Statement { statement }] => Ok(ParserNode::SpellAbility {
                    ability: {
                        let mut statements = boseiju_tree::HeapArrayVec::new();
                        statements.push(statement.clone());
                        boseiju_tree::ability_tree::ability::spell::SpellAbility {
                            effects: statements,
                            #[cfg(feature = "spanned_tree")]
                            span: statement.span(),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<spell ability>" makes an ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::SpellAbility {
                ability: Default::default(),
            }
            .id()]),
            merged: ParserNode::WrittenAbility {
                ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellAbility { ability }] => Ok(ParserNode::WrittenAbility {
                    ability: boseiju_tree::ability_tree::ability::WrittenAbility::Spell(
                        boseiju_tree::ability_tree::ability::spell::SpellAbility {
                            effects: ability.effects.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: ability.span(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
