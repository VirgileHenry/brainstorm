use crate::ability_tree::ability::statik::continuous_effect::*;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [/* "<object> gets +n/+n and has <ability>" */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::CreatureReference { creature: dummy() }.id(),
            ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::PowerToughnessModifiers { modifiers: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Ability { ability: dummy() }.id(),
            ParserNode::ForwardDuration { duration: dummy() }.id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::CreatureReference { creature },
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                ParserNode::PowerToughnessModifiers { modifiers },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                        span: gain_ab_span,
                })),
                ParserNode::Ability { ability },
                ParserNode::ForwardDuration { duration },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                    crate::ability_tree::imperative::GenerateContinuousEffectImperative {
                        effect: ContinuousEffect {
                            effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                                object: crate::ability_tree::object::PermanentReference::Creature(creature.clone()),
                                modifications: {
                                    let mut modifications = crate::utils::HeapArrayVec::new();
                                    modifications.push(ObjectAbilitiesModification::CharacteristicModification(
                                        ObjectCharacteristicModification::PowerToughnessModifiers(modifiers.clone()),
                                    ));
                                    modifications.push(ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                        ability: ability.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: gain_ab_span.merge(&ability.node_span()),
                                    }));
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: creature.node_span().merge(&ability.node_span()),
                            }),
                            #[cfg(feature = "spanned_tree")]
                            span: creature.node_span().merge(&ability.node_span()),
                        },
                        duration: duration.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(&duration.node_span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
