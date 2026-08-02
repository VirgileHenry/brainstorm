use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_lexer::terminal::ForwardDuration;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::ContinuousEffect;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::continuous_effect_kind;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "Until end of turn, <continuous effect>" makes a generated continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::ForwardDuration(ForwardDuration::UntilEndOfTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ContinuousEffect {
                    effect: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ForwardDuration(ForwardDuration::UntilEndOfTurn {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::ContinuousEffect { effect },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        boseiju_tree::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: effect.clone(),
                            duration: ForwardDuration::UntilEndOfTurn {
                                #[cfg(feature = "spanned_tree")]
                                span: *start_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: effect.span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<continuous effect> until end of turn" makes a generated continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ContinuousEffect {
                    effect: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ForwardDuration(ForwardDuration::UntilEndOfTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ContinuousEffect { effect },
                    ParserNode::LexerToken(Token::ForwardDuration(ForwardDuration::UntilEndOfTurn {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        boseiju_tree::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: effect.clone(),
                            duration: ForwardDuration::UntilEndOfTurn {
                                #[cfg(feature = "spanned_tree")]
                                span: *end_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: effect.span().merge(end_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object reference> this turn has <ability>" makes a generated continuous effect. */
        /* Fixme: this only appears with: "the next <spell specifier> this turn has...", maybe we could be more restrictive */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::BackwardDuration(boseiju_lexer::terminal::BackwardDuration::ThisTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Have {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::BackwardDuration(boseiju_lexer::terminal::BackwardDuration::ThisTurn {
                        #[cfg(feature = "spanned_tree")]
                            span: this_turn_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token:
                            intermediate::EnglishVerb::Have {
                                #[cfg(feature = "spanned_tree")]
                                    span: has_span,
                            },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::KeywordAbility { keyword_ability },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        boseiju_tree::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: ContinuousEffect {
                                effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(
                                    continuous_effect_kind::ModifyObjectEffect {
                                        object: permanent.clone(),
                                        modifications: {
                                            let mut modifications = boseiju_tree::HeapArrayVec::new();
                                            let gain_ab_mod = continuous_effect_kind::ObjectAbilitiesModification::GainAbility(
                                                continuous_effect_kind::ObjectGainAbility {
                                                    ability: boseiju_tree::AbilityTree::from_single_ability(
                                                        boseiju_tree::ability_tree::ability::Ability::KeywordAbility(
                                                            keyword_ability.clone(),
                                                        ),
                                                    ),
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: keyword_ability.span().merge(has_span),
                                                },
                                            );
                                            modifications.push(gain_ab_mod);
                                            modifications
                                        },
                                        #[cfg(feature = "spanned_tree")]
                                        span: permanent.span().merge(&keyword_ability.span()),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(&keyword_ability.span()),
                            },
                            duration: ForwardDuration::UntilEndOfTurn {
                                #[cfg(feature = "spanned_tree")]
                                span: *this_turn_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(&keyword_ability.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
