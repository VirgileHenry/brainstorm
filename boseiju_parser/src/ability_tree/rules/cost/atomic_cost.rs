use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<pay mana deed>" makes up an atomic cost */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::PayMana {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::AtomicCost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PayMana { deed }] => Ok(ParserNode::AtomicCost {
                    cost: boseiju_tree::ability_tree::cost::AtomicCost::Mana(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<pay life deed>" makes up an atomic cost */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::PayLife {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::AtomicCost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PayLife { deed }] => Ok(ParserNode::AtomicCost {
                    cost: boseiju_tree::ability_tree::cost::AtomicCost::Life(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<sacrifice deed>" makes up an atomic cost */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedSacrificePassive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::AtomicCost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedSacrificePassive { deed }] => Ok(ParserNode::AtomicCost {
                    cost: boseiju_tree::ability_tree::cost::AtomicCost::Sacrifice(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<tap deed>" makes up an atomic cost */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::TapPassive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::AtomicCost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::TapPassive { deed }] => Ok(ParserNode::AtomicCost {
                    cost: boseiju_tree::ability_tree::cost::AtomicCost::Tap(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<untap deed>" makes up an atomic cost */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::UntapPassive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::AtomicCost {
                cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::UntapPassive { deed }] => Ok(ParserNode::AtomicCost {
                    cost: boseiju_tree::ability_tree::cost::AtomicCost::Untap(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
