pub mod intermediates;

use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::ability_tree::time;
use crate::ability_tree::zone;
use crate::lexer::span::Span;

pub trait IntoToken: Sized {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self>;
}

#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    AbilityWord(terminals::AbilityWord),
    ActionKeyword(intermediates::ActionKeyword),
    AmbiguousToken(intermediates::AmbiguousToken),
    AnyNumberOfClause { clauses: intermediates::AnyNumberOfClause },
    BackwardDuration(time::BackwardDuration),
    CardActions(intermediates::CardActions),
    Choice(intermediates::Choice),
    ChoiceReference(intermediates::ChoiceReference),
    Color(terminals::Color),
    ControlFlow(intermediates::ControlFlow),
    CountSpecifier(intermediates::CountSpecifier),
    Counter(terminals::Counter),
    DamageKind(intermediates::DamageKind),
    EnglishKeyword(intermediates::EnglishKeyword),
    ForwardDuration(time::ForwardDuration),
    GlobalZone(intermediates::GlobalZone),
    InAdditionToPayingItsOtherCost(intermediates::InAdditionToPayingItsOtherCost),
    KeywordAbility(intermediates::KeywordAbility),
    KeywordAction(intermediates::KeywordAction),
    Mana { mana: terminals::Mana },
    NamedToken(terminals::NamedToken),
    NonKind(intermediates::NonKind),
    NotOfAKind { not: intermediates::NotOfAKind },
    Number(intermediates::Number),
    NumberOfTimes(intermediates::NumberOfTimes),
    NumberOperation(intermediates::NumberOperation),
    ObjectKind(object::ObjectKind),
    Order(terminals::Order),
    OwnableZone(zone::OwnableZone),
    OwnerSpecifier(terminals::OwnerSpecifier),
    PermanentProperty(terminals::CardProperty),
    CardState(terminals::CardState),
    Phase(terminals::Phase),
    PlayerAction(intermediates::PlayerAction),
    PlayerProperties(intermediates::PlayerProperties),
    PlayerSpecifier(intermediates::PlayerSpecifier),
    PowerToughnessModElements(intermediates::PowerToughnessModElements),
    PowerToughness { pt: terminals::PowerToughness },
    SagaChapterNumber { chapter: terminals::SagaChapterNumber },
    SelfReferencing { reference: object::SelfReferencingObject },
    SpellProperty(terminals::SpellProperty),
    Step(terminals::Step),
    TapUntapCost(intermediates::TapUntapCost),
    UnderControl(intermediates::UnderControl),
    VhyToSortLater(intermediates::VhyToSortLater),
    WinLoseClause(intermediates::WinLoseClause),
}

