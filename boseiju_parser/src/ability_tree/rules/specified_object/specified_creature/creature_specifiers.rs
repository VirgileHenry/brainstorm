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
    /* "<count> <creature kind> <creature specifiers>" makes a specified creature  */

    let specifiers_to_specified_creatures = ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::CreatureKind {
                creature: Default::default(),
            }
            .id(),
            ParserNode::CreatureSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::SpecifiedCreature {
            creature: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::CreatureKind { creature },
                ParserNode::CreatureSpecifiers { specifiers },
            ] => Ok(ParserNode::SpecifiedCreature {
                creature: object::specified_object::SpecifiedCreature {
                    kind: creature.clone(),
                    specifiers: Some(specifiers.clone()),
                    #[cfg(feature = "spanned_tree")]
                    span: specifiers.span().merge(&creature.span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* creature subtypes can be used in place of the "creature" marker, adding a specifier */
    let subtype_to_creature_specifiers = boseiju_lexer::terminal::CreatureSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::CreatureSubtype(subtype.clone())).id(),
            ParserNode::CreatureSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::SpecifiedCreature {
            creature: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::CreatureSubtype(subtype)),
                ParserNode::CreatureSpecifiers { specifiers },
            ] => Ok(ParserNode::SpecifiedCreature {
                creature: object::specified_object::SpecifiedCreature {
                    kind: object::kind::CreatureKind::Creature {
                        #[cfg(feature = "spanned_tree")]
                        span: subtype.span(),
                    },
                    specifiers: Some(
                        specifiers.add_factor_specifier(object::specified_object::CreatureSpecifier::Subtype(
                            object::specified_object::CreatureSubtypeSpecifier {
                                subtype: subtype.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: subtype.span(),
                            },
                        )),
                    ),
                    #[cfg(feature = "spanned_tree")]
                    span: specifiers.span().merge(&subtype.span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    });

    [
        vec![specifiers_to_specified_creatures],
        subtype_to_creature_specifiers.collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
