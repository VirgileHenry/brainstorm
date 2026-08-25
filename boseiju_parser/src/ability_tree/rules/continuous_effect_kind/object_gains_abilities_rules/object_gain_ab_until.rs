use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::ContinuousEffect;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::continuous_effect_kind;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<permanent reference> gain <ability> <forward duration>" is a generate continuous effect imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentActive {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability {
                    ability: Default::default(),
                }
                .id(),
                ParserNode::ForwardDuration {
                    duration: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentActive { permanent },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Gain {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::Ability { ability },
                    ParserNode::ForwardDuration { duration },
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
                                                    ability: boseiju_tree::AbilityTree::from_single_ability(ability.clone()),
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: span.merge(&ability.span()),
                                                },
                                            );
                                            modifications.push(gain_ab_mod);
                                            modifications
                                        },
                                        #[cfg(feature = "spanned_tree")]
                                        span: permanent.span().merge(&ability.span()),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(&duration.span()),
                            },
                            duration: duration.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(&duration.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> gain <ability>" is forever. It usually happens when the object also get sacrificed at some point */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability {
                    ability: Default::default(),
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
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Gain {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::Ability { ability },
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
                                                    ability: boseiju_tree::AbilityTree::from_single_ability(ability.clone()),
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: span.merge(&ability.span()),
                                                },
                                            );
                                            modifications.push(gain_ab_mod);
                                            modifications
                                        },
                                        #[cfg(feature = "spanned_tree")]
                                        span: permanent.span().merge(&ability.span()),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(&ability.span()),
                            },
                            duration: boseiju_lexer::terminal::ForwardDuration::Forever {
                                #[cfg(feature = "spanned_tree")]
                                span: ability.span().empty_at_end(),
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(&ability.span()),
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
