use crate::ability_tree;

/// Since this can carry entire ability trees, we need to box the biggest variants.
/// Otherwise, this can easily blow up the stack when attempting to store multiple of them.
/// Current size is 112 bytes, let's try to keep it around here ?
#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParserNode {
    LexerToken(crate::lexer::tokens::TokenKind),
    Ability {
        ability: Box<ability_tree::ability::Ability>,
    },
    AbilityTree {
        tree: Box<ability_tree::AbilityTree>,
    },
    Condition {
        condition: ability_tree::conditional::Condition,
    },
    ConditionalImperative {
        imperative: ability_tree::imperative::ConditionalImperative,
    },
    ContinuousEffect {
        effect: ability_tree::ability::statik::continuous_effect::ContinuousEffect,
    },
    ContinuousEffectKind {
        kind: ability_tree::ability::statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind,
    },
    Cost {
        cost: ability_tree::cost::Cost,
    },
    CostModification {
        cost_modification: ability_tree::ability::statik::cost_modification_effect::CostModification,
    },
    CostModificationEffect {
        cost_modification: ability_tree::ability::statik::cost_modification_effect::CostModificationEffect,
    },
    CountSpecifier {
        count: crate::ability_tree::object::CountSpecifier,
    },
    CreatedTokenKind {
        kind: ability_tree::imperative::CreatedTokenKind,
    },
    CreatureAction {
        action: ability_tree::event::CreatureAction,
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
    Imperative {
        imperative: ability_tree::imperative::Imperative,
    },
    ImperativeList {
        imperatives: ability_tree::imperative::ImperativeList,
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
    PlayerAction {
        action: ability_tree::event::PlayerAction,
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
    StaticAbilityKind {
        kind: crate::ability_tree::ability::statik::StaticAbilityKind,
    },
    TriggerCondition {
        condition: ability_tree::ability::triggered::TriggerCondition,
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
