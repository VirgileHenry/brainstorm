use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    let terminal_player_rules = vec![
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Player {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Player {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: boseiju_tree::ability_tree::player::PlayerSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Any {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Opponent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Any {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Opponent {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: boseiju_tree::ability_tree::player::PlayerSpecifier::AnOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Any {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Player {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Any {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Player {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: boseiju_tree::ability_tree::player::PlayerSpecifier::Any {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Opponent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Player {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: boseiju_tree::ability_tree::player::PlayerSpecifier::EachOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::Target {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Opponent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::Target {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Opponent {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: boseiju_tree::ability_tree::player::PlayerSpecifier::TargetOpponent {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(
                intermediate::PlayerSpecifier::ToYourLeft {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::ToYourLeft {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: boseiju_tree::ability_tree::player::PlayerSpecifier::ToYourLeft {
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
                intermediate::PlayerSpecifier::ToYourRight {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::ToYourRight {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: boseiju_tree::ability_tree::player::PlayerSpecifier::ToYourRight {
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
                intermediate::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: boseiju_tree::ability_tree::player::PlayerSpecifier::You {
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
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::ApostropheS {
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
            merged: ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Controller {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: boseiju_tree::ability_tree::player::PlayerSpecifier::ObjectController(
                        boseiju_tree::ability_tree::player::ObjectController {
                            object: Box::new(permanent.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(span),
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
                ParserNode::Card {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::ApostropheS {
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
            merged: ParserNode::Player {
                player: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Card { card },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::PlayerSpecifier(intermediate::PlayerSpecifier::Owner {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Player {
                    player: boseiju_tree::ability_tree::player::PlayerSpecifier::ObjectOwner(
                        boseiju_tree::ability_tree::player::ObjectOwner {
                            object: Box::new(card.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: card.span().merge(span),
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