impl Token {
    pub fn try_from_span(span: Span) -> Option<Token> {
        if let Some(kind) = intermediates::AmbiguousToken::try_from_span(&span) {
            Some(Self::AmbiguousToken(kind))
        } else if let Some(kind) = terminals::Counter::try_from_span(&span) {
            Some(Self::Counter(kind))
        } else if let Some(kind) = intermediates::CountSpecifier::try_from_span(&span) {
            Some(Self::CountSpecifier(kind))
        } else if let Some(kind) = terminals::OwnerSpecifier::try_from_span(&span) {
            Some(Self::OwnerSpecifier(kind))
        } else if let Some(kind) = terminals::Order::try_from_span(&span) {
            Some(Self::Order(kind))
        } else if let Some(kind) = intermediates::CardActions::try_from_span(&span) {
            Some(Self::CardActions(kind))
        } else if let Some(kind) = intermediates::PlayerSpecifier::try_from_span(&span) {
            Some(Self::PlayerSpecifier(kind))
        } else if let Some(kind) = terminals::CardState::try_from_span(&span) {
            Some(Self::CardState(kind))
        } else if let Some(kind) = terminals::CardProperty::try_from_span(&span) {
            Some(Self::PermanentProperty(kind))
        } else if let Some(kind) = terminals::SpellProperty::try_from_span(&span) {
            Some(Self::SpellProperty(kind))
        } else if let Some(kind) = terminals::Phase::try_from_span(&span) {
            Some(Self::Phase(kind))
        } else if let Some(kind) = terminals::Step::try_from_span(&span) {
            Some(Self::Step(kind))
        } else if let Some(pt) = terminals::PowerToughness::try_from_span(&span) {
            Some(Self::PowerToughness { pt })
        } else if let Some(kind) = intermediates::PowerToughnessModElements::try_from_span(&span) {
            Some(Self::PowerToughnessModElements(kind))
        } else if let Some(chapter) = terminals::SagaChapterNumber::try_from_span(&span) {
            Some(Self::SagaChapterNumber { chapter })
        } else if let Some(kind) = intermediates::InAdditionToPayingItsOtherCost::try_from_span(&span) {
            Some(Self::InAdditionToPayingItsOtherCost(kind))
        } else if let Some(kind) = crate::ability_tree::time::ForwardDuration::try_from_span(&span) {
            Some(Self::ForwardDuration(kind))
        } else if let Some(kind) = crate::ability_tree::time::BackwardDuration::try_from_span(&span) {
            Some(Self::BackwardDuration(kind))
        } else if let Some(kind) = terminals::NamedToken::try_from_span(&span) {
            Some(Self::NamedToken(kind))
        } else if let Some(kind) = zone::OwnableZone::try_from_span(&span) {
            Some(Self::OwnableZone(kind))
        } else if let Some(kind) = terminals::Color::try_from_span(&span) {
            Some(Self::Color(kind))
        } else if let Some(kind) = terminals::AbilityWord::try_from_span(&span) {
            Some(Self::AbilityWord(kind))
        } else if let Some(kind) = intermediates::KeywordAbility::try_from_span(&span) {
            Some(Self::KeywordAbility(kind))
        } else if let Some(kind) = intermediates::KeywordAction::try_from_span(&span) {
            Some(Self::KeywordAction(kind))
        } else if let Some(mana) = terminals::Mana::try_from_span(&span) {
            Some(Self::Mana { mana })
        } else if let Some(kind) = object::ObjectKind::try_from_span(&span) {
            Some(Self::ObjectKind(kind))
        } else if let Some(kind) = intermediates::ControlFlow::try_from_span(&span) {
            Some(Self::ControlFlow(kind))
        } else if let Some(kind) = intermediates::TapUntapCost::try_from_span(&span) {
            Some(Self::TapUntapCost(kind))
        } else if let Some(kind) = intermediates::EnglishKeyword::try_from_span(&span) {
            Some(Self::EnglishKeyword(kind))
        } else if let Some(reference) = object::SelfReferencingObject::try_from_span(&span) {
            Some(Self::SelfReferencing { reference })
        } else if let Some(kind) = intermediates::Number::try_from_span(&span) {
            Some(Self::Number(kind))
        } else if let Some(not) = intermediates::NotOfAKind::try_from_span(&span) {
            Some(Self::NotOfAKind { not })
        } else if let Some(kind) = intermediates::ActionKeyword::try_from_span(&span) {
            Some(Self::ActionKeyword(kind))
        } else if let Some(kind) = intermediates::DamageKind::try_from_span(&span) {
            Some(Self::DamageKind(kind))
        } else if let Some(kind) = intermediates::PlayerAction::try_from_span(&span) {
            Some(Self::PlayerAction(kind))
        } else if let Some(kind) = intermediates::NonKind::try_from_span(&span) {
            Some(Self::NonKind(kind))
        } else if let Some(kind) = intermediates::UnderControl::try_from_span(&span) {
            Some(Self::UnderControl(kind))
        } else if let Some(kind) = intermediates::PlayerProperties::try_from_span(&span) {
            Some(Self::PlayerProperties(kind))
        } else if let Some(kind) = intermediates::NumberOfTimes::try_from_span(&span) {
            Some(Self::NumberOfTimes(kind))
        } else if let Some(kind) = intermediates::NumberOperation::try_from_span(&span) {
            Some(Self::NumberOperation(kind))
        } else if let Some(kind) = intermediates::ChoiceReference::try_from_span(&span) {
            Some(Self::ChoiceReference(kind))
        } else if let Some(kind) = intermediates::Choice::try_from_span(&span) {
            Some(Self::Choice(kind))
        } else if let Some(clauses) = intermediates::AnyNumberOfClause::try_from_span(&span) {
            Some(Self::AnyNumberOfClause { clauses })
        } else if let Some(kind) = intermediates::WinLoseClause::try_from_span(&span) {
            Some(Self::WinLoseClause(kind))
        } else if let Some(kind) = intermediates::GlobalZone::try_from_span(&span) {
            Some(Self::GlobalZone(kind))
        } else if let Some(kind) = intermediates::VhyToSortLater::try_from_span(&span) {
            Some(Self::VhyToSortLater(kind))
        } else {
            None
        }
    }

    #[cfg(feature = "spanned_tree")]
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        use crate::ability_tree::AbilityTreeNode;
        match self {
            Self::AbilityWord(child) => child.span,
            Self::ActionKeyword(child) => child.span(),
            Self::AmbiguousToken(child) => child.span(),
            Self::AnyNumberOfClause { clauses } => clauses.span,
            Self::BackwardDuration(child) => child.node_span(),
            Self::CardActions(child) => child.span(),
            Self::Choice(child) => child.span(),
            Self::ChoiceReference(child) => child.span(),
            Self::Color(child) => child.span,
            Self::ControlFlow(child) => child.span(),
            Self::CountSpecifier(child) => child.span(),
            Self::Counter(child) => child.span,
            Self::DamageKind(child) => child.span(),
            Self::EnglishKeyword(child) => child.span(),
            Self::ForwardDuration(child) => child.node_span(),
            Self::GlobalZone(child) => child.span(),
            Self::InAdditionToPayingItsOtherCost(child) => child.span,
            Self::KeywordAbility(child) => child.span,
            Self::KeywordAction(child) => child.span,
            Self::Mana { mana } => mana.node_span(),
            Self::NamedToken(child) => child.node_span(),
            Self::NonKind(child) => child.span(),
            Self::NotOfAKind { not } => not.span,
            Self::Number(child) => child.span(),
            Self::NumberOfTimes(child) => child.span(),
            Self::NumberOperation(child) => child.span(),
            Self::ObjectKind(child) => child.node_span(),
            Self::Order(child) => child.node_span(),
            Self::OwnableZone(child) => child.node_span(),
            Self::OwnerSpecifier(child) => child.node_span(),
            Self::PermanentProperty(child) => child.span(),
            Self::CardState(child) => child.node_span(),
            Self::Phase(child) => child.node_span(),
            Self::PlayerAction(child) => child.span(),
            Self::PlayerProperties(child) => child.span(),
            Self::PlayerSpecifier(child) => child.span(),
            Self::PowerToughnessModElements(child) => child.span(),
            Self::PowerToughness { pt } => pt.span,
            Self::SagaChapterNumber { chapter } => chapter.span,
            Self::SelfReferencing { reference } => reference.span,
            Self::SpellProperty(child) => child.span(),
            Self::Step(child) => child.node_span(),
            Self::TapUntapCost(child) => child.span(),
            Self::UnderControl(child) => child.span(),
            Self::VhyToSortLater(child) => child.span(),
            Self::WinLoseClause(child) => child.span(),
        }
    }
}
