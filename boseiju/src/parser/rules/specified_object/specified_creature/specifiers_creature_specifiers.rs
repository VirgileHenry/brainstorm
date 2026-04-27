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
    /* "<creature specifiers> <creature / creature subtype>" makes a specified creature with an implicit "all" */
    /* Examples: "other creatures", "green elves" */

    let specifiers_to_specified_creatures = ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::CreatureSpecifiers { specifiers: dummy() }.id(),
            ParserNode::LexerToken(Token::CardType(terminals::CardType {
                card_type: mtg_data::CardType::Creature,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::CreatureSpecifiers { specifiers: dummy() }.id(),
        ]),
        merged: ParserNode::SpecifiedCreature { creature: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::CreatureSpecifiers { specifiers: s1 },
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    card_type: mtg_data::CardType::Creature,
                    #[cfg(feature = "spanned_tree")]
                        span: creature_span,
                })),
                ParserNode::CreatureSpecifiers { specifiers: s2 },
            ] => Ok(ParserNode::SpecifiedCreature {
                creature: object::specified_object::SpecifiedCreature {
                    amount: object::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span: s1.node_span().empty_at_start(),
                    },
                    specifiers: Some(s1.merge_specifiers(s2.clone())),
                    #[cfg(feature = "spanned_tree")]
                    span: s2.node_span().merge(creature_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* Creature subtypes can be used in place of the "creature" marker, adding a specifier */
    let subtype_to_creature_specifiers = crate::ability_tree::terminals::CreatureSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::CreatureSpecifiers { specifiers: dummy() }.id(),
            ParserNode::LexerToken(Token::CreatureSubtype(subtype.clone())).id(),
            ParserNode::CreatureSpecifiers { specifiers: dummy() }.id(),
        ]),
        merged: ParserNode::SpecifiedCreature { creature: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::CreatureSpecifiers { specifiers: s1 },
                ParserNode::LexerToken(Token::CreatureSubtype(subtype)),
                ParserNode::CreatureSpecifiers { specifiers: s2 },
            ] => Ok(ParserNode::SpecifiedCreature {
                creature: object::specified_object::SpecifiedCreature {
                    amount: object::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span: s1.node_span().empty_at_start(),
                    },
                    specifiers: Some(s1.merge_specifiers(s2.clone()).add_factor_specifier(
                        object::specified_object::CreatureSpecifier::Subtype(
                            object::specified_object::CreatureSubtypeSpecifier {
                                subtype: subtype.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: subtype.node_span(),
                            },
                        ),
                    )),
                    #[cfg(feature = "spanned_tree")]
                    span: s2.node_span().merge(&subtype.node_span()),
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
