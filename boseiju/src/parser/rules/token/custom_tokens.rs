use crate::ability_tree::terminals;
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
    /* creature subtypes can be converted to the abstract "creature subtype" to avoid rule explosion */
    let creature_subtypes_to_subtype_list = crate::ability_tree::object::CreatureSubtype::all()
        .into_iter()
        .map(|creature_subtype| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::ObjectKind(
                crate::ability_tree::object::ObjectKind::CreatureSubtype(creature_subtype),
            ))
            .id()]),
            merged: ParserNode::CreatureSubtype { subtype: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CreatureSubtype(
                        creature_subtype,
                    ))),
                ] => Ok(ParserNode::CreatureSubtype {
                    subtype: creature_subtype.clone(),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let single_color_to_colors = crate::ability_tree::terminals::Color::all()
        .map(|color| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Color(color)).id()]),
            merged: ParserNode::Colors { colors: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::Color(color))] => Ok(ParserNode::Colors {
                    colors: crate::ability_tree::colors::Colors::from_iter([color.color.clone()].into_iter()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();
    let dual_color_to_colors = crate::ability_tree::terminals::Color::all()
        .map(move |color_1| crate::ability_tree::terminals::Color::all().map(move |color_2| (color_1.clone(), color_2)))
        .flatten()
        .filter(|(c1, c2)| c1 != c2)
        .map(|(c1, c2)| ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::Color(c1)).id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Color(c2)).id(),
            ]),
            merged: ParserNode::Colors { colors: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Color(c1)),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::LexerToken(Token::Color(c2)),
                ] => Ok(ParserNode::Colors {
                    colors: crate::ability_tree::colors::Colors::from_iter([c1.color, c2.color].into_iter()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let create_token_rules = vec![
        /* <P/T> <colors> <subtype> creature token  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PowerToughness {
                    pt: terminals::PowerToughness {
                        power: 0,
                        toughness: 0,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                })
                .id(),
                ParserNode::Colors { colors: dummy() }.id(),
                ParserNode::CreatureSubtype { subtype: dummy() }.id(),
                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                    crate::ability_tree::object::CardType {
                        card_type: mtg_data::CardType::Creature,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                )))
                .id(),
                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                    crate::ability_tree::object::Supertype {
                        supertype: mtg_data::Supertype::Token,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                )))
                .id(),
            ]),
            merged: ParserNode::TokenDefinition { token: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PowerToughness {
                        #[cfg(feature = "spanned_tree")]
                        pt,
                        ..
                    }),
                    ParserNode::Colors { colors },
                    ParserNode::CreatureSubtype { subtype },
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                        crate::ability_tree::object::CardType {
                            card_type: mtg_data::CardType::Creature,
                            ..
                        },
                    ))),
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                        crate::ability_tree::object::Supertype {
                            supertype: mtg_data::Supertype::Token,
                            #[cfg(feature = "spanned_tree")]
                                span: token_span,
                        },
                    ))),
                ] => Ok(ParserNode::TokenDefinition {
                    token: crate::ability_tree::card_layout::TokenLayout {
                        name: format!("{} token", subtype.creature_subtype),
                        card_type: crate::ability_tree::type_line::TypeLine::creature_token(
                            &[subtype.clone()],
                            #[cfg(feature = "spanned_tree")]
                            subtype.node_span().merge(token_span),
                        ),
                        colors: colors.clone(),
                        abilities: crate::AbilityTree::empty(),
                        #[cfg(feature = "spanned_tree")]
                        span: pt.span.merge(token_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* <P/T> <colors> <subtype1> <subtype2> creature token  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PowerToughness {
                    pt: terminals::PowerToughness {
                        power: 0,
                        toughness: 0,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                })
                .id(),
                ParserNode::Colors { colors: dummy() }.id(),
                ParserNode::CreatureSubtype { subtype: dummy() }.id(),
                ParserNode::CreatureSubtype { subtype: dummy() }.id(),
                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                    crate::ability_tree::object::CardType {
                        card_type: mtg_data::CardType::Creature,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                )))
                .id(),
                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                    crate::ability_tree::object::Supertype {
                        supertype: mtg_data::Supertype::Token,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                )))
                .id(),
            ]),
            merged: ParserNode::TokenDefinition { token: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PowerToughness {
                        #[cfg(feature = "spanned_tree")]
                        pt,
                        ..
                    }),
                    ParserNode::Colors { colors },
                    ParserNode::CreatureSubtype { subtype: subtype1 },
                    ParserNode::CreatureSubtype { subtype: subtype2 },
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                        crate::ability_tree::object::CardType {
                            card_type: mtg_data::CardType::Creature,
                            ..
                        },
                    ))),
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                        crate::ability_tree::object::Supertype {
                            supertype: mtg_data::Supertype::Token,
                            #[cfg(feature = "spanned_tree")]
                                span: token_span,
                        },
                    ))),
                ] => Ok(ParserNode::TokenDefinition {
                    token: crate::ability_tree::card_layout::TokenLayout {
                        name: format!("{} token", subtype1.creature_subtype),
                        card_type: crate::ability_tree::type_line::TypeLine::creature_token(
                            &[subtype1.clone(), subtype2.clone()],
                            #[cfg(feature = "spanned_tree")]
                            subtype1.node_span().merge(token_span),
                        ),
                        colors: colors.clone(),
                        abilities: crate::AbilityTree::empty(),
                        #[cfg(feature = "spanned_tree")]
                        span: pt.span.merge(token_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* <P/T> <colors> <subtype> creature token with <ability>  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PowerToughness {
                    pt: terminals::PowerToughness {
                        power: 0,
                        toughness: 0,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                })
                .id(),
                ParserNode::Colors { colors: dummy() }.id(),
                ParserNode::CreatureSubtype { subtype: dummy() }.id(),
                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                    crate::ability_tree::object::CardType {
                        card_type: mtg_data::CardType::Creature,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                )))
                .id(),
                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                    crate::ability_tree::object::Supertype {
                        supertype: mtg_data::Supertype::Token,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                )))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
            ]),
            merged: ParserNode::TokenDefinition { token: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PowerToughness {
                        #[cfg(feature = "spanned_tree")]
                        pt,
                        ..
                    }),
                    ParserNode::Colors { colors },
                    ParserNode::CreatureSubtype { subtype },
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                        crate::ability_tree::object::CardType {
                            card_type: mtg_data::CardType::Creature,
                            ..
                        },
                    ))),
                    ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                        crate::ability_tree::object::Supertype {
                            supertype: mtg_data::Supertype::Token,
                            #[cfg(feature = "spanned_tree")]
                                span: token_span,
                        },
                    ))),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With { .. })),
                    ParserNode::KeywordAbility { keyword_ability },
                ] => Ok(ParserNode::TokenDefinition {
                    token: crate::ability_tree::card_layout::TokenLayout {
                        name: format!("{} token", subtype.creature_subtype),
                        card_type: crate::ability_tree::type_line::TypeLine::creature_token(
                            &[subtype.clone()],
                            #[cfg(feature = "spanned_tree")]
                            subtype.node_span().merge(token_span),
                        ),
                        colors: colors.clone(),
                        abilities: crate::AbilityTree {
                            abilities: {
                                let mut abilities = crate::utils::HeapArrayVec::new();
                                abilities.push(crate::ability_tree::ability::Ability::KeywordAbility(
                                    keyword_ability.clone(),
                                ));
                                abilities
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: keyword_ability.node_span(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: keyword_ability.node_span().merge(&pt.span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        creature_subtypes_to_subtype_list,
        single_color_to_colors,
        dual_color_to_colors,
        create_token_rules,
    ]
    .into_iter()
    .flatten()
}
