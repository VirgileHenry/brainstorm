use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::player;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "You" as an active specified player */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::PlayerActive {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                            span: player_span,
                    })),
                ] => Ok(ParserNode::PlayerActive {
                    player: player::PlayerReference::You(player::You {
                        #[cfg(feature = "spanned_tree")]
                        span: *player_span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "You" as an passive specified player */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::PlayerPassive {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                            span: player_span,
                    })),
                ] => Ok(ParserNode::PlayerPassive {
                    player: player::PlayerReference::You(player::You {
                        #[cfg(feature = "spanned_tree")]
                        span: *player_span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<active quantifier> player" makes for an active specified player */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::QuantifierActive {
                    count: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Player {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::PlayerActive {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::QuantifierActive { count },
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Player {
                        #[cfg(feature = "spanned_tree")]
                            span: player_span,
                    })),
                ] => Ok(ParserNode::PlayerActive {
                    player: player::PlayerReference::SpecifiedPlayer(player::SpecifiedPlayer {
                        count: count.clone(),
                        specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(player_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<passive quantifier> player" makes for an passive specified player */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::QuantifierPassive {
                    count: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Player {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::PlayerPassive {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::QuantifierPassive { count },
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Player {
                        #[cfg(feature = "spanned_tree")]
                            span: player_span,
                    })),
                ] => Ok(ParserNode::PlayerPassive {
                    player: player::PlayerReference::SpecifiedPlayer(player::SpecifiedPlayer {
                        count: count.clone(),
                        specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(player_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<active quantifier> opponent" makes for an active specified player with the "opponent" specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::QuantifierActive {
                    count: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Opponent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::PlayerActive {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::QuantifierActive { count },
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Opponent {
                        #[cfg(feature = "spanned_tree")]
                            span: opponent_span,
                    })),
                ] => Ok(ParserNode::PlayerActive {
                    player: player::PlayerReference::SpecifiedPlayer(player::SpecifiedPlayer {
                        count: count.clone(),
                        specifiers: Some(boseiju_tree::ability_tree::object::specified_object::Specifiers::Single(
                            player::player_specifier::PlayerSpecifier::Opponent(player::player_specifier::OpponentSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *opponent_span,
                            }),
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(opponent_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<passive quantifier> opponent" makes for a passive specified player with the "opponent" specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::QuantifierPassive {
                    count: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Opponent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::PlayerPassive {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::QuantifierPassive { count },
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Opponent {
                        #[cfg(feature = "spanned_tree")]
                            span: opponent_span,
                    })),
                ] => Ok(ParserNode::PlayerPassive {
                    player: player::PlayerReference::SpecifiedPlayer(player::SpecifiedPlayer {
                        count: count.clone(),
                        specifiers: Some(boseiju_tree::ability_tree::object::specified_object::Specifiers::Single(
                            player::player_specifier::PlayerSpecifier::Opponent(player::player_specifier::OpponentSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *opponent_span,
                            }),
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(opponent_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Object's controller is a player specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::PermanentActive {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::ApostropheS {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Controller {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::PlayerActive {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentActive { permanent },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Controller {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::PlayerActive {
                    player: player::PlayerReference::ObjectController(player::ObjectController {
                        object: Box::new(permanent.clone()),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Object's owner is a player specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::CardActive {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::ApostropheS {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Owner {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::PlayerActive {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CardActive { card },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Owner {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::PlayerActive {
                    player: player::PlayerReference::ObjectOwner(player::ObjectOwner {
                        object: Box::new(card.clone()),
                        #[cfg(feature = "spanned_tree")]
                        span: card.span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
