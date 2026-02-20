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
        ability: ability_tree::ability::statik::charasteristic_defining_ability::CharacteristicDefiningAbility,
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
        source: ability_tree::event::replacement::source_ref::EventSourceReference,
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
        choices: Vec<ability_tree::ability::spell::SpellAbility>,
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
    ReplacedCounterKind {
        kind: ability_tree::event::replacement::counter_on_permanent::ReplacedCounterKind,
    },
    ReplacedTokenKind {
        kind: ability_tree::event::replacement::token_creation::ReplacedTokenKind,
    },
    SpellAbility {
        ability: crate::ability_tree::ability::spell::SpellAbility,
    },
    Statement {
        statement: ability_tree::statement::Statement,
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
