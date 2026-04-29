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
    /* "<enchantment / enchantment subtype>" makes a specified enchantment  */
    /* Examples: "target enchantment", "all aura" */

    let specifiers_to_specified_enchantments = ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CardType(terminals::CardType {
            card_type: mtg_data::CardType::Enchantment,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }))
        .id()]),
        merged: ParserNode::SpecifiedEnchantment { enchantment: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    card_type: mtg_data::CardType::Enchantment,
                    #[cfg(feature = "spanned_tree")]
                        span: enchantment_span,
                })),
            ] => Ok(ParserNode::SpecifiedEnchantment {
                enchantment: object::specified_object::SpecifiedEnchantment {
                    specifiers: None,
                    #[cfg(feature = "spanned_tree")]
                    span: *enchantment_span,
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* enchantment subtypes can be used in place of the "enchantment" marker, adding a specifier */
    let subtype_to_enchantment_specifiers = crate::ability_tree::terminals::EnchantmentSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::EnchantmentSubtype(subtype.clone())).id()]),
        merged: ParserNode::SpecifiedEnchantment { enchantment: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(Token::EnchantmentSubtype(subtype))] => Ok(ParserNode::SpecifiedEnchantment {
                enchantment: object::specified_object::SpecifiedEnchantment {
                    specifiers: Some(object::specified_object::Specifiers::Single(
                        object::specified_object::EnchantmentSpecifier::Subtype(
                            object::specified_object::EnchantmentSubtypeSpecifier {
                                subtype: subtype.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: subtype.node_span(),
                            },
                        ),
                    )),
                    #[cfg(feature = "spanned_tree")]
                    span: subtype.node_span(),
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
