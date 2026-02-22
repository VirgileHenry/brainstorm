//! Parser merging rules.
//!
//! The goal of this module is to define the set of rules that allow merging tokens
//! into tree nodes, and those tree nodes into more tree nodes until we have a full ability.
//!
//! To do this, we create a rule struct that represent a single rule for merging nodes,
//! as well as all the rule defintions that are constructed by the [`rules`] function in
//! the various modules.
//!
//! We define a bunch of submodules named after ability tree nodes, and each submodule is
//! responsible for creating the rules that can create this ability tree node.

mod ability;
mod ability_tree;
mod continuous_effects;
mod continuous_effects_kind;
mod cost_modifications;
mod event;
mod event_replacement;
mod if_condition;
mod imperative;
mod keyword_to_ability;
mod mana;
mod number;
mod object_references;
mod object_specifiers;
mod replacement_effect;
mod spell_ability;
mod statement;
mod triggered_ability;
mod zone;

use crate::parser::node::ParserNode;

pub fn default_rules() -> impl Iterator<Item = ParserRule> {
    /* Mmh, I think this will create vtables for Iterator<Item = ParserRule> ? */
    let rules_iters: Vec<Box<dyn Iterator<Item = ParserRule>>> = vec![
        Box::new(ability::rules()),
        Box::new(ability_tree::rules()),
        Box::new(continuous_effects::rules()),
        Box::new(continuous_effects_kind::rules()),
        Box::new(cost_modifications::rules()),
        Box::new(event::rules()),
        Box::new(event_replacement::rules()),
        Box::new(if_condition::rules()),
        Box::new(imperative::rules()),
        Box::new(keyword_to_ability::rules()),
        Box::new(mana::rules()),
        Box::new(number::rules()),
        Box::new(object_references::rules()),
        Box::new(object_specifiers::rules()),
        Box::new(replacement_effect::rules()),
        Box::new(spell_ability::rules()),
        Box::new(statement::rules()),
        Box::new(triggered_ability::rules()),
        Box::new(zone::rules()),
    ];
    rules_iters.into_iter().flatten()
}

/// A single rule of merging nodes together for the parser.
///
/// The parser rule structure has multiple goals.
///
/// We want to be able to know which tokens are accepted next to each other
/// in the rules, and which tokens can be generated from which to build
/// an acceleration structure for parsing.
///
/// Furthermore, we need a reduction function that is able to build the
/// target token from the rules token.
///
/// Finally, we keep the in code location of where the rule was declared for easy debugging
/// if two rules are matching over the same set of tokens.
#[derive(Debug, Clone)]
pub struct ParserRule {
    pub from: RuleLhs,
    pub result: usize,
    pub reduction: fn(&[ParserNode]) -> Option<ParserNode>,
    pub creation_loc: ParserRuleDeclarationLocation,
}

/// The rule left hand side.
///
/// This is basically an non empty array of token ids that are matched by that rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleLhs {
    pub tokens: [usize; Self::MAX_TOKENS],
    pub length: std::num::NonZeroUsize,
}

impl RuleLhs {
    const MAX_TOKENS: usize = 8;

    pub fn new(tokens: &[usize]) -> Self {
        if tokens.len() > Self::MAX_TOKENS {
            panic!("Can only create rules with up to {} tokens", Self::MAX_TOKENS);
        }
        let length = match std::num::NonZeroUsize::new(tokens.len()) {
            Some(length) => length,
            None => panic!("Can only create rules with at least 1 token"),
        };

        Self {
            tokens: std::array::from_fn(|i| tokens.get(i).cloned().unwrap_or(0)),
            length,
        }
    }

    pub fn first(&self) -> usize {
        self.tokens[0]
    }

    pub fn last(&self) -> usize {
        /* SAFETY: safe since self.length is non zero */
        self.tokens[self.length.get() - 1]
    }
}

/// The parser rule declaration location is a debug helper that provides informations
/// on where the rule has been written. This is only usefull while building the rule set,
/// if multiple rules have the same left hend side.
#[derive(Debug, Clone)]
pub struct ParserRuleDeclarationLocation {
    pub file: &'static str,
    pub line: u32,
}

impl ParserRuleDeclarationLocation {
    #[track_caller]
    pub fn here() -> Self {
        let caller = std::panic::Location::caller();
        Self {
            file: caller.file(),
            line: caller.line(),
        }
    }
}

impl std::fmt::Display for ParserRuleDeclarationLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}
