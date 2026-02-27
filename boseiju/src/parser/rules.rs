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
mod ability_kind;
mod ability_tree;
mod conditional;
mod continuous_effect_kind;
mod continuous_effects;
mod cost;
mod cost_modifications;
mod count_specifier;
mod event;
mod event_replacement;
mod imperative;
mod keyword_to_ability;
mod mana;
mod number;
mod object_references;
mod object_specifiers;
mod replacement_effect;
mod spell_ability;
mod statement;
mod statik;
mod triggered_ability;
mod zone;

use crate::parser::node::ParserNode;

pub fn default_rules() -> impl Iterator<Item = ParserRule> {
    /* Mmh, I think this will create vtables for Iterator<Item = ParserRule> ? */
    let rules_iters: Vec<Box<dyn Iterator<Item = ParserRule>>> = vec![
        Box::new(ability::rules()),
        Box::new(ability_tree::rules()),
        Box::new(continuous_effects::rules()),
        Box::new(continuous_effect_kind::rules()),
        Box::new(cost::rules()),
        Box::new(cost_modifications::rules()),
        Box::new(count_specifier::rules()),
        Box::new(event::rules()),
        Box::new(event_replacement::rules()),
        Box::new(conditional::rules()),
        Box::new(imperative::rules()),
        Box::new(keyword_to_ability::rules()),
        Box::new(mana::rules()),
        Box::new(number::rules()),
        Box::new(object_references::rules()),
        Box::new(object_specifiers::rules()),
        Box::new(replacement_effect::rules()),
        Box::new(spell_ability::rules()),
        Box::new(statement::rules()),
        Box::new(statik::rules()),
        Box::new(triggered_ability::rules()),
        Box::new(ability_kind::rules()),
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
    pub expanded: RuleLhs,
    pub merged: usize,
    pub reduction: fn(&[ParserNode]) -> Result<ParserNode, &'static str>,
    pub creation_loc: ParserRuleDeclarationLocation,
}

impl PartialEq for ParserRule {
    fn eq(&self, other: &Self) -> bool {
        self.expanded == other.expanded && self.merged == other.merged
    }
}

impl Eq for ParserRule {}

impl std::hash::Hash for ParserRule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.expanded.hash(state);
        self.merged.hash(state);
    }
}

/// The rule left hand side.
///
/// This is basically an non empty array of token ids that are matched by that rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleLhs {
    tokens: [usize; Self::MAX_TOKENS],
    pub length: std::num::NonZeroUsize,
}

impl RuleLhs {
    pub const MAX_TOKENS: usize = 16;

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
}

impl std::ops::Deref for RuleLhs {
    type Target = [usize];
    fn deref(&self) -> &Self::Target {
        &self.tokens[0..self.length.get()]
    }
}

/// The parser rule declaration location is a debug helper that provides informations
/// on where the rule has been written. This is only usefull while building the rule set,
/// if multiple rules have the same left hend side.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
