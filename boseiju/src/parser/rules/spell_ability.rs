use super::ParserNode;
use crate::{
    lexer::tokens::{TokenKind, non_terminals},
    utils::dummy,
};
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* A Single statement and a dot can make a spell ability. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Statement { statement: dummy() }.id()]),
            merged: ParserNode::SpellAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Statement { statement }] => Ok(ParserNode::SpellAbility {
                    ability: {
                        let mut statements = arrayvec::ArrayVec::new_const();
                        statements.push(statement.clone());
                        crate::ability_tree::ability::spell::SpellAbility {
                            effects: Box::new(statements),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Spell abilities can have multiple statements, so we can add additionnal statements */
        /* Wording with two separate statements */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::SpellAbility { ability: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)).id(),
                ParserNode::Statement { statement: dummy() }.id(),
            ]),
            merged: ParserNode::SpellAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::SpellAbility { ability },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::SpellAbility {
                    ability: {
                        let mut ability = ability.clone();
                        ability.effects.push(statement.clone());
                        ability
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
