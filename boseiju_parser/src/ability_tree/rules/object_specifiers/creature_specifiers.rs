use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = ParserRule> {
    /* <creature subtype> is a creature "subtype" specifier */
    let subtypes_to_specifiers = boseiju_lexer::terminal::CreatureSubtype::all()
        .map(|subtype| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CreatureSubtype(subtype.clone())).id()]),
            merged: ParserNode::CreatureSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::CreatureSubtype(subtype))] => Ok(ParserNode::CreatureSpecifier {
                    specifier: object::specified_object::CreatureSpecifier::Subtype(
                        object::specified_object::CreatureSubtypeSpecifier {
                            subtype: subtype.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: subtype.span(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let characteristic_specifiers = vec![
        /* "with power <number>" makes a power specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(intermediate::CardProperty::Power {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CreatureSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::CardProperty(intermediate::CardProperty::Power { .. })),
                    ParserNode::Number { number },
                ] => Ok(ParserNode::CreatureSpecifier {
                    specifier: object::specified_object::CreatureSpecifier::WithCharacteristic(
                        object::specified_object::CreatureCharacteristicSpecifier::Power(
                            object::specified_object::CreaturePowerSpecifier {
                                power: number.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: number.span().merge(start_span),
                            },
                        ),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "with <keyword ability>" makes a keyword ability specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CreatureSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::KeywordAbility { keyword_ability },
                ] => Ok(ParserNode::CreatureSpecifier {
                    specifier: object::specified_object::CreatureSpecifier::WithCharacteristic(
                        object::specified_object::CreatureCharacteristicSpecifier::KeywordAbility(
                            object::specified_object::KeywordAbilitySpecifier {
                                keyword_ability: Box::new(keyword_ability.clone()),
                                #[cfg(feature = "spanned_tree")]
                                span: keyword_ability.span().merge(start_span),
                            },
                        ),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let common_specifiers = vec![
        /* "<control specifier>" is a creature specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ControlSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::CreatureSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ControlSpecifier { specifier }] => Ok(ParserNode::CreatureSpecifier {
                    specifier: object::specified_object::CreatureSpecifier::Control(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<color specifier>" is a creature specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ColorSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::CreatureSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ColorSpecifier { specifier }] => Ok(ParserNode::CreatureSpecifier {
                    specifier: object::specified_object::CreatureSpecifier::Color(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<another specifier>" is a creature specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::AnotherSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::CreatureSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::AnotherSpecifier { specifier }] => Ok(ParserNode::CreatureSpecifier {
                    specifier: object::specified_object::CreatureSpecifier::Another(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let merging_specifiers = vec![
        /* "<creature specifier>" on its own can make a creature specifiers node */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::CreatureSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::CreatureSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CreatureSpecifier { specifier }] => Ok(ParserNode::CreatureSpecifiers {
                    specifiers: object::specified_object::Specifiers::Single(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature specifier> <creature specifier>" -> and list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureSpecifier {
                    specifier: Default::default(),
                }
                .id(),
                ParserNode::CreatureSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CreatureSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureSpecifier { specifier: s1 },
                    ParserNode::CreatureSpecifier { specifier: s2 },
                ] => Ok(ParserNode::CreatureSpecifiers {
                    specifiers: object::specified_object::Specifiers::And(object::specified_object::SpecifierAndList {
                        specifiers: [s1.clone(), s2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: s1.span().merge(&s2.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature specifier> or <creature specifier>" -> or list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureSpecifier {
                    specifier: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CreatureSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CreatureSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureSpecifier { specifier: s1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or { .. })),
                    ParserNode::CreatureSpecifier { specifier: s2 },
                ] => Ok(ParserNode::CreatureSpecifiers {
                    specifiers: object::specified_object::Specifiers::Or(object::specified_object::SpecifierOrList {
                        specifiers: [s1.clone(), s2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: s1.span().merge(&s2.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        subtypes_to_specifiers,
        characteristic_specifiers,
        common_specifiers,
        merging_specifiers,
    ]
    .into_iter()
    .flatten()
}
