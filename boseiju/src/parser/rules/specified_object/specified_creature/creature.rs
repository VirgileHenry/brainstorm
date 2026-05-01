use crate::ability_tree::object;
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
    /* "<creature kind>" makes a specified creature  */

    let specifiers_to_specified_creatures = ParserRule {
        expanded: RuleLhs::new(&[ParserNode::CreatureKind { creature: dummy() }.id()]),
        merged: ParserNode::SpecifiedCreature { creature: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::CreatureKind { creature }] => Ok(ParserNode::SpecifiedCreature {
                creature: object::specified_object::SpecifiedCreature {
                    kind: creature.clone(),
                    specifiers: None,
                    #[cfg(feature = "spanned_tree")]
                    span: creature.node_span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    };

    /* creature subtypes can be used in place of the "creature" marker, adding a specifier */
    let subtype_to_creature_specifiers = crate::ability_tree::terminals::CreatureSubtype::all().map(|subtype| ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CreatureSubtype(subtype.clone())).id()]),
        merged: ParserNode::SpecifiedCreature { creature: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(Token::CreatureSubtype(subtype))] => Ok(ParserNode::SpecifiedCreature {
                creature: object::specified_object::SpecifiedCreature {
                    kind: object::kind::CreatureKind::Creature {
                        #[cfg(feature = "spanned_tree")]
                        span: subtype.node_span(),
                    },
                    specifiers: Some(object::specified_object::Specifiers::Single(
                        object::specified_object::CreatureSpecifier::Subtype(
                            object::specified_object::CreatureSubtypeSpecifier {
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
        vec![specifiers_to_specified_creatures],
        subtype_to_creature_specifiers.collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
