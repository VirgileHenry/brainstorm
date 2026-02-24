pub mod non_terminals;

use std::str::FromStr;

use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::ability_tree::terminals::Terminal;
use crate::ability_tree::time;
use crate::ability_tree::zone;
use crate::lexer::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub span: crate::lexer::span::Span<'src>,
}

impl<'src> Token<'src> {
    pub fn try_from_str(span: Span<'src>) -> Option<Token<'src>> {
        if let Some(kind) = non_terminals::AmbiguousToken::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::AmbiguousToken(kind),
                span,
            })
        } else if let Some(kind) = terminals::Counter::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Counter(kind),
                span,
            })
        } else if let Some(kind) = terminals::CountSpecifier::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::CountSpecifier(kind),
                span,
            })
        } else if let Some(kind) = terminals::ControlSpecifier::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::ControlSpecifier(kind),
                span,
            })
        } else if let Some(kind) = terminals::OwnerSpecifier::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::OwnerSpecifier(kind),
                span,
            })
        } else if let Some(kind) = terminals::Order::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Order(kind),
                span,
            })
        } else if let Some(kind) = terminals::CardActions::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::CardActions(kind),
                span,
            })
        } else if let Some(kind) = terminals::PlayerSpecifier::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PlayerSpecifier(kind),
                span,
            })
        } else if let Some(kind) = terminals::PermanentState::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PermanentState(kind),
                span,
            })
        } else if let Some(kind) = terminals::PermanentProperty::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PermanentProperty(kind),
                span,
            })
        } else if let Some(kind) = terminals::SpellProperty::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::SpellProperty(kind),
                span,
            })
        } else if let Some(kind) = terminals::Phase::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Phase(kind),
                span,
            })
        } else if let Some(kind) = terminals::Step::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Step(kind),
                span,
            })
        } else if let Some(pt) = terminals::PowerToughness::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PowerToughness { pt },
                span,
            })
        } else if let Some(kind) = terminals::PowerToughnessModifier::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PowerToughnessModifier(kind),
                span,
            })
        } else if let Some(cost) = terminals::PlaneswalkerAbilityCost::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PlaneswalkerAbilityCost { cost },
                span,
            })
        } else if let Some(chapter) = terminals::SagaChapterNumber::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::SagaChapterNumber { chapter },
                span,
            })
        } else if let Some(kind) = crate::ability_tree::time::Instant::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Instant(kind),
                span,
            })
        } else if let Some(kind) = crate::ability_tree::time::ForwardDuration::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::ForwardDuration(kind),
                span,
            })
        } else if let Some(kind) = crate::ability_tree::time::BackwardDuration::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::BackwardDuration(kind),
                span,
            })
        } else if let Some(kind) = terminals::NamedToken::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::NamedToken(kind),
                span,
            })
        } else if let Some(kind) = zone::OwnableZone::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::OwnableZone(kind),
                span,
            })
        } else if let Some(kind) = mtg_data::Color::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Color(kind),
                span,
            })
        } else if let Some(kind) = mtg_data::AbilityWord::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::AbilityWord(kind),
                span,
            })
        } else if let Some(kind) = mtg_data::KeywordAbility::from_str(span.text).ok() {
            Some(Self {
                kind: TokenKind::KeywordAbility(kind),
                span,
            })
        } else if let Some(kind) = mtg_data::KeywordAction::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::KeywordAction(kind),
                span,
            })
        } else if let Some(mana) = mtg_data::Mana::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Mana { mana },
                span,
            })
        } else if let Some(kind) = object::ObjectKind::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::ObjectKind(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::ControlFlow::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::ControlFlow(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::TapUntapCost::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::TapUntapCost(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::EnglishKeyword::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::EnglishKeyword(kind),
                span,
            })
        } else if let Some(reference) = object::SelfReferencingObject::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::SelfReferencing { reference },
                span,
            })
        } else if let Some(number) = non_terminals::Number::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Number(number),
                span,
            })
        } else if let Some(not) = non_terminals::NotOfAKind::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::NotOfAKind { not },
                span,
            })
        } else if let Some(kind) = non_terminals::ActionKeyword::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::ActionKeyword(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::DamageKind::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::DamageKind(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::PlayerAction::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PlayerAction(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::NonKind::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::NonKind(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::UnderControl::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::UnderControl(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::PlayerProperties::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PlayerProperties(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::NumberOfTimes::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::NumberOfTimes(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::ChoiceReference::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::ChoiceReference(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::Choice::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Choice(kind),
                span,
            })
        } else if let Some(clauses) = non_terminals::AnyNumberOfClause::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::AnyNumberOfClause { clauses },
                span,
            })
        } else if let Some(kind) = non_terminals::WinLoseClause::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::WinLoseClause(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::VhyToSortLater::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::VhyToSortLater(kind),
                span,
            })
        } else {
            None
        }
    }

    pub const TOKEN_COUNT: usize = 0;
    pub fn token_id(&self) -> usize {
        <TokenKind as idris::Idris>::COUNT
    }
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    AbilityWord(mtg_data::AbilityWord),
    ActionKeyword(non_terminals::ActionKeyword),
    AmbiguousToken(non_terminals::AmbiguousToken),
    AnyNumberOfClause { clauses: non_terminals::AnyNumberOfClause },
    BackwardDuration(time::BackwardDuration),
    CardActions(terminals::CardActions),
    Choice(non_terminals::Choice),
    ChoiceReference(non_terminals::ChoiceReference),
    Color(mtg_data::Color),
    ControlFlow(non_terminals::ControlFlow),
    ControlSpecifier(terminals::ControlSpecifier),
    CountSpecifier(terminals::CountSpecifier),
    Counter(terminals::Counter),
    DamageKind(non_terminals::DamageKind),
    EnglishKeyword(non_terminals::EnglishKeyword),
    ForwardDuration(time::ForwardDuration),
    GlobalZone(non_terminals::GlobalZone),
    Instant(time::Instant),
    KeywordAbility(mtg_data::KeywordAbility),
    KeywordAction(mtg_data::KeywordAction),
    Mana { mana: mtg_data::Mana },
    NamedToken(terminals::NamedToken),
    NonKind(non_terminals::NonKind),
    NotOfAKind { not: non_terminals::NotOfAKind },
    Number(non_terminals::Number),
    NumberOfTimes(non_terminals::NumberOfTimes),
    ObjectKind(object::ObjectKind),
    Order(terminals::Order),
    OwnableZone(zone::OwnableZone),
    OwnerSpecifier(terminals::OwnerSpecifier),
    PermanentProperty(terminals::PermanentProperty),
    PermanentState(terminals::PermanentState),
    Phase(terminals::Phase),
    PlaneswalkerAbilityCost { cost: terminals::PlaneswalkerAbilityCost },
    PlayerAction(non_terminals::PlayerAction),
    PlayerProperties(non_terminals::PlayerProperties),
    PlayerSpecifier(terminals::PlayerSpecifier),
    PowerToughness { pt: terminals::PowerToughness },
    PowerToughnessModifier(terminals::PowerToughnessModifier),
    SagaChapterNumber { chapter: terminals::SagaChapterNumber },
    SelfReferencing { reference: object::SelfReferencingObject },
    SpellProperty(terminals::SpellProperty),
    Step(terminals::Step),
    TapUntapCost(non_terminals::TapUntapCost),
    UnderControl(non_terminals::UnderControl),
    VhyToSortLater(non_terminals::VhyToSortLater),
    WinLoseClause(non_terminals::WinLoseClause),
}
