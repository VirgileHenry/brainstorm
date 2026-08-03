use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<count> <specified creature>" is a creature */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier {
                    count: Default::default(),
                }
                .id(),
                ParserNode::SpecifiedCreature {
                    creature: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Creature {
                creature: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CountSpecifier { count },
                    ParserNode::SpecifiedCreature { creature },
                ] => Ok(ParserNode::Creature {
                    creature: object::Creature::Reference(object::reference::CreatureReference {
                        count: count.clone(),
                        creature: creature.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(&creature.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified creature>" is a creature with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedCreature {
                creature: Default::default(),
            }
            .id()]),
            merged: ParserNode::Creature {
                creature: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedCreature { creature }] => Ok(ParserNode::Creature {
                    creature: object::Creature::Reference(object::reference::CreatureReference {
                        count: object::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().empty_at_start(),
                        },
                        creature: creature.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
