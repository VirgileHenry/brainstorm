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
    /* "put <number> <counter> on <permanent reference>" */
    let put_counters_rules = terminals::Counter::all()
        .map(|counter| ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::Counter(counter)).id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Permanent { permanent: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Put {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::Counter(counter)),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On { .. })),
                    ParserNode::Permanent { permanent },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::PutCounters(
                        crate::ability_tree::imperative::PutCountersImperative {
                            object: permanent.clone(),
                            counters: {
                                let mut counters = crate::utils::HeapArrayVec::new();
                                counters.push(crate::ability_tree::imperative::CounterOnPermanent {
                                    amount: number.clone(),
                                    counter: crate::ability_tree::imperative::CounterKind::NewCounter(counter.clone()),
                                    #[cfg(feature = "spanned_tree")]
                                    span: number.node_span().merge(&counter.span),
                                });
                                counters
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: span.merge(&permanent.node_span()),
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
