use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::parser::node::DummyInit;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
    let artifacts_subtypes_rules = mtg_data::ArtifactType::all()
        .map(|subtype| super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::ArtifactSubtype(subtype),
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::ArtifactSubtype(
                        subtype,
                    ))),
                ] => Some(ParserNode::ObjectSpecifier {
                    specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                        crate::ability_tree::object::ObjectKind::ArtifactSubtype(*subtype),
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let battle_subtypes_rules = mtg_data::BattleType::all()
        .map(|subtype| super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::BattleSubtype(subtype),
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::BattleSubtype(
                        subtype,
                    ))),
                ] => Some(ParserNode::ObjectSpecifier {
                    specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                        crate::ability_tree::object::ObjectKind::BattleSubtype(*subtype),
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let creature_subtypes_rules = mtg_data::CreatureType::all()
        .map(|subtype| super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::CreatureSubtype(subtype),
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::CreatureSubtype(
                        subtype,
                    ))),
                ] => Some(ParserNode::ObjectSpecifier {
                    specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                        crate::ability_tree::object::ObjectKind::CreatureSubtype(*subtype),
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();
    let enchantments_subtypes_rules = mtg_data::EnchantmentType::all()
        .map(|subtype| super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::EnchantmentSubtype(subtype),
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::EnchantmentSubtype(
                        subtype,
                    ))),
                ] => Some(ParserNode::ObjectSpecifier {
                    specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                        crate::ability_tree::object::ObjectKind::EnchantmentSubtype(*subtype),
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let spell_subtypes_rules = mtg_data::SpellType::all()
        .map(|subtype| super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::InstantSorcerySubtype(subtype),
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::ObjectKind(
                        crate::ability_tree::object::ObjectKind::InstantSorcerySubtype(subtype),
                    )),
                ] => Some(ParserNode::ObjectSpecifier {
                    specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                        crate::ability_tree::object::ObjectKind::InstantSorcerySubtype(*subtype),
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let land_subtypes_rules = mtg_data::LandType::all()
        .map(|subtype| super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::LandSubtype(subtype),
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::LandSubtype(subtype))),
                ] => Some(ParserNode::ObjectSpecifier {
                    specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                        crate::ability_tree::object::ObjectKind::LandSubtype(*subtype),
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let planeswalker_subtypes_rules = mtg_data::PlaneswalkerType::all()
        .map(|subtype| super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::PlaneswalkerSubtype(subtype),
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::PlaneswalkerSubtype(
                        subtype,
                    ))),
                ] => Some(ParserNode::ObjectSpecifier {
                    specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                        crate::ability_tree::object::ObjectKind::PlaneswalkerSubtype(*subtype),
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let supertypes_rules = mtg_data::Supertype::all()
        .map(|subtype| super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::Supertype(subtype),
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(subtype)))] => {
                    Some(ParserNode::ObjectSpecifier {
                        specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                            crate::ability_tree::object::ObjectKind::Supertype(*subtype),
                        ),
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let card_types_rules = mtg_data::CardType::all()
        .map(|subtype| super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::CardType(subtype),
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(subtype)))] => {
                    Some(ParserNode::ObjectSpecifier {
                        specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                            crate::ability_tree::object::ObjectKind::CardType(*subtype),
                        ),
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let group_types_rules = vec![
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::Card,
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Card))] => {
                    Some(ParserNode::ObjectSpecifier {
                        specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                            crate::ability_tree::object::ObjectKind::Card,
                        ),
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::Spell,
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Spell))] => {
                    Some(ParserNode::ObjectSpecifier {
                        specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                            crate::ability_tree::object::ObjectKind::Spell,
                        ),
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ObjectKind(
                crate::ability_tree::object::ObjectKind::Permanent,
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Permanent))] => {
                    Some(ParserNode::ObjectSpecifier {
                        specifier: crate::ability_tree::object::ObjectSpecifier::Kind(
                            crate::ability_tree::object::ObjectKind::Permanent,
                        ),
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        artifacts_subtypes_rules,
        battle_subtypes_rules,
        card_types_rules,
        creature_subtypes_rules,
        enchantments_subtypes_rules,
        spell_subtypes_rules,
        land_subtypes_rules,
        planeswalker_subtypes_rules,
        supertypes_rules,
        group_types_rules,
    ]
    .into_iter()
    .flatten()
}
