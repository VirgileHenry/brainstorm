use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<active add mana deed>" is a player action */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedAddManaActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::PlayerAction {
                action: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedAddManaActive { deed }] => Ok(ParserNode::PlayerAction {
                    action: boseiju_tree::ability_tree::imperative::PlayerAction::AddMana(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active deal damage deed>" is a player action */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedDealDamages {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::PlayerAction {
                action: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedDealDamages { deed }] => Ok(ParserNode::PlayerAction {
                    action: boseiju_tree::ability_tree::imperative::PlayerAction::DealDamages(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active destroy deed>" is a player action */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedDestroyActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::PlayerAction {
                action: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedDestroyActive { deed }] => Ok(ParserNode::PlayerAction {
                    action: boseiju_tree::ability_tree::imperative::PlayerAction::Destroy(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active draw deed>" is a player action */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedDrawActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::PlayerAction {
                action: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedDrawActive { deed }] => Ok(ParserNode::PlayerAction {
                    action: boseiju_tree::ability_tree::imperative::PlayerAction::Draw(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active put counters deed>" is a player action */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::DeedPutCountersActive {
                deed: Default::default(),
            }
            .id()]),
            merged: ParserNode::PlayerAction {
                action: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::DeedPutCountersActive { deed }] => Ok(ParserNode::PlayerAction {
                    action: boseiju_tree::ability_tree::imperative::PlayerAction::PutCounters(deed.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
