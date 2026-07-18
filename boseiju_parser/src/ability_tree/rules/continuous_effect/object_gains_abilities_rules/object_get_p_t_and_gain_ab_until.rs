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
    [/* "<object> gets +n/+n and has <ability>" */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::Creature {
                creature: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::TensedActionKeyword(intermediate::TensedActionKeyword {
                token: intermediate::ActionKeyword::Get {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
            }))
            .id(),
            ParserNode::PowerToughnessModifiers {
                modifiers: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::And {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
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
                ParserNode::Creature { creature },
                ParserNode::LexerToken(Token::TensedActionKeyword(intermediate::TensedActionKeyword {
                    token: intermediate::ActionKeyword::Get { .. },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                })),
                ParserNode::PowerToughnessModifiers { modifiers },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::And { .. })),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                        span: gain_ab_span,
                })),
                ParserNode::Ability { ability },
                ParserNode::ForwardDuration { duration },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                    boseiju_tree::ability_tree::imperative::GenerateContinuousEffectImperative {
                        effect: ContinuousEffect {
                            effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(continuous_effect_kind::ModifyObjectEffect {
                                object: creature.to_permanent(),
                                modifications: {
                                    let mut modifications = boseiju_tree::HeapArrayVec::new();
                                    modifications.push(continuous_effect_kind::ObjectAbilitiesModification::CharacteristicModification(
                                        continuous_effect_kind::ObjectCharacteristicModification::PowerToughnessModifiers(modifiers.clone()),
                                    ));
                                    modifications.push(continuous_effect_kind::ObjectAbilitiesModification::GainAbility(continuous_effect_kind::ObjectGainAbility {
                                        ability: boseiju_tree::AbilityTree::from_single_ability(ability.clone()),
                                        #[cfg(feature = "spanned_tree")]
                                        span: gain_ab_span.merge(&ability.span()),
                                    }));
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(&ability.span()),
                            }),
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().merge(&ability.span()),
                        },
                        duration: duration.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(&duration.span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
