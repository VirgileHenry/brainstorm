mod add_mana_rules;
mod change_zone_rules;
mod choose_rules;
mod create_token_rules;
mod deals_damage_rules;
mod destroy_rules;
mod discard_rules;
mod draw_rules;
mod exile_rules;
mod for_each_rules;
mod gain_life_rules;
mod generate_continuous_effect_rules;
mod generate_delayed_triggered_ab_rules;
mod put_counters_rules;
mod remove_counters_rules;
mod sacrifice_rules;
mod tap_rules;

use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let default_imperative_rules = vec![
        /* "<imperative kind>" -> imperative, where the executing player is "you" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ImperativeKind { imperative: dummy() }.id()]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ImperativeKind { imperative }] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative {
                        kind: imperative.clone(),
                        executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                            #[cfg(feature = "spanned_tree")]
                            span: imperative.node_span().empty_at_start(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: imperative.node_span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player> <imperative kind>" -> imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::ImperativeKind { imperative: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    /* Little comment to stop these two being on a single line */
                    ParserNode::Player { player },
                    ParserNode::ImperativeKind { imperative },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative {
                        kind: imperative.clone(),
                        executing_player: player.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: imperative.node_span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        default_imperative_rules,
        add_mana_rules::rules().collect::<Vec<_>>(),
        choose_rules::rules().collect::<Vec<_>>(),
        create_token_rules::rules().collect::<Vec<_>>(),
        deals_damage_rules::rules().collect::<Vec<_>>(),
        destroy_rules::rules().collect::<Vec<_>>(),
        discard_rules::rules().collect::<Vec<_>>(),
        draw_rules::rules().collect::<Vec<_>>(),
        exile_rules::rules().collect::<Vec<_>>(),
        for_each_rules::rules().collect::<Vec<_>>(),
        gain_life_rules::rules().collect::<Vec<_>>(),
        generate_continuous_effect_rules::rules().collect::<Vec<_>>(),
        generate_delayed_triggered_ab_rules::rules().collect::<Vec<_>>(),
        put_counters_rules::rules().collect::<Vec<_>>(),
        remove_counters_rules::rules().collect::<Vec<_>>(),
        change_zone_rules::rules().collect::<Vec<_>>(),
        sacrifice_rules::rules().collect::<Vec<_>>(),
        tap_rules::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
