pub mod non_terminals;

use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::ability_tree::terminals::Terminal;
use crate::ability_tree::zone;
use crate::lexer::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub span: crate::lexer::span::Span<'src>,
}

impl<'src> Token<'src> {
    pub fn try_from_str(span: Span<'src>) -> Option<Token<'src>> {
        if let Some(kind) = terminals::Number::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Number(kind),
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
        } else if let Some(kind) = terminals::Appartenance::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Appartenance(kind),
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
        } else if let Some(kind) = terminals::PowerToughness::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PowerToughness(kind),
                span,
            })
        } else if let Some(kind) = terminals::PowerToughnessModifier::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PowerToughnessModifier(kind),
                span,
            })
        } else if let Some(kind) = terminals::PlaneswalkerAbilityCost::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PlaneswalkerAbilityCost(kind),
                span,
            })
        } else if let Some(kind) = terminals::SagaChapterNumber::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::SagaChapterNumber(kind),
                span,
            })
        } else if let Some(kind) = terminals::ContinuousEffectDuration::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::ContinuousEffectDuration(kind),
                span,
            })
        } else if let Some(kind) = terminals::NamedToken::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::NamedToken(kind),
                span,
            })
        } else if let Some(kind) = zone::Zone::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Zone(kind),
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
        } else if let Some(kind) = mtg_data::KeywordAbility::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::KeywordAbility(kind),
                span,
            })
        } else if let Some(kind) = mtg_data::KeywordAction::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::KeywordAction(kind),
                span,
            })
        } else if let Some(kind) = mtg_data::Mana::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::Mana(kind),
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
        } else if let Some(kind) = non_terminals::SelfReferencing::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::SelfReferencing(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::NumberReference::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::NumberReference(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::NotOfAKind::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::NotOfAKind(kind),
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
        } else if let Some(kind) = non_terminals::ThisTurn::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::ThisTurn(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::NonKind::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::NonKind(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::AmountReplacement::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::AmountReplacement(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::UnderControl::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::UnderControl(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::IntoHand::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::IntoHand(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::PlayerProperties::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::PlayerProperties(kind),
                span,
            })
        } else if let Some(kind) = non_terminals::IndirectReference::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::IndirectReference(kind),
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
        } else if let Some(kind) = non_terminals::AnyNumberOfClause::try_from_str(span.text) {
            Some(Self {
                kind: TokenKind::AnyNumberOfClause(kind),
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
        TokenKind::COUNT
    }
}

#[derive(idris::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    Number(terminals::Number),
    Counter(terminals::Counter),
    CountSpecifier(terminals::CountSpecifier),
    ControlSpecifier(terminals::ControlSpecifier),
    OwnerSpecifier(terminals::OwnerSpecifier),
    Order(terminals::Order),
    Appartenance(terminals::Appartenance),
    CardActions(terminals::CardActions),
    PlayerSpecifier(terminals::PlayerSpecifier),
    PermanentState(terminals::PermanentState),
    PermanentProperty(terminals::PermanentProperty),
    SpellProperty(terminals::SpellProperty),
    Phase(terminals::Phase),
    Step(terminals::Step),
    PowerToughness(terminals::PowerToughness),
    PowerToughnessModifier(terminals::PowerToughnessModifier),
    PlaneswalkerAbilityCost(terminals::PlaneswalkerAbilityCost),
    SagaChapterNumber(terminals::SagaChapterNumber),
    ContinuousEffectDuration(terminals::ContinuousEffectDuration),
    NamedToken(terminals::NamedToken),
    Zone(zone::Zone),
    Color(mtg_data::Color),
    AbilityWord(mtg_data::AbilityWord),
    KeywordAbility(mtg_data::KeywordAbility),
    KeywordAction(mtg_data::KeywordAction),
    Mana(mtg_data::Mana),
    ObjectKind(object::ObjectKind),
    ControlFlow(non_terminals::ControlFlow),
    TapUntapCost(non_terminals::TapUntapCost),
    EnglishKeyword(non_terminals::EnglishKeyword),
    SelfReferencing(non_terminals::SelfReferencing),
    NumberReference(non_terminals::NumberReference),
    NotOfAKind(non_terminals::NotOfAKind),
    ActionKeyword(non_terminals::ActionKeyword),
    DamageKind(non_terminals::DamageKind),
    PlayerAction(non_terminals::PlayerAction),
    ThisTurn(non_terminals::ThisTurn),
    NonKind(non_terminals::NonKind),
    AmountReplacement(non_terminals::AmountReplacement),
    UnderControl(non_terminals::UnderControl),
    IntoHand(non_terminals::IntoHand),
    PlayerProperties(non_terminals::PlayerProperties),
    IndirectReference(non_terminals::IndirectReference),
    NumberOfTimes(non_terminals::NumberOfTimes),
    ChoiceReference(non_terminals::ChoiceReference),
    Choice(non_terminals::Choice),
    AnyNumberOfClause(non_terminals::AnyNumberOfClause),
    WinLoseClause(non_terminals::WinLoseClause),
    VhyToSortLater(non_terminals::VhyToSortLater),
}
