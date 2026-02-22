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
    ContinuousEffect {
        effect: ability_tree::ability::statik::continuous_effect::ContinuousEffect,
    },
    ContinuousEffectKind {
        kind: ability_tree::ability::statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind,
    },
    Cost {
        inner: ability_tree::cost::Cost,
    },
    CostModification {
        cost_modification: ability_tree::ability::statik::cost_modification_effect::CostModification,
    },
    CostModificationEffect {
        cost_modification: ability_tree::ability::statik::cost_modification_effect::CostModificationEffect,
    },
    CreatedTokenKind {
        kind: ability_tree::imperative::CreatedTokenKind,
    },
    Event {
        event: ability_tree::event::Event,
    },
    EventReplacement {
        replacement: ability_tree::event::replacement::EventReplacement,
    },
    EventSource {
        source: ability_tree::event::source::EventSource,
    },
    EventSourceReference {
        source: ability_tree::event::replacement::EventSourceReference,
    },
    ExileFollowUp {
        follow_up: ability_tree::imperative::ExileFollowUp,
    },
    IfCondition {
        condition: ability_tree::if_condition::IfCondition,
    },
    Imperative {
        imperative: ability_tree::imperative::Imperative,
    },
    ImperativeChoices {
        choices: Box<arrayvec::ArrayVec<ability_tree::ability::spell::SpellAbility, 23 /* Fixme */>>,
    },
    KeywordAbility {
        ability: ability_tree::ability::KeywordAbility,
    },
    ManaCost {
        mana_cost: ability_tree::terminals::ManaCost,
    },
    Number {
        number: ability_tree::number::Number,
    },
    ObjectReference {
        reference: ability_tree::object::ObjectReference,
    },
    ObjectSpecifier {
        specifier: ability_tree::object::ObjectSpecifier,
    },
    ObjectSpecifiers {
        specifiers: ability_tree::object::ObjectSpecifiers,
    },
    PreviouslyMentionnedObject {
        object: ability_tree::object::PreviouslyMentionnedObject,
    },
    PutCounterKind {
        kind: ability_tree::imperative::CounterKind,
    },
    SpellAbility {
        ability: crate::ability_tree::ability::spell::SpellAbility,
    },
    Statement {
        statement: ability_tree::statement::Statement,
    },
    WrittenOrKeywordAbilty {
        ability: Box<ability_tree::ability::WrittenOrKeywordAbilty>,
    },
    ZoneReference {
        zone: ability_tree::zone::ZoneReference,
    },
}

impl<'src> From<crate::lexer::tokens::Token<'src>> for ParserNode {
    fn from(token: crate::lexer::tokens::Token<'src>) -> Self {
        ParserNode::LexerToken(token.kind)
    }
}
