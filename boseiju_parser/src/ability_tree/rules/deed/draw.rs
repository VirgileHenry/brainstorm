use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "draw <number>" is an active draw deed with an implicit "you" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::PlayerAction::Draw {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::DeedDrawActive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::PlayerAction::Draw {
                                #[cfg(feature = "spanned_tree")]
                                    span: draw_span,
                            },
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(boseiju_lexer::Token::GameTerm(intermediate::GameTerm::Card {
                        #[cfg(feature = "spanned_tree")]
                            span: card_span,
                    })),
                ] => Ok(ParserNode::DeedDrawActive {
                    deed: boseiju_tree::ability_tree::deed::draw::DrawActive {
                        player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: draw_span.empty_at_start(),
                            },
                        ),
                        amount: number.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: draw_span.merge(card_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active player reference> draws <number>" is an active draw */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PlayerActive {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    token: intermediate::PlayerAction::Draw {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::DeedDrawActive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PlayerActive { player },
                    ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token: intermediate::PlayerAction::Draw { .. },
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(boseiju_lexer::Token::GameTerm(intermediate::GameTerm::Card {
                        #[cfg(feature = "spanned_tree")]
                            span: card_span,
                    })),
                ] => Ok(ParserNode::DeedDrawActive {
                    deed: boseiju_tree::ability_tree::deed::draw::DrawActive {
                        player: player.clone(),
                        amount: number.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(card_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive player reference> draws <number>" is an passive draw */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PlayerPassive {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    token: intermediate::PlayerAction::Draw {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::DeedDrawPassive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PlayerPassive { player },
                    ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                        token: intermediate::PlayerAction::Draw { .. },
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(boseiju_lexer::Token::GameTerm(intermediate::GameTerm::Card {
                        #[cfg(feature = "spanned_tree")]
                            span: card_span,
                    })),
                ] => Ok(ParserNode::DeedDrawPassive {
                    deed: boseiju_tree::ability_tree::deed::draw::DrawPassive {
                        player: player.clone(),
                        amount: number.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(card_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive player reference> draw <number>" is an passive draw */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PlayerPassive {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::PlayerAction::Draw {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::DeedDrawPassive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PlayerPassive { player },
                    ParserNode::LexerToken(boseiju_lexer::Token::PlayerAction(intermediate::TensedPlayerAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token: intermediate::PlayerAction::Draw { .. },
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(boseiju_lexer::Token::GameTerm(intermediate::GameTerm::Card {
                        #[cfg(feature = "spanned_tree")]
                            span: card_span,
                    })),
                ] => Ok(ParserNode::DeedDrawPassive {
                    deed: boseiju_tree::ability_tree::deed::draw::DrawPassive {
                        player: player.clone(),
                        amount: number.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(card_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
