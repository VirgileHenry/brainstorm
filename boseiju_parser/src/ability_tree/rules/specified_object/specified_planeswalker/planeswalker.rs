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
    /* "<planeswalker kind>" makes a specified planeswalker  */

    let specifiers_to_specified_planeswalkers = ParserRule {
        expanded: RuleLhs::new(&[ParserNode::PlaneswalkerKind {
            planeswalker: Default::default(),
        }
        .id()]),
        merged: ParserNode::SpecifiedPlaneswalker {
            planeswalker: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::PlaneswalkerKind { planeswalker }] => Ok(ParserNode::SpecifiedPlaneswalker {
                planeswalker: object::specified_object::SpecifiedPlaneswalker {
                    kind: planeswalker.clone(),
                    specifiers: None,
                    #[cfg(feature = "spanned_tree")]
                    span: planeswalker.span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* planeswalker subtypes can be used in place of the "planeswalker" marker, adding a specifier */
    let subtype_to_planeswalker_specifiers = boseiju_lexer::terminal::PlaneswalkerSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::PlaneswalkerSubtype(subtype.clone())).id()]),
        merged: ParserNode::SpecifiedPlaneswalker {
            planeswalker: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(Token::PlaneswalkerSubtype(subtype))] => Ok(ParserNode::SpecifiedPlaneswalker {
                planeswalker: object::specified_object::SpecifiedPlaneswalker {
                    kind: object::kind::PlaneswalkerKind::Planeswalker {
                        #[cfg(feature = "spanned_tree")]
                        span: subtype.span(),
                    },
                    specifiers: Some(object::specified_object::Specifiers::Single(
                        object::specified_object::PlaneswalkerSpecifier::Subtype(
                            object::specified_object::PlaneswalkerSubtypeSpecifier {
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
        vec![specifiers_to_specified_planeswalkers],
        subtype_to_planeswalker_specifiers.collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
