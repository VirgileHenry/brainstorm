use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "At <instant>" make trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::At {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::RecurrentInstant {
                    instant: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::TriggerCondition {
                condition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::At {
                        #[cfg(feature = "spanned_tree")]
                            span: at_span,
                    })),
                    ParserNode::RecurrentInstant { instant },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: boseiju_tree::ability_tree::ability::triggered::TriggerCondition {
                        kind: boseiju_tree::ability_tree::ability::triggered::TriggerConditionKind::AtInstant(instant.clone()),
                        condition: None,
                        #[cfg(feature = "spanned_tree")]
                        span: instant.span().merge(at_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "At <instant>, if <condition>" make conditonnal trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::At {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::RecurrentInstant {
                    instant: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition {
                    condition: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::TriggerCondition {
                condition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::At {
                        #[cfg(feature = "spanned_tree")]
                            span: at_span,
                    })),
                    ParserNode::RecurrentInstant { instant },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: boseiju_tree::ability_tree::ability::triggered::TriggerCondition {
                        kind: boseiju_tree::ability_tree::ability::triggered::TriggerConditionKind::AtInstant(instant.clone()),
                        condition: Some(boseiju_tree::ability_tree::conditional::Conditional::If(
                            boseiju_tree::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: condition.span().merge(if_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: instant.span().merge(at_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<whenever> <event>" is a trigger condition */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::Whenever {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event {
                    event: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::TriggerCondition {
                condition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::Whenever {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: boseiju_tree::ability_tree::ability::triggered::TriggerCondition {
                        kind: boseiju_tree::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: None,
                        #[cfg(feature = "spanned_tree")]
                        span: event.span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<when> <event>" is a trigger condition */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::When {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event {
                    event: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::TriggerCondition {
                condition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::When {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: boseiju_tree::ability_tree::ability::triggered::TriggerCondition {
                        kind: boseiju_tree::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: None,
                        #[cfg(feature = "spanned_tree")]
                        span: event.span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* <whenever> <event>, if <condition> can also make a conditionnal trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::Whenever {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event {
                    event: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition {
                    condition: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::TriggerCondition {
                condition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::Whenever {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: boseiju_tree::ability_tree::ability::triggered::TriggerCondition {
                        kind: boseiju_tree::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: Some(boseiju_tree::ability_tree::conditional::Conditional::If(
                            boseiju_tree::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: condition.span().merge(if_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: condition.span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* <when> <event>, if <condition> can also make a conditionnal trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::When {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event {
                    event: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition {
                    condition: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::TriggerCondition {
                condition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::When {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: boseiju_tree::ability_tree::ability::triggered::TriggerCondition {
                        kind: boseiju_tree::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: Some(boseiju_tree::ability_tree::conditional::Conditional::If(
                            boseiju_tree::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: condition.span().merge(if_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: condition.span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<whenever> <event> during your turn" is a conditional without comma or if */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::Whenever {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event {
                    event: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishTemporal(intermediate::EnglishTemporal::During {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Turn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::TriggerCondition {
                condition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::Whenever {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::EnglishTemporal(intermediate::EnglishTemporal::During {
                        #[cfg(feature = "spanned_tree")]
                            span: during_span,
                    })),
                    /* Fixme: a bit weird for a "your turn" ? Maybe it shall be a single token */
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Turn {
                        #[cfg(feature = "spanned_tree")]
                            span: turn_span,
                    })),
                ] => Ok(ParserNode::TriggerCondition {
                    condition: boseiju_tree::ability_tree::ability::triggered::TriggerCondition {
                        kind: boseiju_tree::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: Some(boseiju_tree::ability_tree::conditional::Conditional::If(
                            boseiju_tree::ability_tree::conditional::ConditionalIf {
                                condition: boseiju_tree::ability_tree::conditional::Condition::ThisIsYourTurn(
                                    boseiju_tree::ability_tree::conditional::ConditionThisIsYourTurn {
                                        #[cfg(feature = "spanned_tree")]
                                        span: during_span.merge(turn_span),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: during_span.merge(turn_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: event.span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<when> <event> during your turn" is a conditional without comma or if */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::When {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Event {
                    event: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishTemporal(intermediate::EnglishTemporal::During {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Turn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::TriggerCondition {
                condition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::When {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::EnglishTemporal(intermediate::EnglishTemporal::During {
                        #[cfg(feature = "spanned_tree")]
                            span: during_span,
                    })),
                    /* Fixme: a bit weird for a "your turn" ? Maybe it shall be a single token */
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Turn {
                        #[cfg(feature = "spanned_tree")]
                            span: turn_span,
                    })),
                ] => Ok(ParserNode::TriggerCondition {
                    condition: boseiju_tree::ability_tree::ability::triggered::TriggerCondition {
                        kind: boseiju_tree::ability_tree::ability::triggered::TriggerConditionKind::Event(event.clone()),
                        condition: Some(boseiju_tree::ability_tree::conditional::Conditional::If(
                            boseiju_tree::ability_tree::conditional::ConditionalIf {
                                condition: boseiju_tree::ability_tree::conditional::Condition::ThisIsYourTurn(
                                    boseiju_tree::ability_tree::conditional::ConditionThisIsYourTurn {
                                        #[cfg(feature = "spanned_tree")]
                                        span: during_span.merge(turn_span),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: during_span.merge(turn_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: event.span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
