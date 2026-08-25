use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<add mana active>" is an active deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedAddManaActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedActiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedAddManaActive { deed }] => Ok(ParserNode::DeedActiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::AddMana(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<attack active>" is an active deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedAttackActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedActiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedAttackActive { deed }] => Ok(ParserNode::DeedActiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Attack(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active cast deed>" is an active deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedCastActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedActiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedCastActive { deed }] => Ok(ParserNode::DeedActiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Cast(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<deal damages deed>" is an active deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedDealDamages {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedActiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedDealDamages { deed }] => Ok(ParserNode::DeedActiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::DealDamages(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active destroy deed>" is an active deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedDestroyActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedActiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedDestroyActive { deed }] => Ok(ParserNode::DeedActiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Destroy(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active draw deed>" is an active deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedDrawActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedActiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedDrawActive { deed }] => Ok(ParserNode::DeedActiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Draw(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active put counters deed>" is an active deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedPutCountersActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedActiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedPutCountersActive { deed }] => Ok(ParserNode::DeedActiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::PutCounters(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active sacrifice deed>" is an active deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedSacrificeActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedActiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedSacrificeActive { deed }] => Ok(ParserNode::DeedActiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Sacrifice(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
