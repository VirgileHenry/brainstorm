use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let terminal_player_rules = [
        intermediates::PlayerSpecifier::All {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        intermediates::PlayerSpecifier::Any {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        intermediates::PlayerSpecifier::EachOpponent {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        intermediates::PlayerSpecifier::TargetOpponent {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        intermediates::PlayerSpecifier::ToYourLeft {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        intermediates::PlayerSpecifier::ToYourRight {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        intermediates::PlayerSpecifier::You {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|player| super::ParserRule {
        expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(player)).id()]),
        merged: ParserNode::Player { player: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(Token::PlayerSpecifier(player))] => Ok(ParserNode::Player {
                player: match player {
                    crate::lexer::tokens::intermediates::PlayerSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::player::PlayerSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                    crate::lexer::tokens::intermediates::PlayerSpecifier::Any {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::player::PlayerSpecifier::Any {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                    crate::lexer::tokens::intermediates::PlayerSpecifier::EachOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::player::PlayerSpecifier::EachOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                    crate::lexer::tokens::intermediates::PlayerSpecifier::TargetOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::player::PlayerSpecifier::TargetOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                    crate::lexer::tokens::intermediates::PlayerSpecifier::ToYourLeft {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::player::PlayerSpecifier::ToYourLeft {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                    crate::lexer::tokens::intermediates::PlayerSpecifier::ToYourRight {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::player::PlayerSpecifier::ToYourRight {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                    crate::lexer::tokens::intermediates::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::player::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                    _ => return Err("Unreachable in player rules"),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let non_terminal_player_rules = vec![
        /* Object's controller is a player specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Possessive {
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
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Possessive { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::Controller {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::ObjectController(
                        crate::ability_tree::player::PlayerSpecifierObjectController {
                            object: Box::new(reference.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(span),
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
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Possessive {
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
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Possessive { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::Owner {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: crate::ability_tree::player::PlayerSpecifier::ObjectOwner(
                        crate::ability_tree::player::PlayerSpecifierObjectOwner {
                            object: Box::new(reference.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(span),
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
