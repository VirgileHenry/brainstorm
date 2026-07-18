use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use crate::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* All imperatives without an explicit executing player, so the owner of the effect is the executing player */
    let imperative_lists = vec![
        /* An imperative on its own can make a imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Imperative {
                imperative: Default::default(),
            }
            .id()]),
            merged: ParserNode::ImperativeList {
                imperatives: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Imperative { imperative }] => Ok(ParserNode::ImperativeList {
                    imperatives: boseiju_tree::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = boseiju_tree::HeapArrayVec::new();
                            imperatives.push(imperative.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imperative.span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative> and <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeList {
                imperatives: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::And { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: boseiju_tree::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = boseiju_tree::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.span().merge(&imp2.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative>, then <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Then {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeList {
                imperatives: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Then { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: boseiju_tree::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = boseiju_tree::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.span().merge(&imp2.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative>. then <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Then {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeList {
                imperatives: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Then { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: boseiju_tree::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = boseiju_tree::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.span().merge(&imp2.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative>. <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeList {
                imperatives: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: boseiju_tree::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = boseiju_tree::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.span().merge(&imp2.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative>. <imperative>. <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative {
                    imperative: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeList {
                imperatives: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::Imperative { imperative: imp3 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: boseiju_tree::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = boseiju_tree::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives.push(imp3.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.span().merge(&imp3.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [imperative_lists].into_iter().flatten()
}
