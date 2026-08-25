use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<add mana passive>" is a passive deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedAddManaPassive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedPassiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedAddManaPassive { deed }] => Ok(ParserNode::DeedPassiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::AddMana(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<attack passive>" is a passive deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedAttackPassive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedPassiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedAttackPassive { deed }] => Ok(ParserNode::DeedPassiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Attack(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive cast deed>" is an passive deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedCastPassive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedPassiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedCastPassive { deed }] => Ok(ParserNode::DeedPassiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Cast(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<deal damages deed>" is a passive deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedDealDamages {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedPassiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedDealDamages { deed }] => Ok(ParserNode::DeedPassiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::DealDamages(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive destroy deed>" is a passive deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedDestroyPassive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedPassiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedDestroyPassive { deed }] => Ok(ParserNode::DeedPassiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Destroy(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive draw deed>" is a passive deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedDrawPassive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedPassiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedDrawPassive { deed }] => Ok(ParserNode::DeedPassiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Draw(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<etb deed>" is a passive deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedEtb {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedPassiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedEtb { deed }] => Ok(ParserNode::DeedPassiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Etb(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive put counters deed>" is an passive deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedPutCountersPassive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedPassiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedPutCountersPassive { deed }] => Ok(ParserNode::DeedPassiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::PutCounters(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive sacrifice deed>" is a passive deed */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedSacrificePassive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::DeedPassiveForm {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedSacrificePassive { deed }] => Ok(ParserNode::DeedPassiveForm {
                    deed: boseiju_tree::ability_tree::deed::Deed::Sacrifice(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
