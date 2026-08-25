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
    let common_specifiers = vec![/* "<color specifier>" is a card specifier */ ParserRule {
        expanded: RuleLhs::new(&[ParserNode::ColorSpecifier {
            specifier: Default::default(),
        }
        .id()]),
        merged: ParserNode::CardSpecifier {
            specifier: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::ColorSpecifier { specifier }] => Ok(ParserNode::CardSpecifier {
                specifier: object::specified_object::CardSpecifier::Color(specifier.clone()),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }];

    let characteristic_specifiers = vec![
        /* "with mana value <number>" makes a mana value specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishPreposition(intermediate::EnglishPreposition::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(intermediate::CardProperty::ManaValue {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CardSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishPreposition(intermediate::EnglishPreposition::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::CardProperty(intermediate::CardProperty::ManaValue { .. })),
                    ParserNode::Number { number },
                ] => Ok(ParserNode::CardSpecifier {
                    specifier: object::specified_object::CardSpecifier::WithCharacteristic(
                        object::specified_object::CardCharacteristicSpecifier::ManaValue(
                            object::specified_object::CardManaValueSpecifier {
                                mana_value: number.clone(),
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
                ParserNode::LexerToken(Token::EnglishPreposition(intermediate::EnglishPreposition::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CardSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishPreposition(intermediate::EnglishPreposition::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::KeywordAbility { keyword_ability },
                ] => Ok(ParserNode::CardSpecifier {
                    specifier: object::specified_object::CardSpecifier::WithCharacteristic(
                        object::specified_object::CardCharacteristicSpecifier::KeywordAbility(
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

    let merging_specifiers = vec![
        /* "<card specifier>" on its own can make a card specifiers node */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::CardSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::CardSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CardSpecifier { specifier }] => Ok(ParserNode::CardSpecifiers {
                    specifiers: object::specified_object::Specifiers::Single(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<card specifier> <card specifier>" -> and list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CardSpecifier {
                    specifier: Default::default(),
                }
                .id(),
                ParserNode::CardSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CardSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CardSpecifier { specifier: s1 },
                    ParserNode::CardSpecifier { specifier: s2 },
                ] => Ok(ParserNode::CardSpecifiers {
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
        /* "<card specifier> or <card specifier>" -> or list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CardSpecifier {
                    specifier: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CardSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CardSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CardSpecifier { specifier: s1 },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or { .. })),
                    ParserNode::CardSpecifier { specifier: s2 },
                ] => Ok(ParserNode::CardSpecifiers {
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

    [common_specifiers, characteristic_specifiers, merging_specifiers]
        .into_iter()
        .flatten()
}
