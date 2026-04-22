use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* All imperatives without an explicit executing player, so the owner of the effect is the executing player */
    let imperative_lists = vec![
        /* An imperative on its own can make a imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Imperative { imperative: dummy() }.id()]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Imperative { imperative }] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imperative.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imperative.node_span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative> and <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative { imperative: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.node_span().merge(&imp2.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative>, then <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative { imperative: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Then {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Then { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.node_span().merge(&imp2.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative>. then <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative { imperative: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Then {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Then { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.node_span().merge(&imp2.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative>. <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative { imperative: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.node_span().merge(&imp2.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative>. <imperative>. <imperative>" can make an imperative list */
        ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Imperative { imperative: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative { imperative: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Imperative { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeList { imperatives: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Imperative { imperative: imp1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { .. })),
                    ParserNode::Imperative { imperative: imp2 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { .. })),
                    ParserNode::Imperative { imperative: imp3 },
                ] => Ok(ParserNode::ImperativeList {
                    imperatives: crate::ability_tree::imperative_list::ImperativeList {
                        imperatives: {
                            let mut imperatives = crate::utils::HeapArrayVec::new();
                            imperatives.push(imp1.clone());
                            imperatives.push(imp2.clone());
                            imperatives.push(imp3.clone());
                            imperatives
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imp1.node_span().merge(&imp3.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [imperative_lists].into_iter().flatten()
}
