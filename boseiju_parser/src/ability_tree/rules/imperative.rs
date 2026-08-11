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
    let default_imperative_rules = vec![
        /* "<imperative kind>" -> imperative, where the executing player is "you" */
        // ParserRule {
        //     expanded: RuleLhs::new(&[ParserNode::ImperativeKind {
        //         imperative: Default::default(),
        //     }
        //     .id()]),
        //     merged: ParserNode::Imperative {
        //         imperative: Default::default(),
        //     }
        //     .id(),
        //     reduction: |nodes: &[ParserNode]| match &nodes {
        //         &[ParserNode::ImperativeKind { imperative }] => Ok(ParserNode::Imperative {
        //             imperative: boseiju_tree::ability_tree::imperative::Imperative {
        //                 kind: imperative.clone(),
        //                 executing_player: boseiju_tree::ability_tree::player::PlayerReference::You(
        //                     boseiju_tree::ability_tree::player::You {
        //                         #[cfg(feature = "spanned_tree")]
        //                         span: imperative.span().empty_at_start(),
        //                     },
        //                 ),
        //                 #[cfg(feature = "spanned_tree")]
        //                 span: imperative.span(),
        //             },
        //         }),
        //         _ => Err("Provided tokens do not match rule definition"),
        //     },
        //     creation_loc: ParserRuleDeclarationLocation::here(),
        // },
        /* "<player> <imperative kind>" -> imperative */
        // ParserRule {
        //     expanded: RuleLhs::new(&[
        //         ParserNode::Player {
        //             player: Default::default(),
        //         }
        //         .id(),
        //         ParserNode::ImperativeKind {
        //             imperative: Default::default(),
        //         }
        //         .id(),
        //     ]),
        //     merged: ParserNode::Imperative {
        //         imperative: Default::default(),
        //     }
        //     .id(),
        //     reduction: |nodes: &[ParserNode]| match &nodes {
        //         &[
        //             /* Little comment to stop these two being on a single line */
        //             ParserNode::Player { player },
        //             ParserNode::ImperativeKind { imperative },
        //         ] => Ok(ParserNode::Imperative {
        //             imperative: boseiju_tree::ability_tree::imperative::Imperative {
        //                 kind: imperative.clone(),
        //                 executing_player: player.clone(),
        //                 #[cfg(feature = "spanned_tree")]
        //                 span: imperative.span(),
        //             },
        //         }),
        //         _ => Err("Provided tokens do not match rule definition"),
        //     },
        //     creation_loc: ParserRuleDeclarationLocation::here(),
        // },
    ];

    [
        default_imperative_rules,
        // add_mana_rules::rules().collect::<Vec<_>>(),
        // choose_rules::rules().collect::<Vec<_>>(),
        // deals_damage_rules::rules().collect::<Vec<_>>(),
        // draw_rules::rules().collect::<Vec<_>>(),
        // exile_rules::rules().collect::<Vec<_>>(),
        // for_each_rules::rules().collect::<Vec<_>>(),
        // gain_life_rules::rules().collect::<Vec<_>>(),
        // generate_continuous_effect_rules::rules().collect::<Vec<_>>(),
        // generate_delayed_triggered_ab_rules::rules().collect::<Vec<_>>(),
        // lose_life_rules::rules().collect::<Vec<_>>(),
        // put_counters_rules::rules().collect::<Vec<_>>(),
        // remove_counters_rules::rules().collect::<Vec<_>>(),
        // change_zone_rules::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
