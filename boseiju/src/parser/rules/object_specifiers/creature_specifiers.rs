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
    /* <creature subtype> is a creature "subtype" specifier */
    let subtypes_to_specifiers = crate::ability_tree::terminals::CreatureSubtype::all()
        .map(|subtype| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CreatureSubtype(subtype.clone())).id()]),
            merged: ParserNode::CreatureSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::CreatureSubtype(subtype))] => Ok(ParserNode::CreatureSpecifier {
                    specifier: object::specified_object::CreatureSpecifier::Subtype(
                        object::specified_object::CreatureSubtypeSpecifier {
                            subtype: subtype.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: subtype.node_span(),
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(intermediates::CardProperty::Power {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::CreatureSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::CardProperty(intermediates::CardProperty::Power { .. })),
                    ParserNode::Number { number },
                ] => Ok(ParserNode::CreatureSpecifier {
                    specifier: object::specified_object::CreatureSpecifier::WithCharacteristic(
                        object::specified_object::CreatureCharacteristicSpecifier::Power(
                            object::specified_object::CreaturePowerSpecifier {
                                power: number.clone(),
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
            merged: ParserNode::CreatureSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::KeywordAbility { keyword_ability },
                ] => Ok(ParserNode::CreatureSpecifier {
                    specifier: object::specified_object::CreatureSpecifier::WithCharacteristic(
                        object::specified_object::CreatureCharacteristicSpecifier::KeywordAbility(
                            object::specified_object::CreatureKeywordAbilitySpecifier {
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

    let common_specifiers = vec![
        /* "<control specifier>" is a creature specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ControlSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::CreatureSpecifier { specifier: dummy() }.id(),
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
            expanded: RuleLhs::new(&[ParserNode::ColorSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::CreatureSpecifier { specifier: dummy() }.id(),
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
            expanded: RuleLhs::new(&[ParserNode::AnotherSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::CreatureSpecifier { specifier: dummy() }.id(),
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
            expanded: RuleLhs::new(&[ParserNode::CreatureSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::CreatureSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CreatureSpecifier { specifier }] => Ok(ParserNode::CreatureSpecifiers {
                    specifiers: object::specified_object::Specifiers::Single(specifier.clone()),
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
