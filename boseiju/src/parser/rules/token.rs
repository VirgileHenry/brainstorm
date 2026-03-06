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
    /* Tokens creatures with one creature subtype */
    let creature_subtypes_to_subtype_list = crate::ability_tree::object::CreatureSubtype::all()
        .into_iter()
        .map(|creature_subtype| {
            [
                /* A creature subtype on its own can make a subtype list */
                ParserRule {
                    expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::ObjectKind(
                        crate::ability_tree::object::ObjectKind::CreatureSubtype(creature_subtype),
                    ))
                    .id()]),
                    merged: ParserNode::CreatureSubtypeList {
                        list: dummy(),
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }
                    .id(),
                    reduction: |nodes: &[ParserNode]| match &nodes {
                        &[
                            ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CreatureSubtype(
                                creature_subtype,
                            ))),
                        ] => Ok(ParserNode::CreatureSubtypeList {
                            list: {
                                let mut list = arrayvec::ArrayVec::new();
                                list.push(creature_subtype.clone());
                                list
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: creature_subtype.span,
                        }),
                        _ => Err("Provided tokens do not match rule definition"),
                    },
                    creation_loc: ParserRuleDeclarationLocation::here(),
                },
                /* We can add creatures to that list (we can't express al the rules because of rule explosion here) */
                ParserRule {
                    expanded: RuleLhs::new(&[
                        ParserNode::CreatureSubtypeList {
                            list: dummy(),
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }
                        .id(),
                        ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CreatureSubtype(
                            creature_subtype,
                        )))
                        .id(),
                    ]),
                    merged: ParserNode::CreatureSubtypeList {
                        list: dummy(),
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }
                    .id(),
                    reduction: |nodes: &[ParserNode]| match &nodes {
                        &[
                            ParserNode::CreatureSubtypeList {
                                list,
                                #[cfg(feature = "spanned_tree")]
                                span,
                            },
                            ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CreatureSubtype(
                                creature_subtype,
                            ))),
                        ] => Ok(ParserNode::CreatureSubtypeList {
                            list: {
                                let mut list = list.clone();
                                list.push(creature_subtype.clone());
                                list
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: span.merge(&creature_subtype.span),
                        }),
                        _ => Err("Provided tokens do not match rule definition"),
                    },
                    creation_loc: ParserRuleDeclarationLocation::here(),
                },
            ]
        })
        .flatten()
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
        /* Token layout with the form: <p>/<t> <colors> <creature subtypes> creature token with <ability> */
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
                ParserNode::CreatureSubtypeList {
                    list: dummy(),
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }
                .id(),
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
                ParserNode::KeywordAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::TokenDefinition { token: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PowerToughness { pt }),
                    ParserNode::Colors { colors },
                    ParserNode::CreatureSubtypeList {
                        list: creature_subtypes_list,
                        #[cfg(feature = "spanned_tree")]
                            span: creature_subtypes_span,
                    },
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
                    ParserNode::KeywordAbility { ability },
                ] => Ok(ParserNode::TokenDefinition {
                    token: crate::ability_tree::card_layout::TokenLayout {
                        name: format!(
                            "{} token",
                            creature_subtypes_list
                                .iter()
                                .map(|st| format!("{} ", st.creature_subtype))
                                .collect::<String>()
                        ),
                        card_type: crate::ability_tree::type_line::TypeLine::creature_token(
                            creature_subtypes_list,
                            #[cfg(feature = "spanned_tree")]
                            *token_span,
                            #[cfg(feature = "spanned_tree")]
                            *creature_subtypes_span,
                        ),
                        colors: colors.clone(),
                        abilities: crate::AbilityTree {
                            abilities: {
                                let mut abilities = crate::utils::HeapArrayVec::new();
                                abilities.push(crate::ability_tree::ability::AbilityKind::Keyword(ability.clone()));
                                abilities
                            },
                            span: ability.node_span(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: ability.node_span().merge(&pt.span),
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
