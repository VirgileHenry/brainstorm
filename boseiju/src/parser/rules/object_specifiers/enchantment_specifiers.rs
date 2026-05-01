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
    /* <enchantment subtype> is a enchantment "subtype" specifier */
    let subtypes_to_specifiers = crate::ability_tree::terminals::EnchantmentSubtype::all()
        .map(|subtype| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::EnchantmentSubtype(subtype.clone())).id()]),
            merged: ParserNode::EnchantmentSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::EnchantmentSubtype(subtype))] => Ok(ParserNode::EnchantmentSpecifier {
                    specifier: object::specified_object::EnchantmentSpecifier::Subtype(
                        object::specified_object::EnchantmentSubtypeSpecifier {
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

    let common_specifiers = vec![
        /* "<control specifier>" is a enchantment specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ControlSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::EnchantmentSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ControlSpecifier { specifier }] => Ok(ParserNode::EnchantmentSpecifier {
                    specifier: object::specified_object::EnchantmentSpecifier::Control(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<color specifier>" is a enchantment specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ColorSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::EnchantmentSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ColorSpecifier { specifier }] => Ok(ParserNode::EnchantmentSpecifier {
                    specifier: object::specified_object::EnchantmentSpecifier::Color(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<another specifier>" is a enchantment specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::AnotherSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::EnchantmentSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::AnotherSpecifier { specifier }] => Ok(ParserNode::EnchantmentSpecifier {
                    specifier: object::specified_object::EnchantmentSpecifier::Another(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let merging_specifiers = vec![
        /* "<enchantment specifier>" on its own can make a enchantment specifiers node */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::EnchantmentSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::EnchantmentSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::EnchantmentSpecifier { specifier }] => Ok(ParserNode::EnchantmentSpecifiers {
                    specifiers: object::specified_object::Specifiers::Single(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<enchantment specifier> <enchantment specifier>" -> and list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::EnchantmentSpecifier { specifier: dummy() }.id(),
                ParserNode::EnchantmentSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::EnchantmentSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EnchantmentSpecifier { specifier: s1 },
                    ParserNode::EnchantmentSpecifier { specifier: s2 },
                ] => Ok(ParserNode::EnchantmentSpecifiers {
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
        /* "<enchantment specifier> or <enchantment specifier>" -> or list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::EnchantmentSpecifier { specifier: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::EnchantmentSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::EnchantmentSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EnchantmentSpecifier { specifier: s1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::EnchantmentSpecifier { specifier: s2 },
                ] => Ok(ParserNode::EnchantmentSpecifiers {
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

    [subtypes_to_specifiers, common_specifiers, merging_specifiers]
        .into_iter()
        .flatten()
}
