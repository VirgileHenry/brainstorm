use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* "<enchantment kind>" makes a specified enchantment  */

    let specifiers_to_specified_enchantments = ParserRule {
        expanded: RuleLhs::new(&[ParserNode::EnchantmentKind {
            enchantment: Default::default(),
        }
        .id()]),
        merged: ParserNode::SpecifiedEnchantment {
            enchantment: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::EnchantmentKind { enchantment }] => Ok(ParserNode::SpecifiedEnchantment {
                enchantment: object::specified_object::SpecifiedEnchantment {
                    kind: enchantment.clone(),
                    specifiers: None,
                    #[cfg(feature = "spanned_tree")]
                    span: enchantment.span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* enchantment subtypes can be used in place of the "enchantment" marker, adding a specifier */
    let subtype_to_enchantment_specifiers = boseiju_lexer::terminal::EnchantmentSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::EnchantmentSubtype(subtype.clone())).id()]),
        merged: ParserNode::SpecifiedEnchantment {
            enchantment: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(Token::EnchantmentSubtype(subtype))] => Ok(ParserNode::SpecifiedEnchantment {
                enchantment: object::specified_object::SpecifiedEnchantment {
                    kind: object::kind::EnchantmentKind::Enchantment {
                        #[cfg(feature = "spanned_tree")]
                        span: subtype.span(),
                    },
                    specifiers: Some(object::specified_object::Specifiers::Single(
                        object::specified_object::EnchantmentSpecifier::Subtype(
                            object::specified_object::EnchantmentSubtypeSpecifier {
                                subtype: subtype.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: subtype.span(),
                            },
                        ),
                    )),
                    #[cfg(feature = "spanned_tree")]
                    span: subtype.span(),
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
