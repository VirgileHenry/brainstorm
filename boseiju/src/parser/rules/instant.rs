use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let all_steps = [
        terminals::Step::Untap {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::Upkeep {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::Draw {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::BeginningOfCombat {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::DeclareAttackers {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::DeclareBlockers {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::FirstStrikeDamage {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::Damage {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::LastStrikeDamage {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::EndOfCombat {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::End {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        terminals::Step::Cleanup {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ];
    let steps_to_owned_instants = all_steps.into_iter().flat_map(|step| {
        [
            /* "the beginning of your <step>" is a owned instant */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Beginning {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::Step(step)).id(),
                ]),
                merged: ParserNode::RecurrentInstant { instant: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The {
                            #[cfg(feature = "spanned_tree")]
                                span: start_span,
                        })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Beginning { .. })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of { .. })),
                        ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your {
                            #[cfg(feature = "spanned_tree")]
                                span: player_span,
                        })),
                        ParserNode::LexerToken(Token::Step(step)),
                    ] => Ok(ParserNode::RecurrentInstant {
                        instant: crate::ability_tree::time::RecurrentInstant {
                            step_or_phase: crate::ability_tree::time::StepOrPhase::Step(step.clone()),
                            owner: crate::ability_tree::player::PlayerSpecifier::You {
                                #[cfg(feature = "spanned_tree")]
                                span: *player_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(&step.node_span()),
                        },
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* "the beginning of each <step>" is a owned instant for all players */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Beginning {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::Step(step)).id(),
                ]),
                merged: ParserNode::RecurrentInstant { instant: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The {
                            #[cfg(feature = "spanned_tree")]
                                span: start_span,
                        })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Beginning { .. })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of { .. })),
                        ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                                span: player_span,
                        })),
                        ParserNode::LexerToken(Token::Step(step)),
                    ] => Ok(ParserNode::RecurrentInstant {
                        instant: crate::ability_tree::time::RecurrentInstant {
                            step_or_phase: crate::ability_tree::time::StepOrPhase::Step(step.clone()),
                            owner: crate::ability_tree::player::PlayerSpecifier::All {
                                #[cfg(feature = "spanned_tree")]
                                span: *player_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(&step.node_span()),
                        },
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* "the beginning of your next <step>" is an incoming instant for "you" player */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Beginning {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Next {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::Step(step)).id(),
                ]),
                merged: ParserNode::IncomingInstant { instant: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The {
                            #[cfg(feature = "spanned_tree")]
                                span: start_span,
                        })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Beginning { .. })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of { .. })),
                        ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your {
                            #[cfg(feature = "spanned_tree")]
                                span: player_span,
                        })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Next { .. })),
                        ParserNode::LexerToken(Token::Step(step)),
                    ] => Ok(ParserNode::IncomingInstant {
                        instant: crate::ability_tree::time::IncomingInstant::NextStepOrPhase(
                            crate::ability_tree::time::IncomingNextStepOrPhase {
                                step_or_phase: crate::ability_tree::time::StepOrPhase::Step(step.clone()),
                                owner: crate::ability_tree::player::PlayerSpecifier::You {
                                    #[cfg(feature = "spanned_tree")]
                                    span: *player_span,
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: step.node_span().merge(start_span),
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
