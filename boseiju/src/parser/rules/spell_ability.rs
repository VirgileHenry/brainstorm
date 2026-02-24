use super::ParserNode;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* A Single statement can make a spell ability. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Statement { statement: dummy() }.id()]),
            merged: ParserNode::SpellAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Statement { statement }] => Some(ParserNode::SpellAbility {
                    ability: {
                        let mut statements = arrayvec::ArrayVec::new_const();
                        statements.push(statement.clone());
                        crate::ability_tree::ability::spell::SpellAbility {
                            effects: Box::new(statements),
                        }
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Spell abilities can have multiple statements, so we can add additionnal statements */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::SpellAbility { ability: dummy() }.id(),
                ParserNode::Statement { statement: dummy() }.id(),
            ]),
            merged: ParserNode::SpellAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellAbility { ability }, ParserNode::Statement { statement }] => Some(ParserNode::SpellAbility {
                    ability: {
                        let mut ability = ability.clone();
                        ability.effects.push(statement.clone());
                        ability
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
