use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate::ControlFlow;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<imperative cost>" makes up a cost */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::AtomicCost {
                cost: Default::default(),
            }
            .id()]),
            merged: ParserNode::Cost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::AtomicCost { cost }] => Ok(ParserNode::Cost {
                    cost: boseiju_tree::ability_tree::cost::Cost {
                        costs: [cost.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: cost.span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative cost>, <imperative cost>" makes up a cost */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Cost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AtomicCost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::AtomicCost { cost: c2 },
                ] => Ok(ParserNode::Cost {
                    cost: boseiju_tree::ability_tree::cost::Cost {
                        costs: [c1.clone(), c2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: c1.span().merge(&c2.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative cost>, <imperative cost>, <imperative cost>" makes up a cost */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Cost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AtomicCost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::AtomicCost { cost: c2 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::AtomicCost { cost: c3 },
                ] => Ok(ParserNode::Cost {
                    cost: boseiju_tree::ability_tree::cost::Cost {
                        costs: [c1.clone(), c2.clone(), c3.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: c1.span().merge(&c3.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative cost>, <imperative cost>, <imperative cost>,  <imperative cost>" makes up a cost */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Cost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AtomicCost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::AtomicCost { cost: c2 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::AtomicCost { cost: c3 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::AtomicCost { cost: c4 },
                ] => Ok(ParserNode::Cost {
                    cost: boseiju_tree::ability_tree::cost::Cost {
                        costs: [c1.clone(), c2.clone(), c3.clone(), c4.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: c1.span().merge(&c4.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative cost>, <imperative cost>, <imperative cost>, <imperative cost>, <imperative cost>" makes up a cost */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AtomicCost {
                    cost: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Cost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AtomicCost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::AtomicCost { cost: c2 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::AtomicCost { cost: c3 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::AtomicCost { cost: c4 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::AtomicCost { cost: c5 },
                ] => Ok(ParserNode::Cost {
                    cost: boseiju_tree::ability_tree::cost::Cost {
                        costs: [c1.clone(), c2.clone(), c3.clone(), c4.clone(), c5.clone()]
                            .into_iter()
                            .collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: c1.span().merge(&c5.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
