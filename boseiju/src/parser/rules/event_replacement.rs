use super::ParserNode;
use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let default_event_replacement_rules = vec![
        /* Create source reference: that effect (it), that player, etc */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::EventSourceReference { source: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::EventSourceReference {
                    source: crate::ability_tree::event::replacement::EventSourceReference::ThatEvent {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let token_creation_replacement_rules = vec![
        /* "Of those tokens" is a reference to previously mentionned tokens */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Those {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Supertype(object::Supertype {
                    supertype: mtg_data::Supertype::Token,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                })))
                .id(),
            ]),
            merged: ParserNode::CreatedTokenKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Those { .. })),
                    ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Supertype(object::Supertype {
                        supertype: mtg_data::Supertype::Token,
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    }))),
                ] => Ok(ParserNode::CreatedTokenKind {
                    kind: crate::ability_tree::imperative::CreatedTokenKind::PreviouslyMentionnedToken {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* token creation replacement with "twice that many" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::EventSourceReference { source: dummy() }.id(),
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Create,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediates::Number::TwiceThatMany {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CreatedTokenKind { kind: dummy() }.id(),
            ]),
            merged: ParserNode::EventReplacement { replacement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSourceReference { source },
                    ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Create,
                        #[cfg(feature = "spanned_tree")]
                            span: create_span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::TwiceThatMany {
                        #[cfg(feature = "spanned_tree")]
                            span: num_span,
                    })),
                    ParserNode::CreatedTokenKind { kind },
                    /* Fixme: "instead" can be here ? */
                ] => {
                    use crate::ability_tree::event::replacement;
                    use crate::ability_tree::number::Number;
                    Ok(ParserNode::EventReplacement {
                        replacement: replacement::EventReplacement::TokenCreation(replacement::TokenCreationReplacement {
                            source_ref: source.clone(),
                            create_tokens: crate::ability_tree::imperative::CreateTokenImperative {
                                tokens: {
                                    let mut tokens = crate::utils::HeapArrayVec::new();
                                    tokens.push(crate::ability_tree::imperative::TokenCreation {
                                        amount: Number::ThatMany {
                                            #[cfg(feature = "spanned_tree")]
                                            span: *num_span,
                                        },
                                        token: kind.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: num_span.merge(&kind.node_span()),
                                    });
                                    tokens.push(crate::ability_tree::imperative::TokenCreation {
                                        amount: Number::ThatMany {
                                            #[cfg(feature = "spanned_tree")]
                                            span: *num_span,
                                        },
                                        token: kind.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: num_span.merge(&kind.node_span()),
                                    });
                                    tokens
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: create_span.merge(&kind.node_span()),
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: source.node_span().merge(&kind.node_span()),
                        }),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let put_counter_on_permanent_replacement_rules = vec![
        /* "Of those counters" is a reference to previously mentionned counters */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Those {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Counter {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(), /* Fixme */
            ]),
            merged: ParserNode::PutCounterKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Those { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Counter {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::PutCounterKind {
                    kind: crate::ability_tree::imperative::CounterKind::PreviouslyMentionnedCounter {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* counter replacement with "twice that many" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::EventSourceReference { source: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediates::Number::TwiceThatMany {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::PutCounterKind { kind: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::EventReplacement { replacement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSourceReference { source },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put {
                        #[cfg(feature = "spanned_tree")]
                            span: put_span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::TwiceThatMany {
                        #[cfg(feature = "spanned_tree")]
                            span: num_span,
                    })),
                    ParserNode::PutCounterKind { kind: counter_kind },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On { .. })),
                    ParserNode::ObjectReference {
                        reference: permanent_kind,
                    },
                ] => {
                    use crate::ability_tree::event::replacement;
                    use crate::ability_tree::number::Number;
                    Ok(ParserNode::EventReplacement {
                        replacement: replacement::EventReplacement::CounterOnPermanent(
                            replacement::CounterOnPermanentReplacement {
                                source_ref: source.clone(),
                                put_counters: crate::ability_tree::imperative::PutCountersImperative {
                                    object: permanent_kind.clone(),
                                    counters: {
                                        let mut counters = crate::utils::HeapArrayVec::new();
                                        counters.push(crate::ability_tree::imperative::CounterOnPermanent {
                                            amount: Number::ThatMany {
                                                #[cfg(feature = "spanned_tree")]
                                                span: *num_span,
                                            },
                                            counter: counter_kind.clone(),
                                            #[cfg(feature = "spanned_tree")]
                                            span: num_span.merge(&counter_kind.node_span()),
                                        });
                                        counters.push(crate::ability_tree::imperative::CounterOnPermanent {
                                            amount: Number::ThatMany {
                                                #[cfg(feature = "spanned_tree")]
                                                span: *num_span,
                                            },
                                            counter: counter_kind.clone(),
                                            #[cfg(feature = "spanned_tree")]
                                            span: num_span.merge(&counter_kind.node_span()),
                                        });
                                        counters
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: put_span.merge(&permanent_kind.node_span()),
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: source.node_span().merge(&permanent_kind.node_span()),
                            },
                        ),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        default_event_replacement_rules,
        token_creation_replacement_rules,
        put_counter_on_permanent_replacement_rules,
    ]
    .into_iter()
    .flatten()
}
