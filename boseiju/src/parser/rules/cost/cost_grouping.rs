use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates::ControlFlow;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<imperative cost>" makes up a cost */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ImperativeAsCost { cost: dummy() }.id()]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ImperativeAsCost { cost }] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost {
                        costs: [cost.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: cost.node_span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative cost>, <imperative cost>" makes up a cost */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
            ]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeAsCost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::ImperativeAsCost { cost: c2 },
                ] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost {
                        costs: [c1.clone(), c2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: c1.node_span().merge(&c2.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative cost>, <imperative cost>, <imperative cost>" makes up a cost */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
            ]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeAsCost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::ImperativeAsCost { cost: c2 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::ImperativeAsCost { cost: c3 },
                ] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost {
                        costs: [c1.clone(), c2.clone(), c3.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: c1.node_span().merge(&c3.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative cost>, <imperative cost>, <imperative cost>,  <imperative cost>" makes up a cost */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
            ]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeAsCost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::ImperativeAsCost { cost: c2 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::ImperativeAsCost { cost: c3 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::ImperativeAsCost { cost: c4 },
                ] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost {
                        costs: [c1.clone(), c2.clone(), c3.clone(), c4.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: c1.node_span().merge(&c4.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative cost>, <imperative cost>, <imperative cost>, <imperative cost>, <imperative cost>" makes up a cost */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeAsCost { cost: dummy() }.id(),
            ]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeAsCost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::ImperativeAsCost { cost: c2 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::ImperativeAsCost { cost: c3 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::ImperativeAsCost { cost: c4 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::ImperativeAsCost { cost: c5 },
                ] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost {
                        costs: [c1.clone(), c2.clone(), c3.clone(), c4.clone(), c5.clone()]
                            .into_iter()
                            .collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: c1.node_span().merge(&c5.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
