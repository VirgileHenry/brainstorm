use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
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
        /* "if <condition>, <imperative>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition { condition: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
            ]),
            merged: ParserNode::Statement { statement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::ImperativeList { imperatives },
                ] => Ok(ParserNode::Statement {
                    statement: crate::ability_tree::statement::Statement::ConditionalImperative(
                        crate::ability_tree::statement::ConditionalImperative {
                            condition: crate::ability_tree::conditional::Conditional::If(
                                crate::ability_tree::conditional::ConditionalIf {
                                    condition: condition.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: condition.node_span().merge(if_span),
                                },
                            ),
                            condition_met_clause: imperatives.clone(),
                            cond_not_met_clause: None,
                            #[cfg(feature = "spanned_tree")]
                            span: imperatives.node_span().merge(if_span),
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
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition { condition: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Instead {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Statement { statement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeList { imperatives: imp1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::ImperativeList { imperatives: imp2 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Instead {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Statement {
                    statement: crate::ability_tree::statement::Statement::ConditionalImperative(
                        crate::ability_tree::statement::ConditionalImperative {
                            condition: crate::ability_tree::conditional::Conditional::If(
                                crate::ability_tree::conditional::ConditionalIf {
                                    condition: condition.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: condition.node_span().merge(if_span),
                                },
                            ),
                            condition_met_clause: imp2.clone(),
                            cond_not_met_clause: Some(imp1.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: imp1.node_span().merge(end_span),
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
