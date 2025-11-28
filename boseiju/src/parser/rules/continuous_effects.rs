use super::ParserNode;

use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;

macro_rules! get_power_touhness_mod {
    ( $keyword:path ) => {
        crate::make_parser_rule!(
            [
                ParserNode::ObjectReference(object),
                ParserNode::LexerToken(TokenKind::ActionKeywords(non_terminals::ActionKeywords::Get)),
                ParserNode::LexerToken(TokenKind::PowerToughnessModifier($keyword))
            ] => Some(ParserNode::ContinuousEffectKind({
                crate::ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                    object: object.clone(),
                    abilities: Box::new(crate::ability_tree::AbilityTree {
                        abilities: {
                            let mut abilities = arrayvec::ArrayVec::new();
                            abilities.push(crate::ability_tree::ability::Ability::Static(
                                crate::ability_tree::ability::statik::StaticAbility::CharasteristicDefiningAbility(
                                    crate::ability_tree::charasteristic_defining_ability::CharasteristicDefiningAbility::PowerToughnessModifier($keyword)
                                )
                            ));
                            abilities
                        },
                    }),
                }}
            ))
        )
    };
}

macro_rules! duration_to_continuous_effect {
    ( $duration:path ) => {
        crate::make_parser_rule!(
            /* Continuous effect kind need a time specifier to be complete continuous effects */
            [
                ParserNode::ContinuousEffectKind(effect),
                ParserNode::LexerToken(TokenKind::ContinuousEffectDuration($duration)),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot))
            ] => Some(ParserNode::ContinuousEffect({
                crate::ability_tree::continuous_effect::ContinuousEffect {
                    duration: $duration,
                    effect: effect.clone(),
                }}
            ))
        )
    };
}

macro_rules! gain_ab_tree_to_continuous_effect {
    ( $duration:path ) => {
        crate::make_parser_rule!(
            [
                ParserNode::ObjectReference(object),
                ParserNode::LexerToken(TokenKind::ActionKeywords(non_terminals::ActionKeywords::Gain)),
                ParserNode::AbilityTree(tree),
                ParserNode::LexerToken(TokenKind::ContinuousEffectDuration($duration)),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot))
            ] => Some(ParserNode::ContinuousEffect({
                crate::ability_tree::continuous_effect::ContinuousEffect {
                    duration: $duration,
                    effect: crate::ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                        object: object.clone(),
                        abilities: tree.clone(),
                    },
                }
            }))
        )
    };
}

#[rustfmt::skip]
pub const CONTINUOUS_EFFECTS_RULES: &[super::ParserRule] = &[


    /* When it's power / toughness modifiers, creatures "get" it */
    /* Manual rule creation, since we want to be able to match all power / toighness values */
    crate::parser::rules::ParserRule::create(
        crate::state_id!( [
            ParserNode::ObjectReference(object),
            ParserNode::LexerToken(TokenKind::ActionKeywords(non_terminals::ActionKeywords::Get)),
            ParserNode::LexerToken(TokenKind::PowerToughnessModifier(crate::ability_tree::terminals::PowerToughnessModifier::Constant {
                power: 0,
                toughness: 0
            }))
        ] ),
        crate::parser::node::ParserNodeKind::ContinuousEffectKind.id(),
        |tokens| match tokens {
            [
                ParserNode::ObjectReference(object),
                ParserNode::LexerToken(TokenKind::ActionKeywords(non_terminals::ActionKeywords::Get)),
                ParserNode::LexerToken(TokenKind::PowerToughnessModifier(crate::ability_tree::terminals::PowerToughnessModifier::Constant {
                    power,
                    toughness
                }))
            ] => Some(ParserNode::ContinuousEffectKind({
                crate::ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                    object: object.clone(),
                    abilities: Box::new(crate::ability_tree::AbilityTree {
                        abilities: {
                            let mut abilities = arrayvec::ArrayVec::new();
                            abilities.push(crate::ability_tree::ability::Ability::Static(
                                crate::ability_tree::ability::statik::StaticAbility::CharasteristicDefiningAbility(
                                    crate::ability_tree::charasteristic_defining_ability::CharasteristicDefiningAbility::PowerToughnessModifier(
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
                }}
            )),
            _ => None,
        },
        crate::parser::rules::ParserRuleDeclarationLocation {
            file: std::file!(),
            line: std::line!(),
        },
    ),

    get_power_touhness_mod!(crate::ability_tree::terminals::PowerToughnessModifier::PlusXPlusX),
    get_power_touhness_mod!(crate::ability_tree::terminals::PowerToughnessModifier::PlusXMinusX),
    get_power_touhness_mod!(crate::ability_tree::terminals::PowerToughnessModifier::MinusXPlusX),

    duration_to_continuous_effect!(crate::ability_tree::terminals::ContinuousEffectDuration::ObjectStaticAbility),
    duration_to_continuous_effect!(crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfTurn),
    duration_to_continuous_effect!(crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfNextTurn),

    /* For entire abiiliy trees, objects can "gain" them, when it's temporary (until end of turn for example) */
    gain_ab_tree_to_continuous_effect!(crate::ability_tree::terminals::ContinuousEffectDuration::ObjectStaticAbility),
    gain_ab_tree_to_continuous_effect!(crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfTurn),
    gain_ab_tree_to_continuous_effect!(crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfNextTurn),

    /* Without duration, objects "have" them: "creature have haste" */
    crate::make_parser_rule!(
        [
            ParserNode::ObjectReference(object),
            ParserNode::LexerToken(TokenKind::EnglishKeywords(non_terminals::EnglishKeywords::Have)),
            ParserNode::AbilityTree(tree),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot))
        ] => Some(ParserNode::ContinuousEffect({
            crate::ability_tree::continuous_effect::ContinuousEffect {
                duration: crate::ability_tree::terminals::ContinuousEffectDuration::ObjectStaticAbility,
                effect: crate::ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
                    object: object.clone(),
                    abilities: tree.clone(),
                },
            }}
        ))
    )
];
