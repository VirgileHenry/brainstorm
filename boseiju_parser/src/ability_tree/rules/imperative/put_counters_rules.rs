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
    /* "put <number> <counter> on <permanent reference>" */
    let put_counters_rules = terminal::Counter::all()
        .map(|counter| ParserRule {
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
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::On {
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
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::On { .. })),
                    ParserNode::Permanent { permanent },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::PutCounters(
                        boseiju_tree::ability_tree::imperative::PutCountersImperative {
                            object: permanent.clone(),
                            counters: {
                                let mut counters = boseiju_tree::HeapArrayVec::new();
                                counters.push(boseiju_tree::ability_tree::imperative::CounterOnPermanent {
                                    amount: number.clone(),
                                    counter: boseiju_tree::ability_tree::imperative::CounterKind::NewCounter(counter.clone()),
                                    #[cfg(feature = "spanned_tree")]
                                    span: number.span().merge(&counter.span),
                                });
                                counters
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: put_span.merge(&permanent.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [put_counters_rules].into_iter().flatten()
}
