use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
    let power_toughness_modifier_rules = [
        crate::ability_tree::terminals::PowerToughnessModifier::PlusXPlusX,
    crate::ability_tree::terminals::PowerToughnessModifier::PlusXMinusX,
    crate::ability_tree::terminals::PowerToughnessModifier::MinusXPlusX,
    ].into_iter().map(|modifier| super::ParserRule {
        from: super::RuleLhs::new(&[
                ParserNode::ObjectReference{ reference: DummyInit::dummy_init() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)).id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModifier(modifier)).id()
        ]),
        result: ParserNode::ContinuousEffectKind { kind: DummyInit::dummy_init() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)),
                ParserNode::LexerToken(TokenKind::PowerToughnessModifier(modifier))
            ] => Some(ParserNode::ContinuousEffectKind{
                kind:
                crate::ability_tree::ability::statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                    object: reference.clone(),
                    abilities: Box::new(crate::ability_tree::AbilityTree {
                        abilities: {
                            let mut abilities = arrayvec::ArrayVec::new();
                            abilities.push(crate::ability_tree::ability::Ability::Static(
                                crate::ability_tree::ability::statik::StaticAbility::CharasteristicDefiningAbility(
                                    crate::ability_tree::ability::statik::charasteristic_defining_ability::CharacteristicDefiningAbility::PowerToughnessModifier(*modifier)
                                )
                            ));
                            abilities
                        },
                    }),
                }}),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    },)    .collect::<Vec<_>>();

    let duration_to_continuous_effect = [
        crate::ability_tree::terminals::ContinuousEffectDuration::ObjectStaticAbility,
        crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfTurn,
        crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfNextTurn,
    ]
    .into_iter()
    .map(|duration| super::ParserRule {
        from: super::RuleLhs::new(&[
            ParserNode::ContinuousEffectKind {
                kind: DummyInit::dummy_init(),
            }
            .id(),
            ParserNode::LexerToken(TokenKind::ContinuousEffectDuration(duration)).id(),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
        ]),
        result: ParserNode::ContinuousEffect {
            effect: DummyInit::dummy_init(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ContinuousEffectKind { kind },
                ParserNode::LexerToken(TokenKind::ContinuousEffectDuration(duration)),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
            ] => Some(ParserNode::ContinuousEffect {
                effect: crate::ability_tree::ability::statik::continuous_effect::ContinuousEffect {
                    duration: *duration,
                    effect: kind.clone(),
                },
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let gain_ability_tree_rules = [
        crate::ability_tree::terminals::ContinuousEffectDuration::ObjectStaticAbility,
        crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfTurn,
        crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfNextTurn,
    ]
    .into_iter()
    .map(|duration| super::ParserRule {
        from: super::RuleLhs::new(&[
            ParserNode::ObjectReference {
                reference: DummyInit::dummy_init(),
            }
            .id(),
            ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Gain)).id(),
            ParserNode::AbilityTree {
                tree: DummyInit::dummy_init(),
            }
            .id(),
            ParserNode::LexerToken(TokenKind::ContinuousEffectDuration(duration)).id(),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
        ]),
        result: ParserNode::ContinuousEffect {
            effect: DummyInit::dummy_init(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Gain)),
                ParserNode::AbilityTree { tree },
                ParserNode::LexerToken(TokenKind::ContinuousEffectDuration(duration)),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
            ] => Some(ParserNode::ContinuousEffect {
                effect: crate::ability_tree::ability::statik::continuous_effect::ContinuousEffect {
                    duration: *duration,
                    effect:
                        crate::ability_tree::ability::statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                            object: reference.clone(),
                            abilities: tree.clone(),
                        },
                },
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let non_repetitive_rules = vec![
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ObjectReference {
                    reference: DummyInit::dummy_init(),
                }
                .id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)).id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModifier(
                    crate::ability_tree::terminals::PowerToughnessModifier::Constant { power: 0, toughness: 0 },
                ))
                .id(),
            ]),
            result: ParserNode::ContinuousEffectKind {
                kind: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| {
                match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)),
                    ParserNode::LexerToken(TokenKind::PowerToughnessModifier(
                        crate::ability_tree::terminals::PowerToughnessModifier::Constant { power, toughness },
                    )),
                ] => Some(ParserNode::ContinuousEffectKind {
                    kind:
                        crate::ability_tree::ability::statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                            object: reference.clone(),
                            abilities: Box::new(crate::ability_tree::AbilityTree {
                                abilities: {
                                    let mut abilities = arrayvec::ArrayVec::new();
                                    abilities.push(crate::ability_tree::ability::Ability::Static(
                                crate::ability_tree::ability::statik::StaticAbility::CharasteristicDefiningAbility(
                                    crate::ability_tree::ability::statik::charasteristic_defining_ability::CharacteristicDefiningAbility::PowerToughnessModifier(
                                        crate::ability_tree::terminals::PowerToughnessModifier::Constant {
                                            power: *power,
                                            toughness: *toughness
                                        })
                                    )
                                )
                            );
                                    abilities
                                },
                            }),
                        },
                }),
                _ => None,
            }
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ObjectReference {
                    reference: DummyInit::dummy_init(),
                }
                .id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)).id(),
                ParserNode::AbilityTree {
                    tree: DummyInit::dummy_init(),
                }
                .id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
            ]),
            result: ParserNode::ContinuousEffect {
                effect: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| {
                match &nodes {
                &[
                    ParserNode::ObjectReference { reference},
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)),
                    ParserNode::AbilityTree { tree },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot))
                ] => Some(ParserNode::ContinuousEffect{
                    effect: crate::ability_tree::ability::statik::continuous_effect::ContinuousEffect {
                        duration: crate::ability_tree::terminals::ContinuousEffectDuration::ObjectStaticAbility,
                        effect: crate::ability_tree::ability::statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                            object: reference.clone(),
                            abilities: tree.clone(),
                        },
                    }}
                ),
                _ => None,
            }
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        power_toughness_modifier_rules,
        duration_to_continuous_effect,
        gain_ability_tree_rules,
        non_repetitive_rules,
    ]
    .into_iter()
    .flatten()
}
