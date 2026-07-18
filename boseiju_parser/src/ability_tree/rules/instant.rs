use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_lexer::terminal;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    let all_steps = [
        terminal::Step::Untap {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::Upkeep {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::Draw {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::BeginningOfCombat {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::DeclareAttackers {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::DeclareBlockers {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::FirstStrikeDamage {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::Damage {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::LastStrikeDamage {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::EndOfCombat {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::End {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminal::Step::Cleanup {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ];
    let steps_to_owned_instants = all_steps.into_iter().flat_map(|step| {
        [
            /* "the beginning of your <step>" is a owned instant */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::The {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Beginning {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Of {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::Step(step)).id(),
                ]),
                merged: ParserNode::RecurrentInstant {
                    instant: Default::default(),
                }
                .id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::The {
                            #[cfg(feature = "spanned_tree")]
                                span: start_span,
                        })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Beginning { .. })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Of { .. })),
                        ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                            #[cfg(feature = "spanned_tree")]
                                span: player_span,
                        })),
                        ParserNode::LexerToken(Token::Step(step)),
                    ] => Ok(ParserNode::RecurrentInstant {
                        instant: boseiju_tree::ability_tree::time::RecurrentInstant {
                            step_or_phase: boseiju_tree::ability_tree::time::StepOrPhase::Step(step.clone()),
                            owner: boseiju_tree::ability_tree::player::PlayerSpecifier::You {
                                #[cfg(feature = "spanned_tree")]
                                span: *player_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(&step.span()),
                        },
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* "the beginning of each <step>" is a owned instant for all players */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::The {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Beginning {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Of {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::Step(step)).id(),
                ]),
                merged: ParserNode::RecurrentInstant {
                    instant: Default::default(),
                }
                .id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::The {
                            #[cfg(feature = "spanned_tree")]
                                span: start_span,
                        })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Beginning { .. })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Of { .. })),
                        ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                                span: player_span,
                        })),
                        ParserNode::LexerToken(Token::Step(step)),
                    ] => Ok(ParserNode::RecurrentInstant {
                        instant: boseiju_tree::ability_tree::time::RecurrentInstant {
                            step_or_phase: boseiju_tree::ability_tree::time::StepOrPhase::Step(step.clone()),
                            owner: boseiju_tree::ability_tree::player::PlayerSpecifier::All {
                                #[cfg(feature = "spanned_tree")]
                                span: *player_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(&step.span()),
                        },
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* "the beginning of your next <step>" is an incoming instant for "you" player */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::The {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Beginning {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Of {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Next {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::Step(step)).id(),
                ]),
                merged: ParserNode::IncomingInstant {
                    instant: Default::default(),
                }
                .id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::The {
                            #[cfg(feature = "spanned_tree")]
                                span: start_span,
                        })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Beginning { .. })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Of { .. })),
                        ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                            #[cfg(feature = "spanned_tree")]
                                span: player_span,
                        })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Next { .. })),
                        ParserNode::LexerToken(Token::Step(step)),
                    ] => Ok(ParserNode::IncomingInstant {
                        instant: boseiju_tree::ability_tree::time::IncomingInstant::NextStepOrPhase(
                            boseiju_tree::ability_tree::time::IncomingNextStepOrPhase {
                                step_or_phase: boseiju_tree::ability_tree::time::StepOrPhase::Step(step.clone()),
                                owner: boseiju_tree::ability_tree::player::PlayerSpecifier::You {
                                    #[cfg(feature = "spanned_tree")]
                                    span: *player_span,
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: step.span().merge(start_span),
                            },
                        ),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
        ]
    });

    [steps_to_owned_instants].into_iter().flatten()
}
