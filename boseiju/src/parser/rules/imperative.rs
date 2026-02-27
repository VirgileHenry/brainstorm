use crate::lexer::tokens::{TokenKind, non_terminals};
use crate::parser::ParserNode;
use crate::utils::dummy;
use idris::Idris;

mod choose_rules;
mod deals_damage_rules;
mod destroy_rules;
mod discard_rules;
mod draw_rules;
mod exile_follow_up_rules;
mod exile_rules;
mod put_counters_rules;
mod remove_counters_rules;
mod return_rules;
mod sacrifice_rules;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let imperative_rules = vec![
        /* An imperative on its own is a conditional imperative with no condition */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Imperative { imperative: dummy() }.id()]),
            merged: ParserNode::ConditionalImperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Imperative { imperative }] => Ok(ParserNode::ConditionalImperative {
                    imperative: crate::ability_tree::imperative::ConditionalImperative {
                        imperative: imperative.clone(),
                        condition: None,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* An imperative with a condition is a conditional imperative: "X unless Y" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative { imperative: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Unless)).id(),
                ParserNode::Condition { condition: dummy() }.id(),
            ]),
            merged: ParserNode::ConditionalImperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Unless)),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::ConditionalImperative {
                    imperative: crate::ability_tree::imperative::ConditionalImperative {
                        imperative: imperative.clone(),
                        condition: Some(crate::ability_tree::conditional::Conditional::Unless(
                            crate::ability_tree::conditional::ConditionalUnless {
                                condition: condition.clone(),
                            },
                        )),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Conditional imperative with if will start by the conditional: "if X, Y" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)).id(),
                ParserNode::Condition { condition: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::Imperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ConditionalImperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::Imperative { imperative },
                ] => Ok(ParserNode::ConditionalImperative {
                    imperative: crate::ability_tree::imperative::ConditionalImperative {
                        imperative: imperative.clone(),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                            },
                        )),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* A conditional imperative on its own can make a imperative list */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ConditionalImperative { imperative: dummy() }.id()]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ConditionalImperative { imperative }] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imperative.clone());
                            imperatives
                        },
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* We can add imperatives to imperative list: A then B */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Then)).id(),
                ParserNode::ConditionalImperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Then)),
                    ParserNode::ConditionalImperative { imperative },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: {
                        let mut imperatives = imperatives.clone();
                        imperatives.imperatives.push(imperative.clone());
                        imperatives
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* We can add imperatives to imperative list: A. then B */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Then)).id(),
                ParserNode::ConditionalImperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Then)),
                    ParserNode::ConditionalImperative { imperative },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: {
                        let mut imperatives = imperatives.clone();
                        imperatives.imperatives.push(imperative.clone());
                        imperatives
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* We can add imperatives to imperative list: A. B */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
                ParserNode::ConditionalImperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                    ParserNode::ConditionalImperative { imperative },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: {
                        let mut imperatives = imperatives.clone();
                        imperatives.imperatives.push(imperative.clone());
                        imperatives
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        imperative_rules,
        choose_rules::rules().collect::<Vec<_>>(),
        deals_damage_rules::rules().collect::<Vec<_>>(),
        destroy_rules::rules().collect::<Vec<_>>(),
        discard_rules::rules().collect::<Vec<_>>(),
        draw_rules::rules().collect::<Vec<_>>(),
        exile_follow_up_rules::rules().collect::<Vec<_>>(),
        exile_rules::rules().collect::<Vec<_>>(),
        put_counters_rules::rules().collect::<Vec<_>>(),
        remove_counters_rules::rules().collect::<Vec<_>>(),
        return_rules::rules().collect::<Vec<_>>(),
        sacrifice_rules::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
