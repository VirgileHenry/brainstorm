use crate::lexer::tokens::Token;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* The special tap cost "{T}" is an imperative cost to tap self */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::TapUntapCost(
                crate::lexer::tokens::intermediates::TapUntapCost::Tap {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::ImperativeAsCost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TapUntapCost(crate::lexer::tokens::intermediates::TapUntapCost::Tap {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::ImperativeAsCost {
                    cost: crate::ability_tree::imperative::Imperative {
                        kind: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                            crate::ability_tree::imperative::KeywordAction {
                                keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Tap(
                                    crate::ability_tree::imperative::tap::TapKeywordAction {
                                        permanent: crate::ability_tree::object::Permanent::SelfReferencing(
                                            crate::ability_tree::object::SelfReferencing {
                                                #[cfg(feature = "spanned_tree")]
                                                span: *span,
                                            },
                                        ),
                                        #[cfg(feature = "spanned_tree")]
                                        span: *span,
                                    },
                                ),
                                ability: crate::ability_tree::imperative::tap::ability(
                                    &crate::ability_tree::object::Permanent::SelfReferencing(
                                        crate::ability_tree::object::SelfReferencing {
                                            #[cfg(feature = "spanned_tree")]
                                            span: *span,
                                        },
                                    ),
                                    #[cfg(feature = "spanned_tree")]
                                    *span,
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: *span,
                            },
                        ),
                        executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                            #[cfg(feature = "spanned_tree")]
                            span: span.empty_at_end(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: span.clone(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* The special untap cost "{Q}" is an imperative cost to untap self */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::TapUntapCost(
                crate::lexer::tokens::intermediates::TapUntapCost::Untap {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::ImperativeAsCost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TapUntapCost(crate::lexer::tokens::intermediates::TapUntapCost::Untap {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::ImperativeAsCost {
                    cost: crate::ability_tree::imperative::Imperative {
                        kind: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                            crate::ability_tree::imperative::KeywordAction {
                                keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Untap(
                                    crate::ability_tree::imperative::untap::UntapKeywordAction {
                                        permanent: crate::ability_tree::object::Permanent::SelfReferencing(
                                            crate::ability_tree::object::SelfReferencing {
                                                #[cfg(feature = "spanned_tree")]
                                                span: *span,
                                            },
                                        ),
                                        #[cfg(feature = "spanned_tree")]
                                        span: *span,
                                    },
                                ),
                                ability: crate::ability_tree::imperative::untap::ability(
                                    &crate::ability_tree::object::Permanent::SelfReferencing(
                                        crate::ability_tree::object::SelfReferencing {
                                            #[cfg(feature = "spanned_tree")]
                                            span: *span,
                                        },
                                    ),
                                    #[cfg(feature = "spanned_tree")]
                                    *span,
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: *span,
                            },
                        ),
                        executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                            #[cfg(feature = "spanned_tree")]
                            span: span.empty_at_end(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: span.clone(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
