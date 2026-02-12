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
mod object_kinds;
mod object_references;
mod object_specifiers;
mod replacement_effect;
mod statement;
mod trigger_condition;
mod zone;

use crate::parser::node::ParserNode;

pub fn default_rules() -> impl Iterator<Item = ParserRule> {
    /* Mmh, I think this will create vtables for Iterator<Item = ParserRule> ? */
    let rules_iters: Vec<Box<dyn Iterator<Item = ParserRule>>> = vec![
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
        Box::new(object_kinds::rules()),
        Box::new(object_references::rules()),
        Box::new(object_specifiers::rules()),
        Box::new(replacement_effect::rules()),
        Box::new(statement::rules()),
        Box::new(trigger_condition::rules()),
        Box::new(zone::rules()),
    ];
    rules_iters.into_iter().flatten()
}

/// A single rule of merging nodes together for the parser.
#[derive(Debug, Clone)]
pub struct ParserRule {
    pub from: RuleLhs,
    pub result: usize,
    pub reduction: fn(&[ParserNode]) -> Option<ParserNode>,
    pub creation_loc: ParserRuleDeclarationLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleLhs {
    pub tokens: [usize; Self::MAX_TOKENS],
    pub length: usize,
}

impl RuleLhs {
    const MAX_TOKENS: usize = 8;

    pub fn new(tokens: &[usize]) -> Self {
        if tokens.len() > Self::MAX_TOKENS {
            panic!("Can only create rules with up to {} tokens", Self::MAX_TOKENS);
        }
        if tokens.is_empty() {
            panic!("Can only create rules with at least 1 token");
        }

        Self {
            tokens: std::array::from_fn(|i| tokens.get(i).cloned().unwrap_or(0)),
            length: tokens.len(),
        }
    }

    pub fn first(&self) -> usize {
        self.tokens[0]
    }

    pub fn last(&self) -> usize {
        self.tokens[self.length - 1]
    }
}

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
