mod ability_tree;
mod continuous_effects;
mod imperative;
mod keyword_to_ability;
mod object_kinds;
mod object_references;
mod object_specifiers;
mod statement;
mod trigger_condition;

mod rule_loc;
mod state_id;

pub use rule_loc::ParserRuleDeclarationLocation;
pub use state_id::StateId;

use crate::parser::node::ParserNode;

pub const ALL_RULES: &[&[ParserRule]] = &[
    ability_tree::ABILITY_TREE_RULES,
    continuous_effects::CONTINUOUS_EFFECTS_RULES,
    imperative::IMPERATIVE_RULES,
    keyword_to_ability::KEYWORD_TO_ABILITY_RULES,
    object_references::OBJECT_REFERENCES_RULES,
    object_specifiers::OBJECT_SPECIFIER_RULES,
    statement::STATEMENT_RULES,
    trigger_condition::TRIGGER_CONDITION_RULES,
];

/// A single rule of merging nodes together for the parser.
#[derive(Debug, Clone)]
pub struct ParserRule {
    pub state: StateId,
    pub result: usize,
    pub conversion_func: fn(&[ParserNode]) -> Option<ParserNode>,
    pub creation_loc: ParserRuleDeclarationLocation,
}

impl ParserRule {
    pub const fn create(
        state: StateId,
        result: usize,
        conversion_func: fn(&[ParserNode]) -> Option<ParserNode>,
        creation_loc: ParserRuleDeclarationLocation,
    ) -> Self {
        Self {
            state,
            result,
            conversion_func,
            creation_loc,
        }
    }
}

#[macro_export]
macro_rules! make_parser_rule {
    ( [ $($tokens:tt)+ ] => Some( ParserNode:: $node:ident ( { $($gen:tt)+ } ) ) ) => {
        crate::parser::rules::ParserRule::create(
            crate::state_id!( [ $($tokens)+ ] ),
            crate::parser::node::ParserNodeKind::$node.id(),
            |tokens| match tokens {
                [ $($tokens)+ ] => Some( ParserNode:: $node ( { $($gen)+ } )),
                _ => None,
            },
            crate::parser::rules::ParserRuleDeclarationLocation {
                file: std::file!(),
                line: std::line!(),
            },
        )
    };
}
