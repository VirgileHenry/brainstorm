use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_lexer::terminal;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::ContinuousEffect;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind;
use boseiju_tree::ability_tree::replacement_effect;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    let default_etb_replacements = vec![
        /* "<permanent reference> enter <permanent state>" is a replacement effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardState(intermediate::CardState::Tapped {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                            span: enters_span,
                    })),
                    ParserNode::LexerToken(Token::CardState(intermediate::CardState::Tapped {
                        #[cfg(feature = "spanned_tree")]
                            span: tapped_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ReplacementEffect(replacement_effect::ReplacementEffect::Etb(
                            replacement_effect::EtbReplacementEffect {
                                etb_event: boseiju_tree::ability_tree::action::PermanentEtbAction {
                                    permanent: permanent.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: permanent.span().merge(enters_span),
                                },
                                etb_modifiers: [replacement_effect::EtbModifier::WithState(replacement_effect::EtbWithState {
                                    state: boseiju_tree::ability_tree::state::PermanentState::Tapped(
                                        boseiju_tree::ability_tree::state::PermanentTappedState {
                                            #[cfg(feature = "spanned_tree")]
                                            span: *tapped_span,
                                        },
                                    ),
                                    #[cfg(feature = "spanned_tree")]
                                    span: *tapped_span,
                                })]
                                .into_iter()
                                .collect(),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(tapped_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(tapped_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "as <permanent reference> enters, <spell ability>" is an etb perform action */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::As {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility {
                    ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::As {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                            span: enters_span,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ReplacementEffect(replacement_effect::ReplacementEffect::Etb(
                            replacement_effect::EtbReplacementEffect {
                                etb_event: boseiju_tree::ability_tree::action::PermanentEtbAction {
                                    permanent: permanent.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: permanent.span().merge(enters_span),
                                },
                                etb_modifiers: [replacement_effect::EtbModifier::PerformAction(
                                    replacement_effect::EtbPerformAction {
                                        action: ability.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: ability.span().merge(start_span),
                                    },
                                )]
                                .into_iter()
                                .collect(),
                                #[cfg(feature = "spanned_tree")]
                                span: ability.span().merge(start_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: ability.span().merge(start_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let etb_with_counter = terminal::Counter::all()
        .map(|counter|
        /* "<object reference> enters with <number> <counter> on it" is a replacement effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: Default::default() }.id(),
                ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: Default::default() }.id(),
                ParserNode::LexerToken(Token::Counter(counter))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::On {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: Default::default() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                        span: enters_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::With {
                        #[cfg(feature = "spanned_tree")]
                        span: with_span,
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::Counter(counter)),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::On { .. })),
                    ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                        #[cfg(feature = "spanned_tree")]
                        span: end_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ReplacementEffect(
                            replacement_effect::ReplacementEffect::Etb(
                                replacement_effect::EtbReplacementEffect {
                                etb_event: boseiju_tree::ability_tree::action::PermanentEtbAction {
                                    permanent: permanent.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: permanent.span().merge(enters_span),
                                },
                                etb_modifiers: {
                                    let mut modifiers = boseiju_tree::HeapArrayVec::new();
                                    modifiers.push(replacement_effect::EtbModifier::WithCounters(
                                        replacement_effect::EtbWithCounters {
                                            counter_kind: counter.clone(),
                                            amount: number.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                            span: counter.span().merge(with_span) },
                                    ));
                                    modifiers
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(end_span),
                            }),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [default_etb_replacements, etb_with_counter].into_iter().flatten()
}
