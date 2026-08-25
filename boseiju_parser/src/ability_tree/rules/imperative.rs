// mod add_mana_rules;
// mod change_zone_rules;
// mod choose_rules;
// mod deals_damage_rules;
// mod draw_rules;
// mod exile_rules;
// mod for_each_rules;
// mod gain_life_rules;
// mod generate_continuous_effect_rules;
// mod generate_delayed_triggered_ab_rules;
// mod lose_life_rules;
// mod put_counters_rules;
// mod remove_counters_rules;

use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [/* "<player action>" -> imperative */ ParserRule {
        expanded: RuleLhs::new(&[ParserNode::PlayerAction {
            action: Default::default(),
        }
        .id()]),
        merged: ParserNode::Imperative {
            imperative: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::PlayerAction { action }] => Ok(ParserNode::Imperative {
                imperative: boseiju_tree::ability_tree::imperative::Imperative {
                    action: action.clone(),
                    #[cfg(feature = "spanned_tree")]
                    span: action.span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
