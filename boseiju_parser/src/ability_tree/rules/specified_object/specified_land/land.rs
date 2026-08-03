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
    /* "<land kind>" makes a specified land  */

    let specifiers_to_specified_lands = ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LandKind {
            land: Default::default(),
        }
        .id()]),
        merged: ParserNode::SpecifiedLand {
            land: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LandKind { land }] => Ok(ParserNode::SpecifiedLand {
                land: object::specified_object::SpecifiedLand {
                    kind: land.clone(),
                    specifiers: None,
                    #[cfg(feature = "spanned_tree")]
                    span: land.span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* land subtypes can be used in place of the "land" marker, adding a specifier */
    let subtype_to_land_specifiers = boseiju_lexer::terminal::LandSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::LandSubtype(subtype.clone())).id()]),
        merged: ParserNode::SpecifiedLand {
            land: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(Token::LandSubtype(subtype))] => Ok(ParserNode::SpecifiedLand {
                land: object::specified_object::SpecifiedLand {
                    kind: object::kind::LandKind::Land {
                        #[cfg(feature = "spanned_tree")]
                        span: subtype.span(),
                    },
                    specifiers: Some(object::specified_object::Specifiers::Single(
                        object::specified_object::LandSpecifier::Subtype(object::specified_object::LandSubtypeSpecifier {
                            subtype: subtype.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: subtype.span(),
                        }),
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
        vec![specifiers_to_specified_lands],
        subtype_to_land_specifiers.collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
