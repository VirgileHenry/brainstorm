use super::ParserNode;
use crate::ability_tree::time;
use crate::lexer::tokens::{TokenKind, non_terminals};
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let backward_duration_to_event_occured_condition = [time::BackwardDuration::ThisTurn]
        .into_iter()
        .map(|duration| super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Event { event: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::BackwardDuration(duration)).id(),
            ]),
            merged: ParserNode::Condition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Event { event },
                    ParserNode::LexerToken(TokenKind::BackwardDuration(duration)),
                ] => Ok(ParserNode::Condition {
                    condition: crate::ability_tree::conditional::Condition::EventOccured(
                        crate::ability_tree::conditional::ConditionEventOccured {
                            timeframe: *duration,
                            event: event.clone(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let condition_rules = vec![
        /* Object match specifiers: "if it is a red card, ..." */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Is)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::Condition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Is)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::Condition {
                    condition: crate::ability_tree::conditional::Condition::ObjectMatchSpecifiers(
                        crate::ability_tree::conditional::ConditionObjectMatchSpecifiers {
                            object: reference.clone(),
                            specifiers: specifiers.clone(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Object match specifiers: "if it is an enchantment card, ..." */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Is)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::An)).id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::Condition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Is)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::An)),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::Condition {
                    condition: crate::ability_tree::conditional::Condition::ObjectMatchSpecifiers(
                        crate::ability_tree::conditional::ConditionObjectMatchSpecifiers {
                            object: reference.clone(),
                            specifiers: specifiers.clone(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [backward_duration_to_event_occured_condition, condition_rules]
        .into_iter()
        .flatten()
}
