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
    let put_counters_rules = terminal::Counter::all()
        .flat_map(|counter| {
            [
                /* "put <number> <counter> on <active permanent reference>" is an active put counters deed */
                ParserRule {
                    expanded: RuleLhs::new(&[
                        ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                            token: intermediate::ActionKeyword::Put {
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
                        ParserNode::LexerToken(Token::Counter(counter)).id(),
                        ParserNode::LexerToken(Token::EnglishPreposition(intermediate::EnglishPreposition::On {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::PermanentActive {
                            permanent: Default::default(),
                        }
                        .id(),
                    ]),
                    merged: ParserNode::DeedPutCountersActive {
                        deed: Default::default(),
                    }
                    .id(),
                    reduction: |nodes: &[ParserNode]| match &nodes {
                        &[
                            ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                                token:
                                    intermediate::ActionKeyword::Put {
                                        #[cfg(feature = "spanned_tree")]
                                            span: put_span,
                                    },
                                tense: boseiju_lexer::Tense::BaseForm,
                            })),
                            ParserNode::Number { number },
                            ParserNode::LexerToken(Token::Counter(counter)),
                            ParserNode::LexerToken(Token::EnglishPreposition(intermediate::EnglishPreposition::On { .. })),
                            ParserNode::PermanentActive { permanent },
                        ] => Ok(ParserNode::DeedPutCountersActive {
                            deed: boseiju_tree::ability_tree::deed::put_counters::PutCounters {
                                object: permanent.clone(),
                                counters: std::iter::once(boseiju_tree::ability_tree::deed::put_counters::CounterOnPermanent {
                                    amount: number.clone(),
                                    counter: boseiju_tree::ability_tree::deed::put_counters::CounterKind::NewCounter(
                                        counter.clone(),
                                    ),
                                    #[cfg(feature = "spanned_tree")]
                                    span: number.span().merge(&counter.span),
                                })
                                .collect(),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(put_span),
                            },
                        }),
                        _ => Err("Provided tokens do not match rule definition"),
                    },
                    creation_loc: ParserRuleDeclarationLocation::here(),
                },
                /* "put <number> <counter> on <passive permanent reference>" is an passive put counters deed */
                ParserRule {
                    expanded: RuleLhs::new(&[
                        ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                            token: intermediate::ActionKeyword::Put {
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
                        ParserNode::LexerToken(Token::Counter(counter)).id(),
                        ParserNode::LexerToken(Token::EnglishPreposition(intermediate::EnglishPreposition::On {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }))
                        .id(),
                        ParserNode::PermanentPassive {
                            permanent: Default::default(),
                        }
                        .id(),
                    ]),
                    merged: ParserNode::DeedPutCountersPassive {
                        deed: Default::default(),
                    }
                    .id(),
                    reduction: |nodes: &[ParserNode]| match &nodes {
                        &[
                            ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                                token:
                                    intermediate::ActionKeyword::Put {
                                        #[cfg(feature = "spanned_tree")]
                                            span: put_span,
                                    },
                                tense: boseiju_lexer::Tense::BaseForm,
                            })),
                            ParserNode::Number { number },
                            ParserNode::LexerToken(Token::Counter(counter)),
                            ParserNode::LexerToken(Token::EnglishPreposition(intermediate::EnglishPreposition::On { .. })),
                            ParserNode::PermanentPassive { permanent },
                        ] => Ok(ParserNode::DeedPutCountersPassive {
                            deed: boseiju_tree::ability_tree::deed::put_counters::PutCounters {
                                object: permanent.clone(),
                                counters: std::iter::once(boseiju_tree::ability_tree::deed::put_counters::CounterOnPermanent {
                                    amount: number.clone(),
                                    counter: boseiju_tree::ability_tree::deed::put_counters::CounterKind::NewCounter(
                                        counter.clone(),
                                    ),
                                    #[cfg(feature = "spanned_tree")]
                                    span: number.span().merge(&counter.span),
                                })
                                .collect(),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(put_span),
                            },
                        }),
                        _ => Err("Provided tokens do not match rule definition"),
                    },
                    creation_loc: ParserRuleDeclarationLocation::here(),
                },
            ]
        })
        .collect::<Vec<_>>();

    [put_counters_rules].into_iter().flatten()
}
