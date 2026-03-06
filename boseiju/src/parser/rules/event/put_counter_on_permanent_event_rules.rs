use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let put_specific_counter_events = terminals::Counter::all()
        .map(|counter| {
            [
                /* straight up "put a counter on a permanent" */
                ParserRule {
                    expanded: RuleLhs::new(&[
                        ParserNode::EventSource { source: dummy() }.id(),
                        ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::Number { number: dummy() }.id(),
                        ParserNode::LexerToken(Token::Counter(counter.clone())).id(),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::ObjectReference { reference: dummy() }.id(),
                    ]),
                    merged: ParserNode::Event { event: dummy() }.id(),
                    reduction: |nodes: &[ParserNode]| match &nodes {
                        &[
                            ParserNode::EventSource { source },
                            ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put { .. })),
                            ParserNode::Number { number },
                            ParserNode::LexerToken(Token::Counter(counter)),
                            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On { .. })),
                            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                            ParserNode::ObjectReference { reference },
                        ] => Ok(ParserNode::Event {
                            event: crate::ability_tree::event::Event::PutCounterOnPermanent(
                                crate::ability_tree::event::PutCounterOnPermanentEvent {
                                    source: source.clone(),
                                    quantity: number.clone(),
                                    counter_kind: Some(counter.clone()),
                                    on_permanent: reference.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: source.node_span().merge(&reference.node_span()),
                                },
                            ),
                        }),
                        _ => Err("Provided tokens do not match rule definition"),
                    },
                    creation_loc: ParserRuleDeclarationLocation::here(),
                },
                /* Would put counter on permanent */
                ParserRule {
                    expanded: RuleLhs::new(&[
                        ParserNode::EventSource { source: dummy() }.id(),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::Number { number: dummy() }.id(),
                        ParserNode::LexerToken(Token::Counter(counter.clone())).id(),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::ObjectReference { reference: dummy() }.id(),
                    ]),
                    merged: ParserNode::Event { event: dummy() }.id(),
                    reduction: |nodes: &[ParserNode]| match &nodes {
                        &[
                            ParserNode::EventSource { source },
                            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would { .. })),
                            ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put { .. })),
                            ParserNode::Number { number },
                            ParserNode::LexerToken(Token::Counter(counter)),
                            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On { .. })),
                            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                            ParserNode::ObjectReference { reference },
                        ] => Ok(ParserNode::Event {
                            event: crate::ability_tree::event::Event::PutCounterOnPermanent(
                                crate::ability_tree::event::PutCounterOnPermanentEvent {
                                    source: source.clone(),
                                    quantity: number.clone(),
                                    counter_kind: Some(counter.clone()),
                                    on_permanent: reference.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: source.node_span().merge(&reference.node_span()),
                                },
                            ),
                        }),
                        _ => Err("Provided tokens do not match rule definition"),
                    },
                    creation_loc: ParserRuleDeclarationLocation::here(),
                },
            ]
        })
        .flatten()
        .collect::<Vec<_>>();

    let put_any_counter_events = vec![
        /* straight up "put a counter on a permanent" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Counter {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(), /* Fixme: counter is a verb here */
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put { .. })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Counter { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                    ParserNode::ObjectReference { reference },
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::PutCounterOnPermanent(
                        crate::ability_tree::event::PutCounterOnPermanentEvent {
                            source: source.clone(),
                            quantity: number.clone(),
                            counter_kind: None,
                            on_permanent: reference.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: source.node_span().merge(&reference.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Would put counter on permanent */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Counter {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would { .. })),
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put { .. })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Counter { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                    ParserNode::ObjectReference { reference },
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::PutCounterOnPermanent(
                        crate::ability_tree::event::PutCounterOnPermanentEvent {
                            source: source.clone(),
                            quantity: number.clone(),
                            counter_kind: None,
                            on_permanent: reference.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: source.node_span().merge(&reference.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [put_specific_counter_events, put_any_counter_events].into_iter().flatten()
}
