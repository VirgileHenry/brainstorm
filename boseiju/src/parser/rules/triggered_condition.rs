use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "At <instant>" make trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::At {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Instant { instant: dummy() }.id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::At {
                        #[cfg(feature = "spanned_tree")]
                            span: at_span,
                    })),
                    ParserNode::Instant { instant },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        kind: crate::ability_tree::ability::triggered::TriggerConditionKind::AtInstant(instant.clone()),
                        condition: None,
                        #[cfg(feature = "spanned_tree")]
                        span: instant.node_span().merge(at_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "At <instant>, if <condition>" make conditonnal trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::At {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Instant { instant: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition { condition: dummy() }.id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::At {
                        #[cfg(feature = "spanned_tree")]
                            span: at_span,
                    })),
                    ParserNode::Instant { instant },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        kind: crate::ability_tree::ability::triggered::TriggerConditionKind::AtInstant(instant.clone()),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: condition.node_span().merge(if_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: instant.node_span().merge(at_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<whenever> <event>" is a trigger condition */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Whenever {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event { event: dummy() }.id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Whenever {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        kind: crate::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: None,
                        #[cfg(feature = "spanned_tree")]
                        span: event.node_span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<when> <event>" is a trigger condition */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::When {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event { event: dummy() }.id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::When {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        kind: crate::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: None,
                        #[cfg(feature = "spanned_tree")]
                        span: event.node_span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* <whenever> <event>, if <condition> can also make a conditionnal trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Whenever {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event { event: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition { condition: dummy() }.id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Whenever {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        kind: crate::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: condition.node_span().merge(if_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: condition.node_span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* <when> <event>, if <condition> can also make a conditionnal trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::When {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event { event: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition { condition: dummy() }.id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::When {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        kind: crate::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: condition.node_span().merge(if_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: condition.node_span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<whenever> <event> during your turn" is a conditional without comma or if */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Whenever {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event { event: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::During {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Turn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Whenever {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::During {
                        #[cfg(feature = "spanned_tree")]
                            span: during_span,
                    })),
                    /* Fixme: a bit weird for a "your turn" ? Maybe it shall be a single token */
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your { .. })),
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Turn {
                        #[cfg(feature = "spanned_tree")]
                            span: turn_span,
                    })),
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        kind: crate::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: crate::ability_tree::conditional::Condition::ThisIsYourTurn(
                                    crate::ability_tree::conditional::ConditionThisIsYourTurn {
                                        #[cfg(feature = "spanned_tree")]
                                        span: during_span.merge(turn_span),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: during_span.merge(turn_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: event.node_span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<when> <event> during your turn" is a conditional without comma or if */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::When {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event { event: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::During {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Turn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::When {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::During {
                        #[cfg(feature = "spanned_tree")]
                            span: during_span,
                    })),
                    /* Fixme: a bit weird for a "your turn" ? Maybe it shall be a single token */
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your { .. })),
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Turn {
                        #[cfg(feature = "spanned_tree")]
                            span: turn_span,
                    })),
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        kind: crate::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: crate::ability_tree::conditional::Condition::ThisIsYourTurn(
                                    crate::ability_tree::conditional::ConditionThisIsYourTurn {
                                        #[cfg(feature = "spanned_tree")]
                                        span: during_span.merge(turn_span),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: during_span.merge(turn_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: event.node_span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
