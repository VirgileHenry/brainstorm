use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let terminal_player_rules = vec![
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediates::PlayerSpecifier::All {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player { player: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediates::PlayerSpecifier::AnOpponent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player { player: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::AnOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::AnOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediates::PlayerSpecifier::Any {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player { player: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::Any {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::Any {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediates::PlayerSpecifier::EachOpponent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player { player: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::EachOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::EachOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediates::PlayerSpecifier::TargetOpponent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player { player: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::TargetOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::TargetOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediates::PlayerSpecifier::ToYourLeft {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player { player: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::ToYourLeft {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::ToYourLeft {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediates::PlayerSpecifier::ToYourRight {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player { player: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::ToYourRight {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::ToYourRight {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediates::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player { player: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let non_terminal_player_rules = vec![
        /* Object's controller is a player specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::PermanentReference { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ApostropheS {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::Controller {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Player { player: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentReference { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::Controller {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::ObjectController(
                        crate::ability_tree::player::PlayerSpecifierObjectController {
                            object: Box::new(permanent.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Object's owner is a player specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::CardReference { card: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ApostropheS {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::Owner {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Player { player: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CardReference { card },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::Owner {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::ObjectOwner(
                        crate::ability_tree::player::PlayerSpecifierObjectOwner {
                            object: Box::new(card.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: card.node_span().merge(span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [terminal_player_rules, non_terminal_player_rules].into_iter().flatten()
}
