use crate::ability_tree::ability::statik::continuous_effect;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::lexer::tokens::non_terminals::PowerToughnessModElements;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = ParserRule> {
    /* Object can "get" a power / toughness modifier */
    let power_toughness_modifier_rules = vec![
        /* Object gets straight +x/+x */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)).id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Bar)).id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)).id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)),
                    ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Bar)),
                    ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::ContinuousEffectKind {
                    kind: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                        continuous_effect::ContinuousEffectModifyObject {
                            object: reference.clone(),
                            modifications: {
                                use continuous_effect::modify_object::PowerToughnessModifiers::PlusPlus;
                                use continuous_effect::modify_object::*;

                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                    ObjectCharacteristicModification::PowerToughnessModifiers(PlusPlus(
                                        PowerToughnessModifiersPlusPlus {
                                            power_mod: power.clone(),
                                            toughness_mod: toughness.clone(),
                                        },
                                    )),
                                );

                                modifications.push(characteristic_mod);
                                modifications
                            },
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Object gets +x/+x and has an ability */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)).id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Bar)).id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)).id(),
                ParserNode::AbilityKind { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)),
                    ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Bar)),
                    ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)),
                    ParserNode::Number { number: toughness },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)),
                    ParserNode::AbilityKind { ability },
                ] => Ok(ParserNode::ContinuousEffectKind {
                    kind: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                        continuous_effect::ContinuousEffectModifyObject {
                            object: reference.clone(),
                            modifications: {
                                use continuous_effect::modify_object::PowerToughnessModifiers::PlusPlus;
                                use continuous_effect::modify_object::*;

                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                    ObjectCharacteristicModification::PowerToughnessModifiers(PlusPlus(
                                        PowerToughnessModifiersPlusPlus {
                                            power_mod: power.clone(),
                                            toughness_mod: toughness.clone(),
                                        },
                                    )),
                                );
                                let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: ability.clone(),
                                });

                                modifications.push(characteristic_mod);
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Object gets +x/+x and has an ability and another ability */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)).id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Bar)).id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)).id(),
                ParserNode::AbilityKind { ability: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)).id(),
                ParserNode::AbilityKind { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)),
                    ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Bar)),
                    ParserNode::LexerToken(TokenKind::PowerToughnessModElements(PowerToughnessModElements::Plus)),
                    ParserNode::Number { number: toughness },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)),
                    ParserNode::AbilityKind { ability: ability_1 },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)),
                    ParserNode::AbilityKind { ability: ability_2 },
                ] => Ok(ParserNode::ContinuousEffectKind {
                    kind: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                        continuous_effect::ContinuousEffectModifyObject {
                            object: reference.clone(),
                            modifications: {
                                use continuous_effect::modify_object::PowerToughnessModifiers::PlusPlus;
                                use continuous_effect::modify_object::*;

                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                    ObjectCharacteristicModification::PowerToughnessModifiers(PlusPlus(
                                        PowerToughnessModifiersPlusPlus {
                                            power_mod: power.clone(),
                                            toughness_mod: toughness.clone(),
                                        },
                                    )),
                                );
                                let gain_ab1_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: ability_1.clone(),
                                });
                                let gain_ab2_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: ability_2.clone(),
                                });

                                modifications.push(characteristic_mod);
                                modifications.push(gain_ab1_mod);
                                modifications.push(gain_ab2_mod);
                                modifications
                            },
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    /* Objects "have" abilities is a continuous effect kind. */
    /* "have" means its static, we shall parse it directly */
    let objects_have_abilities_only = vec![ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)).id(),
            ParserNode::AbilityKind { ability: dummy() }.id(),
        ]),
        merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)),
                ParserNode::AbilityKind { ability },
            ] => Ok(ParserNode::ContinuousEffect {
                effect: continuous_effect::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                        continuous_effect::ContinuousEffectModifyObject {
                            object: reference.clone(),
                            modifications: {
                                use continuous_effect::modify_object::*;

                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: ability.clone(),
                                });
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                        },
                    ),
                    duration: crate::ability_tree::time::ForwardDuration::ObjectLifetime,
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }];

    /* Objects can "gain" abilities, but it means it is for some duration. */
    let object_gains_abilities_until = [
        crate::ability_tree::time::ForwardDuration::UntilEndOfTurn,
        crate::ability_tree::time::ForwardDuration::UntilEndOfYourNextTurn,
    ]
    .into_iter()
    .map(|duration| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Gain)).id(),
            ParserNode::AbilityKind { ability: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ForwardDuration(duration)).id(),
        ]),
        merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Gain)),
                ParserNode::AbilityKind { ability },
                ParserNode::LexerToken(TokenKind::ForwardDuration(duration)),
            ] => Ok(ParserNode::ContinuousEffect {
                effect: continuous_effect::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                        continuous_effect::ContinuousEffectModifyObject {
                            object: reference.clone(),
                            modifications: {
                                use continuous_effect::modify_object::*;

                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: ability.clone(),
                                });
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                        },
                    ),
                    duration: duration.clone(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    [
        power_toughness_modifier_rules,
        objects_have_abilities_only,
        object_gains_abilities_until,
    ]
    .into_iter()
    .flatten()
}
