use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Put counters on something, "put 2 +1/+1 counters on each creature you control" */
    let remove_counters_rules = terminals::Counter::all()
        .map(|counter| ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Remove {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::Counter(counter)).id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Among {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Remove {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::Counter(counter)),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Among { .. })),
                    ParserNode::ObjectReference { reference },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::RemoveCounters(
                        crate::ability_tree::imperative::RemoveCountersImperative {
                            object: reference.clone(),
                            counters: {
                                let mut counters = crate::utils::HeapArrayVec::new();
                                counters.push(crate::ability_tree::imperative::RemovableCounterOnPermanent {
                                    amount: number.clone(),
                                    counter: crate::ability_tree::imperative::RemovableCounterKind::NewCounter(counter.clone()),
                                    #[cfg(feature = "spanned_tree")]
                                    span: number.node_span().merge(&counter.span),
                                });
                                counters
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: span.merge(&reference.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let remove_any_counter_rules = vec![ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Remove {
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
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Among {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::ObjectReference { reference: dummy() }.id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Remove {
                    #[cfg(feature = "spanned_tree")]
                        span: remove_span,
                })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Counter {
                    #[cfg(feature = "spanned_tree")]
                        span: counter_span,
                })),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From { .. })),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Among { .. })),
                ParserNode::ObjectReference { reference },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::RemoveCounters(
                    crate::ability_tree::imperative::RemoveCountersImperative {
                        object: reference.clone(),
                        counters: {
                            let mut counters = crate::utils::HeapArrayVec::new();
                            counters.push(crate::ability_tree::imperative::RemovableCounterOnPermanent {
                                amount: number.clone(),
                                counter: crate::ability_tree::imperative::RemovableCounterKind::AnyCounter {
                                    #[cfg(feature = "spanned_tree")]
                                    span: *counter_span,
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: number.node_span().merge(counter_span),
                            });
                            counters
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: remove_span.merge(&reference.node_span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }];

    [remove_counters_rules, remove_any_counter_rules].into_iter().flatten()
}
