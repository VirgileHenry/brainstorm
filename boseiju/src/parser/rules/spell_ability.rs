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
                        let mut statements = crate::utils::HeapArrayVec::new();
                        statements.push(statement.clone());
                        crate::ability_tree::ability::spell::SpellAbility { effects: statements }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Spell abilities can have multiple statements.*/
        /* Wording with two separate statements: "A. B." */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Statement { statement: dummy() }.id(),
                ParserNode::Statement { statement: dummy() }.id(),
            ]),
            merged: ParserNode::SpellAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Statement { statement: s1 },
                    ParserNode::Statement { statement: s2 },
                ] => Ok(ParserNode::SpellAbility {
                    ability: {
                        let mut statements = crate::utils::HeapArrayVec::new();
                        statements.push(s1.clone());
                        statements.push(s2.clone());
                        crate::ability_tree::ability::spell::SpellAbility { effects: statements }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Wording with two separate statements and a "Then": "A. Then B." */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Statement { statement: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Then)).id(),
                ParserNode::Statement { statement: dummy() }.id(),
            ]),
            merged: ParserNode::SpellAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Statement { statement: s1 },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Then)),
                    ParserNode::Statement { statement: s2 },
                ] => Ok(ParserNode::SpellAbility {
                    ability: {
                        let mut statements = crate::utils::HeapArrayVec::new();
                        statements.push(s1.clone());
                        statements.push(s2.clone());
                        crate::ability_tree::ability::spell::SpellAbility { effects: statements }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
