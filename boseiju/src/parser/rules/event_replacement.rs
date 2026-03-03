use super::ParserNode;
use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let default_event_replacement_rules = vec![
        /* Create source reference: that effect (it), that player, etc */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::EventSourceReference { source: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It { span }))] => {
                    Ok(ParserNode::EventSourceReference {
                        source: crate::ability_tree::event::replacement::EventSourceReference::ThatEvent { span: *span },
                    })
                }
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
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Those {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Supertype(object::Supertype {
                    supertype: mtg_data::Supertype::Token,
                    span: Default::default(),
                })))
                .id(),
            ]),
            merged: ParserNode::CreatedTokenKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of { span: start_span })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Those { .. })),
                    ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Supertype(object::Supertype {
                        supertype: mtg_data::Supertype::Token,
                        span: end_span,
                    }))),
                ] => Ok(ParserNode::CreatedTokenKind {
                    kind: crate::ability_tree::imperative::CreatedTokenKind::PreviouslyMentionnedToken {
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
                ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Create,
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediates::Number::TwiceThatMany {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CreatedTokenKind { kind: dummy() }.id(),
            ]),
            merged: ParserNode::EventReplacement { replacement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSourceReference { source },
                    ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Create,
                        span: create_span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::TwiceThatMany { span: num_span })),
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
                                        amount: Number::ThatMany { span: *num_span },
                                        token: kind.clone(),
                                        span: num_span.merge(&kind.span()),
                                    });
                                    tokens.push(crate::ability_tree::imperative::TokenCreation {
                                        amount: Number::ThatMany { span: *num_span },
                                        token: kind.clone(),
                                        span: num_span.merge(&kind.span()),
                                    });
                                    tokens
                                },
                                span: create_span.merge(&kind.span()),
                            },
                            span: source.span().merge(&kind.span()),
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
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Those {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Counter,
                    span: Default::default(),
                }))
                .id(), /* Fixme */
            ]),
            merged: ParserNode::PutCounterKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of { span: start_span })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Those { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(terminals::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Counter,
                        span: end_span,
                    })),
                ] => Ok(ParserNode::PutCounterKind {
                    kind: crate::ability_tree::imperative::CounterKind::PreviouslyMentionnedCounter {
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
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediates::Number::TwiceThatMany {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::PutCounterKind { kind: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::EventReplacement { replacement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSourceReference { source },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put { span: put_span })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::TwiceThatMany { span: num_span })),
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
                                            amount: Number::ThatMany { span: *num_span },
                                            counter: counter_kind.clone(),
                                            span: num_span.merge(&counter_kind.span()),
                                        });
                                        counters.push(crate::ability_tree::imperative::CounterOnPermanent {
                                            amount: Number::ThatMany { span: *num_span },
                                            counter: counter_kind.clone(),
                                            span: num_span.merge(&counter_kind.span()),
                                        });
                                        counters
                                    },
                                    span: put_span.merge(&permanent_kind.span()),
                                },
                                span: source.span().merge(&permanent_kind.span()),
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
