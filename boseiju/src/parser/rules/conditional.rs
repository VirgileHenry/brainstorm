use super::ParserNode;
use crate::ability_tree::time;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let backward_duration_to_event_occured_condition = [time::BackwardDuration::ThisTurn {
        #[cfg(feature = "spanned_tree")]
        span: Default::default(),
    }]
    .into_iter()
    .map(|duration| super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::Event { event: dummy() }.id(),
            ParserNode::LexerToken(Token::BackwardDuration(duration)).id(),
        ]),
        merged: ParserNode::Condition { condition: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Event { event },
                ParserNode::LexerToken(Token::BackwardDuration(duration)),
            ] => Ok(ParserNode::Condition {
                condition: crate::ability_tree::conditional::Condition::EventOccured(
                    crate::ability_tree::conditional::ConditionEventOccured {
                        timeframe: *duration,
                        event: event.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: event.node_span().merge(&duration.node_span()),
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::Condition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::Condition {
                    condition: crate::ability_tree::conditional::Condition::ObjectMatchSpecifiers(
                        crate::ability_tree::conditional::ConditionObjectMatchSpecifiers {
                            object: reference.clone(),
                            specifiers: specifiers.clone(),
                            shall_match: true,
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(&specifiers.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Object match specifiers: "if it's a red card, ..." */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ApostropheS {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::Condition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::Condition {
                    condition: crate::ability_tree::conditional::Condition::ObjectMatchSpecifiers(
                        crate::ability_tree::conditional::ConditionObjectMatchSpecifiers {
                            object: reference.clone(),
                            specifiers: specifiers.clone(),
                            shall_match: true,
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(&specifiers.node_span()),
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::An {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::Condition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::An { .. })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::Condition {
                    condition: crate::ability_tree::conditional::Condition::ObjectMatchSpecifiers(
                        crate::ability_tree::conditional::ConditionObjectMatchSpecifiers {
                            object: reference.clone(),
                            specifiers: specifiers.clone(),
                            shall_match: true,
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(&specifiers.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Object match specifiers: "if it isn't a red card, ..." */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Isnt {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::Condition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Isnt { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::Condition {
                    condition: crate::ability_tree::conditional::Condition::ObjectMatchSpecifiers(
                        crate::ability_tree::conditional::ConditionObjectMatchSpecifiers {
                            object: reference.clone(),
                            specifiers: specifiers.clone(),
                            shall_match: false,
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(&specifiers.node_span()),
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
