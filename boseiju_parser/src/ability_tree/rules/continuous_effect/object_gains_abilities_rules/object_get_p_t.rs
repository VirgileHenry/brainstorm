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
    /* "<creature> get <power toughness modifiers>" */
    [ParserRule {
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
        ]),
        merged: ParserNode::ContinuousEffect {
            effect: Default::default(),
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
            ] => Ok(ParserNode::ContinuousEffect {
                effect: ContinuousEffect {
                    effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(
                        continuous_effect_kind::ModifyObjectEffect {
                            object: creature.to_permanent(),
                            modifications: {
                                let mut modifications = boseiju_tree::HeapArrayVec::new();
                                let characteristic_mod =
                                    continuous_effect_kind::ObjectAbilitiesModification::CharacteristicModification(
                                        continuous_effect_kind::ObjectCharacteristicModification::PowerToughnessModifiers(
                                            modifiers.clone(),
                                        ),
                                    );
                                modifications.push(characteristic_mod);
                                modifications
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().merge(&modifiers.span()),
                        },
                    ),
                    #[cfg(feature = "spanned_tree")]
                    span: creature.span().merge(&modifiers.span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
