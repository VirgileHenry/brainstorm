use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = ParserRule> {
    let common_specifiers = vec![/* "<color specifier>" is a card specifier */ ParserRule {
        expanded: RuleLhs::new(&[ParserNode::ColorSpecifier { specifier: dummy() }.id()]),
        merged: ParserNode::CardSpecifier { specifier: dummy() }.id(),
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(intermediates::CardProperty::ManaValue {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::CardSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::CardProperty(intermediates::CardProperty::ManaValue { .. })),
                    ParserNode::Number { number },
                ] => Ok(ParserNode::CardSpecifier {
                    specifier: object::specified_object::CardSpecifier::WithCharacteristic(
                        object::specified_object::CardCharacteristicSpecifier::ManaValue(
                            object::specified_object::CardManaValueSpecifier {
                                mana_value: number.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: number.node_span().merge(start_span),
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
            merged: ParserNode::CardSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
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
                                span: keyword_ability.node_span().merge(start_span),
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
            expanded: RuleLhs::new(&[ParserNode::CardSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::CardSpecifiers { specifiers: dummy() }.id(),
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
                ParserNode::CardSpecifier { specifier: dummy() }.id(),
                ParserNode::CardSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::CardSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CardSpecifier { specifier: s1 },
                    ParserNode::CardSpecifier { specifier: s2 },
                ] => Ok(ParserNode::CardSpecifiers {
                    specifiers: object::specified_object::Specifiers::And(object::specified_object::SpecifierAndList {
                        specifiers: [s1.clone(), s2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: s1.node_span().merge(&s2.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<card specifier> or <card specifier>" -> or list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CardSpecifier { specifier: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CardSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::CardSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CardSpecifier { specifier: s1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::CardSpecifier { specifier: s2 },
                ] => Ok(ParserNode::CardSpecifiers {
                    specifiers: object::specified_object::Specifiers::Or(object::specified_object::SpecifierOrList {
                        specifiers: [s1.clone(), s2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: s1.node_span().merge(&s2.node_span()),
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
