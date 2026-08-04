use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "the top cards of <player>'s library" is a card reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AdverbialPositional(intermediate::AdverbialPositional::Top {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::ApostropheS {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Card {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::AdverbialPositional(intermediate::AdverbialPositional::Top { .. })),
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                        #[cfg(feature = "spanned_tree")]
                            span: card_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of { .. })),
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Card {
                    card: object::Card::TopCardsOfLibrary(object::TopCardsOfLibrary {
                        amount: boseiju_tree::ability_tree::number::Number::Number(
                            boseiju_tree::ability_tree::number::FixedNumber {
                                number: 1,
                                #[cfg(feature = "spanned_tree")]
                                span: card_span.empty_at_start(),
                            },
                        ),
                        player: player.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "the top cards of your library" is a card reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AdverbialPositional(intermediate::AdverbialPositional::Top {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Card {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::AdverbialPositional(intermediate::AdverbialPositional::Top { .. })),
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                        #[cfg(feature = "spanned_tree")]
                            span: card_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                        #[cfg(feature = "spanned_tree")]
                            span: your_span,
                    })),
                    ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Card {
                    card: object::Card::TopCardsOfLibrary(object::TopCardsOfLibrary {
                        amount: boseiju_tree::ability_tree::number::Number::Number(
                            boseiju_tree::ability_tree::number::FixedNumber {
                                number: 1,
                                #[cfg(feature = "spanned_tree")]
                                span: card_span.empty_at_start(),
                            },
                        ),
                        player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: *your_span,
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "the top cards of their library" is a card reference */
        /* Fixme: need context to properly parse this
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AdverbialPositional(intermediate::AdverbialPositional::Top {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPossessive(intermediate::EnglishPossessive::Their {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Card {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::AdverbialPositional(intermediate::AdverbialPositional::Top { .. })),
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                        #[cfg(feature = "spanned_tree")]
                            span: card_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of { .. })),
                    ParserNode::LexerToken(Token::EnglishPossessive(intermediate::EnglishPossessive::Their {
                        #[cfg(feature = "spanned_tree")]
                            span: their_span,
                    })),
                    ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Card {
                    card: object::Card::TopCardsOfLibrary(object::TopCardsOfLibrary {
                        amount: boseiju_tree::ability_tree::number::Number::Number(
                            boseiju_tree::ability_tree::number::FixedNumber {
                                number: 1,
                                #[cfg(feature = "spanned_tree")]
                                span: card_span.empty_at_start(),
                            },
                        ),
                        player: boseiju_tree::ability_tree::player::PlayerReference::PerviouslyMentionnedPlayer {
                            #[cfg(feature = "spanned_tree")]
                            span: *their_span,
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        }, */
        /* "the top <number> cards of <player>'s library" is a card reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AdverbialPositional(intermediate::AdverbialPositional::Top {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::ApostropheS {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Card {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::AdverbialPositional(intermediate::AdverbialPositional::Top { .. })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card { .. })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of { .. })),
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Card {
                    card: object::Card::TopCardsOfLibrary(object::TopCardsOfLibrary {
                        amount: number.clone(),
                        player: player.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "the top <number> cards of your library" is a card reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AdverbialPositional(intermediate::AdverbialPositional::Top {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Card {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::AdverbialPositional(intermediate::AdverbialPositional::Top { .. })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card { .. })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                        #[cfg(feature = "spanned_tree")]
                            span: your_span,
                    })),
                    ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Card {
                    card: object::Card::TopCardsOfLibrary(object::TopCardsOfLibrary {
                        amount: number.clone(),
                        player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: *your_span,
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
