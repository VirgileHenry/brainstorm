use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_lexer::terminal;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    let remove_counters_rules = terminal::Counter::all()
        .flat_map(|counter| {
            [
                /* "remove <number> <counter> from <permanent ref>": remove counters imperative */
                ParserRule {
                    expanded: RuleLhs::new(&[
                        ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                            token: intermediate::PlayerAction::Remove {
                                #[cfg(feature = "spanned_tree")]
                                span: Default::default(),
                            },
                            tense: boseiju_lexer::Tense::BaseForm,
                        }))
                        .id(),
                        ParserNode::Number {
                            number: Default::default(),
                        }
                        .id(),
                        ParserNode::LexerToken(Token::Counter(counter.clone())).id(),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::From {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::Permanent {
                            permanent: Default::default(),
                        }
                        .id(),
                    ]),
                    merged: ParserNode::ImperativeKind {
                        imperative: Default::default(),
                    }
                    .id(),
                    reduction: |nodes: &[ParserNode]| match &nodes {
                        &[
                            ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                                token:
                                    intermediate::PlayerAction::Remove {
                                        #[cfg(feature = "spanned_tree")]
                                            span: remove_span,
                                    },
                                tense: boseiju_lexer::Tense::BaseForm,
                            })),
                            ParserNode::Number { number },
                            ParserNode::LexerToken(Token::Counter(counter)),
                            ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::From { .. })),
                            ParserNode::Permanent { permanent },
                        ] => Ok(ParserNode::ImperativeKind {
                            imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::RemoveCounters(
                                boseiju_tree::ability_tree::imperative::RemoveCountersImperative {
                                    object: permanent.clone(),
                                    counters: {
                                        let mut counters = boseiju_tree::HeapArrayVec::new();
                                        counters.push(boseiju_tree::ability_tree::imperative::RemovableCounterOnPermanent {
                                            amount: number.clone(),
                                            counter: boseiju_tree::ability_tree::imperative::RemovableCounterKind::NewCounter(
                                                counter.clone(),
                                            ),
                                            #[cfg(feature = "spanned_tree")]
                                            span: number.span().merge(&counter.span),
                                        });
                                        counters
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: remove_span.merge(&permanent.span()),
                                },
                            ),
                        }),
                        _ => Err("Provided tokens do not match rule definition"),
                    },
                    creation_loc: ParserRuleDeclarationLocation::here(),
                },
                /* "remove <number> <counter> from among <permanent ref>": remove counters imperative */
                ParserRule {
                    expanded: RuleLhs::new(&[
                        ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                            token: intermediate::PlayerAction::Remove {
                                #[cfg(feature = "spanned_tree")]
                                span: Default::default(),
                            },
                            tense: boseiju_lexer::Tense::BaseForm,
                        }))
                        .id(),
                        ParserNode::Number {
                            number: Default::default(),
                        }
                        .id(),
                        ParserNode::LexerToken(Token::Counter(counter.clone())).id(),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::From {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Among {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::Permanent {
                            permanent: Default::default(),
                        }
                        .id(),
                    ]),
                    merged: ParserNode::ImperativeKind {
                        imperative: Default::default(),
                    }
                    .id(),
                    reduction: |nodes: &[ParserNode]| match &nodes {
                        &[
                            ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                                token:
                                    intermediate::PlayerAction::Remove {
                                        #[cfg(feature = "spanned_tree")]
                                            span: remove_span,
                                    },
                                tense: boseiju_lexer::Tense::BaseForm,
                            })),
                            ParserNode::Number { number },
                            ParserNode::LexerToken(Token::Counter(counter)),
                            ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::From { .. })),
                            ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Among { .. })),
                            ParserNode::Permanent { permanent },
                        ] => Ok(ParserNode::ImperativeKind {
                            imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::RemoveCounters(
                                boseiju_tree::ability_tree::imperative::RemoveCountersImperative {
                                    object: permanent.clone(),
                                    counters: {
                                        let mut counters = boseiju_tree::HeapArrayVec::new();
                                        counters.push(boseiju_tree::ability_tree::imperative::RemovableCounterOnPermanent {
                                            amount: number.clone(),
                                            counter: boseiju_tree::ability_tree::imperative::RemovableCounterKind::NewCounter(
                                                counter.clone(),
                                            ),
                                            #[cfg(feature = "spanned_tree")]
                                            span: number.span().merge(&counter.span),
                                        });
                                        counters
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: remove_span.merge(&permanent.span()),
                                },
                            ),
                        }),
                        _ => Err("Provided tokens do not match rule definition"),
                    },
                    creation_loc: ParserRuleDeclarationLocation::here(),
                },
            ]
        })
        .collect::<Vec<_>>();

    let remove_any_counter_rules = vec![ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                token: intermediate::PlayerAction::Remove {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::BaseForm,
            }))
            .id(),
            ParserNode::Number {
                number: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Counter {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::From {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Among {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::ImperativeKind {
            imperative: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                    token:
                        intermediate::PlayerAction::Remove {
                            #[cfg(feature = "spanned_tree")]
                                span: remove_span,
                        },
                    tense: boseiju_lexer::Tense::BaseForm,
                })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Counter {
                    #[cfg(feature = "spanned_tree")]
                        span: counter_span,
                })),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::From { .. })),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Among { .. })),
                ParserNode::Permanent { permanent },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::RemoveCounters(
                    boseiju_tree::ability_tree::imperative::RemoveCountersImperative {
                        object: permanent.clone(),
                        counters: {
                            let mut counters = boseiju_tree::HeapArrayVec::new();
                            counters.push(boseiju_tree::ability_tree::imperative::RemovableCounterOnPermanent {
                                amount: number.clone(),
                                counter: boseiju_tree::ability_tree::imperative::RemovableCounterKind::AnyCounter {
                                    #[cfg(feature = "spanned_tree")]
                                    span: *counter_span,
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: number.span().merge(counter_span),
                            });
                            counters
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: remove_span.merge(&permanent.span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }];

    [remove_counters_rules, remove_any_counter_rules].into_iter().flatten()
}
