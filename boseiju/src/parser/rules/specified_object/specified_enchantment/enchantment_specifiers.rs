use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "<count> <enchantment / enchantment subtype> <enchantment specifiers>" makes a specified enchantment  */
    /* Examples: "a enchantment you control" */

    let specifiers_to_specified_enchantments = ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::CardType(terminals::CardType {
                card_type: mtg_data::CardType::Enchantment,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::EnchantmentSpecifiers { specifiers: dummy() }.id(),
        ]),
        merged: ParserNode::SpecifiedEnchantment { enchantment: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    card_type: mtg_data::CardType::Enchantment,
                    ..
                })),
                ParserNode::EnchantmentSpecifiers { specifiers },
            ] => Ok(ParserNode::SpecifiedEnchantment {
                enchantment: object::specified_object::SpecifiedEnchantment {
                    specifiers: Some(specifiers.clone()),
                    #[cfg(feature = "spanned_tree")]
                    span: specifiers.node_span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* enchantment subtypes can be used in place of the "enchantment" marker, adding a specifier */
    let subtype_to_enchantment_specifiers = crate::ability_tree::terminals::EnchantmentSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::EnchantmentSubtype(subtype.clone())).id(),
            ParserNode::EnchantmentSpecifiers { specifiers: dummy() }.id(),
        ]),
        merged: ParserNode::SpecifiedEnchantment { enchantment: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::EnchantmentSubtype(subtype)),
                ParserNode::EnchantmentSpecifiers { specifiers },
            ] => Ok(ParserNode::SpecifiedEnchantment {
                enchantment: object::specified_object::SpecifiedEnchantment {
                    specifiers: Some(
                        specifiers.add_factor_specifier(object::specified_object::EnchantmentSpecifier::Subtype(
                            object::specified_object::EnchantmentSubtypeSpecifier {
                                subtype: subtype.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: subtype.node_span(),
                            },
                        )),
                    ),
                    #[cfg(feature = "spanned_tree")]
                    span: specifiers.node_span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    });

    [
        vec![specifiers_to_specified_enchantments],
        subtype_to_enchantment_specifiers.collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
