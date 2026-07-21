use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "if <condition>, <imperative>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition {
                    condition: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::ImperativeList { imperatives },
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::ConditionalImperative(
                        boseiju_tree::ability_tree::statement::ConditionalImperative {
                            condition: boseiju_tree::ability_tree::conditional::Conditional::If(
                                boseiju_tree::ability_tree::conditional::ConditionalIf {
                                    condition: condition.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: condition.span().merge(if_span),
                                },
                            ),
                            condition_met_clause: imperatives.clone(),
                            cond_not_met_clause: None,
                            #[cfg(feature = "spanned_tree")]
                            span: imperatives.span().merge(if_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<imperative>. if <condition>, <imperative> instead" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition {
                    condition: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList {
                    imperatives: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Instead {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Statement {
                statement: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeList { imperatives: imp1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::ImperativeList { imperatives: imp2 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Instead {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Statement {
                    statement: boseiju_tree::ability_tree::statement::Statement::ConditionalImperative(
                        boseiju_tree::ability_tree::statement::ConditionalImperative {
                            condition: boseiju_tree::ability_tree::conditional::Conditional::If(
                                boseiju_tree::ability_tree::conditional::ConditionalIf {
                                    condition: condition.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: condition.span().merge(if_span),
                                },
                            ),
                            condition_met_clause: imp2.clone(),
                            cond_not_met_clause: Some(imp1.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: imp1.span().merge(end_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
