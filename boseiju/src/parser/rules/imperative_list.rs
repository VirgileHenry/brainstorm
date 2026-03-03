use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* All imperatives without an explicit executing player, so the owner of the effect is the executing player */
    let owner_of_effect_is_executing_player = vec![
        /* An imperative on its own can make a imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Imperative { imperative: dummy() }.id()]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Imperative { imperative }] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imperative.clone());
                            imperatives
                        },
                        executing_player: crate::ability_tree::terminals::PlayerSpecifier::You {
                            span: imperative.span().empty_at_start(),
                        },
                        condition: None,
                        span: imperative.span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative> and <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative { imperative: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives
                        },
                        executing_player: crate::ability_tree::terminals::PlayerSpecifier::You {
                            span: imp1.span().empty_at_start(),
                        },
                        condition: None,
                        span: imp1.span().merge(&imp2.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative> unless <condition>" can make an imperative list with a condition */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative { imperative: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Unless {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition { condition: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Unless { .. })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imperative.clone());
                            imperatives
                        },
                        executing_player: crate::ability_tree::terminals::PlayerSpecifier::You {
                            span: imperative.span().empty_at_start(),
                        },
                        condition: Some(crate::ability_tree::conditional::Conditional::Unless(
                            crate::ability_tree::conditional::ConditionalUnless {
                                condition: condition.clone(),
                                span: imperative.span().merge(&condition.span()),
                            },
                        )),
                        span: imperative.span().merge(&condition.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "if <condition> <imperative>" can make an imperative list with a condition */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition { condition: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { span: if_span })),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Imperative { imperative },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imperative.clone());
                            imperatives
                        },
                        executing_player: crate::ability_tree::terminals::PlayerSpecifier::You {
                            span: imperative.span().empty_at_start(),
                        },
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                span: if_span.merge(&imperative.span()),
                            },
                        )),
                        span: if_span.merge(&imperative.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let imperative_list_with_executing_player = [
        terminals::PlayerSpecifier::All {
            span: Default::default(),
        },
        terminals::PlayerSpecifier::EachOpponent {
            span: Default::default(),
        },
        terminals::PlayerSpecifier::TargetOpponent {
            span: Default::default(),
        },
        terminals::PlayerSpecifier::ToYourLeft {
            span: Default::default(),
        },
        terminals::PlayerSpecifier::ToYourRight {
            span: Default::default(),
        },
        terminals::PlayerSpecifier::You {
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|player| {
        [
            /* <player> <imperative> makes an owned imperative list */
            ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::PlayerSpecifier(player)).id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                ]),
                merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::PlayerSpecifier(player)),
                        ParserNode::Imperative { imperative },
                    ] => Ok(ParserNode::ImperativeList {
                        imperatives: crate::ability_tree::imperative::ImperativeList {
                            imperatives: {
                                let mut imperatives = crate::utils::HeapArrayVec::new();
                                imperatives.push(imperative.clone());
                                imperatives
                            },
                            executing_player: player.clone(),
                            condition: None,
                            span: player.span().merge(&imperative.span()),
                        },
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* "<player> <imperative> and <imperative>" can make an imperative list */
            ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::PlayerSpecifier(player)).id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                ]),
                merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::PlayerSpecifier(player)),
                        ParserNode::Imperative { imperative: imp1 },
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                        ParserNode::Imperative { imperative: imp2 },
                    ] => Ok(ParserNode::ImperativeList {
                        imperatives: crate::ability_tree::imperative::ImperativeList {
                            imperatives: {
                                let mut imperatives = crate::utils::HeapArrayVec::new();
                                imperatives.push(imp1.clone());
                                imperatives.push(imp2.clone());
                                imperatives
                            },
                            executing_player: player.clone(),
                            condition: None,
                            span: player.span().merge(&imp2.span()),
                        },
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* "<player> <imperative> unless <condition>" can make an imperative list with a condition */
            ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::PlayerSpecifier(player)).id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Unless {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::Condition { condition: dummy() }.id(),
                ]),
                merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::PlayerSpecifier(player)),
                        ParserNode::Imperative { imperative },
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Unless {
                            span: unless_span,
                        })),
                        ParserNode::Condition { condition },
                    ] => Ok(ParserNode::ImperativeList {
                        imperatives: crate::ability_tree::imperative::ImperativeList {
                            imperatives: {
                                let mut imperatives = crate::utils::HeapArrayVec::new();
                                imperatives.push(imperative.clone());
                                imperatives
                            },
                            executing_player: player.clone(),
                            condition: Some(crate::ability_tree::conditional::Conditional::Unless(
                                crate::ability_tree::conditional::ConditionalUnless {
                                    condition: condition.clone(),
                                    span: unless_span.merge(&condition.span()),
                                },
                            )),
                            span: player.span().merge(&condition.span()),
                        },
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* "if <condition>, <player> <imperative>" can make an imperative list with a condition */
            ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::Condition { condition: dummy() }.id(),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::PlayerSpecifier(player)).id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                ]),
                merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { span: if_span })),
                        ParserNode::Condition { condition },
                        ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                        ParserNode::LexerToken(Token::PlayerSpecifier(player)),
                        ParserNode::Imperative { imperative },
                    ] => Ok(ParserNode::ImperativeList {
                        imperatives: crate::ability_tree::imperative::ImperativeList {
                            imperatives: {
                                let mut imperatives = crate::utils::HeapArrayVec::new();
                                imperatives.push(imperative.clone());
                                imperatives
                            },
                            executing_player: player.clone(),
                            condition: Some(crate::ability_tree::conditional::Conditional::If(
                                crate::ability_tree::conditional::ConditionalIf {
                                    condition: condition.clone(),
                                    span: if_span.merge(&condition.span()),
                                },
                            )),
                            span: if_span.merge(&imperative.span()),
                        },
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
        ]
    })
    .flatten()
    .collect::<Vec<_>>();

    [owner_of_effect_is_executing_player, imperative_list_with_executing_player]
        .into_iter()
        .flatten()
}
