use crate::ability_tree::object;
use crate::ability_tree::terminals;
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

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "another creature" another is special, it's a + other */
    /* Examples: "another creature" */

    let specifiers_to_specified_creatures = ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::CardType(terminals::CardType {
                card_type: mtg_data::CardType::Creature,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::SpecifiedCreature { creature: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                    #[cfg(feature = "spanned_tree")]
                        span: another_span,
                })),
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    card_type: mtg_data::CardType::Creature,
                    #[cfg(feature = "spanned_tree")]
                        span: end_span,
                })),
            ] => Ok(ParserNode::SpecifiedCreature {
                creature: object::specified_object::SpecifiedCreature {
                    amount: object::CountSpecifier::A {
                        #[cfg(feature = "spanned_tree")]
                        span: *another_span,
                    },
                    specifiers: Some(object::specified_object::Specifiers::Single(
                        object::specified_object::CreatureSpecifier::Another(object::specified_object::AnotherObjectSpecifier {
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        }),
                    )),
                    #[cfg(feature = "spanned_tree")]
                    span: another_span.merge(end_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* Creature subtypes can be used in place of the "creature" marker, adding a specifier */
    let subtype_to_creature_specifiers = crate::ability_tree::terminals::CreatureSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::CreatureSubtype(subtype.clone())).id(),
        ]),
        merged: ParserNode::SpecifiedCreature { creature: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                    #[cfg(feature = "spanned_tree")]
                        span: another_span,
                })),
                ParserNode::LexerToken(Token::CreatureSubtype(subtype)),
            ] => Ok(ParserNode::SpecifiedCreature {
                creature: object::specified_object::SpecifiedCreature {
                    amount: object::CountSpecifier::A {
                        #[cfg(feature = "spanned_tree")]
                        span: *another_span,
                    },
                    specifiers: Some(
                        object::specified_object::Specifiers::Single(object::specified_object::CreatureSpecifier::Another(
                            object::specified_object::AnotherObjectSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *another_span,
                            },
                        ))
                        .add_factor_specifier(object::specified_object::CreatureSpecifier::Subtype(
                            object::specified_object::CreatureSubtypeSpecifier {
                                subtype: subtype.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: subtype.node_span(),
                            },
                        )),
                    ),
                    #[cfg(feature = "spanned_tree")]
                    span: another_span.merge(&subtype.node_span()),
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
