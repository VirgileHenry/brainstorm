use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* The special tap cost "{T}" is an imperative cost to tap self */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::TapUntapCost(
                boseiju_lexer::intermediate::TapUntapCost::Tap {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::ImperativeAsCost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TapUntapCost(boseiju_lexer::intermediate::TapUntapCost::Tap {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::ImperativeAsCost {
                    cost: boseiju_tree::ability_tree::imperative::Imperative {
                        kind: boseiju_tree::ability_tree::imperative::ImperativeKind::KeywordAction(
                            boseiju_tree::ability_tree::imperative::KeywordAction {
                                keyword: boseiju_tree::ability_tree::imperative::ExpandedKeywordAction::Tap(
                                    boseiju_tree::ability_tree::imperative::tap::TapKeywordAction {
                                        permanent: boseiju_tree::ability_tree::object::Permanent::SelfReferencing(
                                            boseiju_tree::ability_tree::object::SelfReferencing {
                                                #[cfg(feature = "spanned_tree")]
                                                span: *span,
                                            },
                                        ),
                                        #[cfg(feature = "spanned_tree")]
                                        span: *span,
                                    },
                                ),
                                ability: boseiju_tree::ability_tree::imperative::tap::ability(
                                    &boseiju_tree::ability_tree::object::Permanent::SelfReferencing(
                                        boseiju_tree::ability_tree::object::SelfReferencing {
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
                        executing_player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: span.empty_at_start(),
                            },
                        ),
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
                boseiju_lexer::intermediate::TapUntapCost::Untap {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::ImperativeAsCost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TapUntapCost(boseiju_lexer::intermediate::TapUntapCost::Untap {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::ImperativeAsCost {
                    cost: boseiju_tree::ability_tree::imperative::Imperative {
                        kind: boseiju_tree::ability_tree::imperative::ImperativeKind::KeywordAction(
                            boseiju_tree::ability_tree::imperative::KeywordAction {
                                keyword: boseiju_tree::ability_tree::imperative::ExpandedKeywordAction::Untap(
                                    boseiju_tree::ability_tree::imperative::untap::UntapKeywordAction {
                                        permanent: boseiju_tree::ability_tree::object::Permanent::SelfReferencing(
                                            boseiju_tree::ability_tree::object::SelfReferencing {
                                                #[cfg(feature = "spanned_tree")]
                                                span: *span,
                                            },
                                        ),
                                        #[cfg(feature = "spanned_tree")]
                                        span: *span,
                                    },
                                ),
                                ability: boseiju_tree::ability_tree::imperative::untap::ability(
                                    &boseiju_tree::ability_tree::object::Permanent::SelfReferencing(
                                        boseiju_tree::ability_tree::object::SelfReferencing {
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
                        executing_player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: span.empty_at_end(),
                            },
                        ),
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
