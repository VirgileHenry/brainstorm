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
    /* "<count> <land / land subtype> <land specifiers>" makes a specified land  */
    /* Examples: "a land you control" */

    let specifiers_to_specified_lands = ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::CardType(terminals::CardType {
                card_type: mtg_data::CardType::Land,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LandSpecifiers { specifiers: dummy() }.id(),
        ]),
        merged: ParserNode::SpecifiedLand { land: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    card_type: mtg_data::CardType::Land,
                    ..
                })),
                ParserNode::LandSpecifiers { specifiers },
            ] => Ok(ParserNode::SpecifiedLand {
                land: object::specified_object::SpecifiedLand {
                    specifiers: Some(specifiers.clone()),
                    #[cfg(feature = "spanned_tree")]
                    span: specifiers.node_span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* land subtypes can be used in place of the "land" marker, adding a specifier */
    let subtype_to_land_specifiers = crate::ability_tree::terminals::LandSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::LandSubtype(subtype.clone())).id(),
            ParserNode::LandSpecifiers { specifiers: dummy() }.id(),
        ]),
        merged: ParserNode::SpecifiedLand { land: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::LandSubtype(subtype)),
                ParserNode::LandSpecifiers { specifiers },
            ] => Ok(ParserNode::SpecifiedLand {
                land: object::specified_object::SpecifiedLand {
                    specifiers: Some(
                        specifiers.add_factor_specifier(object::specified_object::LandSpecifier::Subtype(
                            object::specified_object::LandSubtypeSpecifier {
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
        vec![specifiers_to_specified_lands],
        subtype_to_land_specifiers.collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
