use super::ParserNode;
use crate::ability_tree::ability::statik;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /*
    let power_toughness_modifier_rules = [
        crate::ability_tree::terminals::PowerToughnessModifier::PlusXPlusX,
        crate::ability_tree::terminals::PowerToughnessModifier::PlusXMinusX,
        crate::ability_tree::terminals::PowerToughnessModifier::MinusXPlusX,
    ]
    .into_iter()
    .map(|modifier| super::ParserRule {
        from: super::RuleLhs::new(&[
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)).id(),
            ParserNode::LexerToken(TokenKind::PowerToughnessModifier(modifier)).id(),
        ]),
        result: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)),
                ParserNode::LexerToken(TokenKind::PowerToughnessModifier(modifier)),
            ] => {
                use statik::characteristic_defining_ability::CharacteristicDefiningAbility;
                Some(ParserNode::ContinuousEffectKind {
                    kind: statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies(
                        statik::continuous_effect::continuous_effect_kind::ContinuousEffectObjectGainsAbilies {
                            object: reference.clone(),
                            abilities: Box::new(crate::ability_tree::AbilityTree {
                                abilities: {
                                    let mut abilities = arrayvec::ArrayVec::new_const();
                                    abilities.push(crate::ability_tree::ability::Ability::Static(
                                        statik::StaticAbility::CharasteristicDefiningAbility(
                                            CharacteristicDefiningAbility::PowerToughnessModifier(*modifier),
                                        ),
                                    ));
                                    abilities
                                },
                            }),
                        },
                    ),
                })
            }
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();
    */

    let default_continuous_effect_kind_rules = vec![
        /* Fixme: gain indicate we expect a duration, have indicates there will be none. We can parse straight to continuous effects ? */
        /* Object gains abilities is a continuous effect kind */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Gain)).id(),
                ParserNode::AbilityTree { tree: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Gain)),
                    ParserNode::AbilityTree { tree },
                ] => Some(ParserNode::ContinuousEffectKind {
                    kind: statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies(
                        statik::continuous_effect::continuous_effect_kind::ContinuousEffectObjectGainsAbilies {
                            object: reference.clone(),
                            abilities: tree.clone(),
                        },
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Objects have abilities is a continuous effect kind */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)).id(),
                ParserNode::AbilityTree { tree: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)),
                    ParserNode::AbilityTree { tree },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                ] => Some(ParserNode::ContinuousEffectKind {
                    kind: statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies(
                        statik::continuous_effect::continuous_effect_kind::ContinuousEffectObjectGainsAbilies {
                            object: reference.clone(),
                            abilities: tree.clone(),
                        },
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Objects get power / toughness modifiers */
        /*
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)).id(),
                ParserNode::LexerToken(TokenKind::PowerToughnessModifier(
                    crate::ability_tree::terminals::PowerToughnessModifier::Constant { power: 0, toughness: 0 },
                ))
                .id(),
            ]),
            result: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Get)),
                    ParserNode::LexerToken(TokenKind::PowerToughnessModifier(
                        crate::ability_tree::terminals::PowerToughnessModifier::Constant { power, toughness },
                    )),
                ] => {
                    let mut abilities = arrayvec::ArrayVec::new_const();
                    abilities.push(crate::ability_tree::ability::Ability::Static(
                        statik::StaticAbility::CharasteristicDefiningAbility(
                            statik::characteristic_defining_ability::CharacteristicDefiningAbility::PowerToughnessModifier(
                                crate::ability_tree::terminals::PowerToughnessModifier::Constant {
                                    power: *power,
                                    toughness: *toughness,
                                },
                            ),
                        ),
                    ));
                    Some(ParserNode::ContinuousEffectKind {
                        kind: statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                            object: reference.clone(),
                            abilities: Box::new(crate::ability_tree::AbilityTree { abilities }),
                        },
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        */
    ];

    [
        /* power_toughness_modifier_rules, */ default_continuous_effect_kind_rules,
    ]
    .into_iter()
    .flatten()
}
