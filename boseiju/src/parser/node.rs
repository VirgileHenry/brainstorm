use crate::ability_tree;

/// Since this can carry entire ability trees, we need to box the biggest variants.
/// Otherwise, this can easily blow up the stack when attempting to store multiple of them.
/// Current size is 112 bytes, let's try to keep it around here ?
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParserNode {
    LexerToken(crate::lexer::tokens::TokenKind),
    Ability {
        ability: Box<ability_tree::ability::Ability>,
    },
    AbilityTree {
        tree: Box<ability_tree::AbilityTree>,
    },
    CharacteristicDefiningAbility {
        ability: ability_tree::charasteristic_defining_ability::CharasteristicDefiningAbility,
    },
    ContinuousEffect {
        effect: ability_tree::continuous_effect::ContinuousEffect,
    },
    ContinuousEffectKind {
        inner: ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind,
    },
    Cost {
        inner: ability_tree::cost::Cost,
    },
    Imperative {
        inner: ability_tree::imperative::Imperative,
    },
    ObjectKind {
        inner: ability_tree::object::ObjectKind,
    },
    ObjectReference {
        inner: ability_tree::object::ObjectReference,
    },
    ObjectSpecifier {
        inner: ability_tree::object::ObjectSpecifier,
    },
    ObjectSpecifiers {
        inner: ability_tree::object::ObjectSpecifiers,
    },
    Statement {
        statement: ability_tree::statement::Statement,
    },
    TriggerCondition {
        condition: ability_tree::ability::triggered::trigger_cond::TriggerCondition,
    },
}

impl<'src> From<crate::lexer::tokens::Token<'src>> for ParserNode {
    fn from(token: crate::lexer::tokens::Token<'src>) -> Self {
        ParserNode::LexerToken(token.kind)
    }
}
