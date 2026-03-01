use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Multiple keyword abilities can be separated by commas instead of newlines */
        /* 2 keyword abilities */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::KeywordAbility { ability: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::KeywordAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::KeywordAbility { ability: ab1 },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::KeywordAbility { ability: ab2 },
                ] => Ok(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = crate::utils::HeapArrayVec::new();
                        abilities.push(crate::ability_tree::ability::AbilityKind::Keyword(ab1.clone()));
                        abilities.push(crate::ability_tree::ability::AbilityKind::Keyword(ab2.clone()));
                        crate::AbilityTree { abilities }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* 3 keyword abilities */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::KeywordAbility { ability: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::KeywordAbility { ability: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::KeywordAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::KeywordAbility { ability: ab1 },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::KeywordAbility { ability: ab2 },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::KeywordAbility { ability: ab3 },
                ] => Ok(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = crate::utils::HeapArrayVec::new();
                        abilities.push(crate::ability_tree::ability::AbilityKind::Keyword(ab1.clone()));
                        abilities.push(crate::ability_tree::ability::AbilityKind::Keyword(ab2.clone()));
                        abilities.push(crate::ability_tree::ability::AbilityKind::Keyword(ab3.clone()));
                        crate::AbilityTree { abilities }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* A single Ability can be turned into an ability tree with a single element */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::AbilityKind { ability: dummy() }.id()]),
            merged: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::AbilityKind { ability }] => Ok(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = crate::utils::HeapArrayVec::new();
                        abilities.push(ability.clone());
                        crate::AbilityTree { abilities }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Abilities separated by new lines can be merged into a single ability tree */
        /* Fixme: fixed number of abilities per cards ? */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::AbilityTree { tree: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)).id(),
                ParserNode::AbilityKind { ability: dummy() }.id(),
            ]),
            merged: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AbilityTree { tree },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
                    ParserNode::AbilityKind { ability },
                ] => Ok(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = tree.abilities.clone();
                        abilities.push(ability.clone());
                        crate::AbilityTree { abilities }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
