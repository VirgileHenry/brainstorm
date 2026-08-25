use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_tree::ability_tree::object;
use boseiju_tree::ability_tree::quantifier;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<active quantifier> <specified creature>" is an active creature */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::QuantifierActive {
                    count: Default::default(),
                }
                .id(),
                ParserNode::SpecifiedCreature {
                    creature: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CreatureActive {
                creature: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::QuantifierActive { count },
                    ParserNode::SpecifiedCreature { creature },
                ] => Ok(ParserNode::CreatureActive {
                    creature: object::Creature::Reference(object::reference::CreatureReference {
                        quantifier: count.clone(),
                        creature: creature.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(&creature.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive quantifier> <specified creature>" is a passive creature */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::QuantifierPassive {
                    count: Default::default(),
                }
                .id(),
                ParserNode::SpecifiedCreature {
                    creature: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CreaturePassive {
                creature: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::QuantifierPassive { count },
                    ParserNode::SpecifiedCreature { creature },
                ] => Ok(ParserNode::CreaturePassive {
                    creature: object::Creature::Reference(object::reference::CreatureReference {
                        quantifier: count.clone(),
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
            merged: ParserNode::CreatureActive {
                creature: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedCreature { creature }] => Ok(ParserNode::CreatureActive {
                    creature: object::Creature::Reference(object::reference::CreatureReference {
                        quantifier: quantifier::ActiveQuantifier::All(quantifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().empty_at_start(),
                        }),
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
