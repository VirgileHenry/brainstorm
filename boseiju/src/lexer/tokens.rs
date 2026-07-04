pub mod intermediates;

use crate::ability_tree::state;
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
    AbilityWord(intermediates::AbilityWord),
    ActionKeyword(intermediates::ActionKeyword),
    AmbiguousToken(intermediates::AmbiguousToken),
    AnyNumberOfClause { clauses: intermediates::AnyNumberOfClause },
    AttachedObject(intermediates::AttachedObject),
    BackwardDuration(time::BackwardDuration),
    Bid(intermediates::Bid),
    CardActions(intermediates::CardActions),
    CardFace(intermediates::CardFace),
    CreatureGrouping(intermediates::CreatureGrouping),
    Choice(intermediates::Choice),
    ChoiceReference(intermediates::ChoiceReference),
    CoinFlip(intermediates::CoinFlip),
    Color(terminals::Color),
    ControlFlow(intermediates::ControlFlow),
    CountSpecifier(intermediates::CountSpecifier),
    Counter(terminals::Counter),
    DamageKind(terminals::DamageKind),
    DayNight(intermediates::DayNight),
    Die(intermediates::DieRoll),
    Direction(intermediates::Direction),
    EnglishKeyword(intermediates::EnglishKeyword),
    ForwardDuration(time::ForwardDuration),
    FlavorWord(terminals::FlavorWord),
    GlobalZone(intermediates::GlobalZone),
    InAdditionToPayingItsOtherCost(intermediates::InAdditionToPayingItsOtherCost),
    KeywordAbility(intermediates::KeywordAbility),
    KeywordAction(intermediates::KeywordAction),
    Mana { mana: terminals::Mana },
    MayChooseTheSameModeMoreThanOnce(intermediates::MayChooseTheSameModeMoreThanOnce),
    NamedCard(terminals::NamedCard),
    NamedChoice(terminals::NamedChoice),
    NamedExpansion(terminals::NamedExpansion),
    NamedPartners(terminals::NamedPartners),
    NamedToken(terminals::NamedToken),
    NamedTransformation(terminals::NamedTransformation),
    NamedVotes(terminals::NamedVotes),
    NonKind(intermediates::NonKind),
    NotOfAKind { not: intermediates::NotOfAKind },
    Number(intermediates::Number),
    NumberOfTimes(intermediates::NumberOfTimes),
    NumberOperation(intermediates::NumberOperation),
    Order(terminals::Order),
    OwnableZone(zone::OwnableZone),
    OwnerSpecifier(terminals::OwnerSpecifier),
    CardProperty(intermediates::CardProperty),
    CardState(intermediates::CardState),
    CardOwnName(intermediates::CardOwnName),
    PartnerKind(intermediates::PartnerKind),
    Phase(terminals::Phase),
    PlayerAction(intermediates::PlayerAction),
    PlayerDesignation(intermediates::PlayerDesignation),
    PlayerProperties(intermediates::PlayerProperties),
    PlayerSpecifier(intermediates::PlayerSpecifier),
    PowerToughnessModElements(intermediates::PowerToughnessModElements),
    PowerToughness { pt: terminals::PowerToughness },
    SagaChapterNumber { chapter: terminals::SagaChapterNumber },
    SpecialCost(intermediates::SpecialCost),
    StackObjectState(state::StackObjectState),
    Step(terminals::Step),
    TapUntapCost(intermediates::TapUntapCost),
    TheSameIsTrueFor(intermediates::TheSameIsTrueFor),
    UnderControl(intermediates::UnderControl),
    VhyToSortLater(intermediates::VhyToSortLater),
    WinLoseClause(intermediates::WinLoseClause),
    ArtifactSubtype(terminals::ArtifactSubtype),
    BattleSubtype(terminals::BattleSubtype),
    CardType(terminals::CardType),
    CreatureSubtype(terminals::CreatureSubtype),
    EnchantmentSubtype(terminals::EnchantmentSubtype),
    LandSubtype(terminals::LandSubtype),
    PlaneswalkerSubtype(terminals::PlaneswalkerSubtype),
    InstantSorcerySubtype(terminals::InstantSorcerySubtype),
    Supertype(terminals::Supertype),
}

impl Token {
    pub fn try_from_span(span: Span) -> Option<Token> {
        if let Some(kind) = intermediates::AmbiguousToken::try_from_span(&span) {
            Some(Self::AmbiguousToken(kind))
        } else if let Some(kind) = terminals::Counter::try_from_span(&span) {
            Some(Self::Counter(kind))
        } else if let Some(kind) = intermediates::CountSpecifier::try_from_span(&span) {
            Some(Self::CountSpecifier(kind))
        } else if let Some(kind) = intermediates::AttachedObject::try_from_span(&span) {
            Some(Self::AttachedObject(kind))
        } else if let Some(kind) = terminals::OwnerSpecifier::try_from_span(&span) {
            Some(Self::OwnerSpecifier(kind))
        } else if let Some(kind) = terminals::Order::try_from_span(&span) {
            Some(Self::Order(kind))
        } else if let Some(kind) = intermediates::Bid::try_from_span(&span) {
            Some(Self::Bid(kind))
        } else if let Some(kind) = intermediates::CardActions::try_from_span(&span) {
            Some(Self::CardActions(kind))
        } else if let Some(kind) = intermediates::CardFace::try_from_span(&span) {
            Some(Self::CardFace(kind))
        } else if let Some(kind) = intermediates::CreatureGrouping::try_from_span(&span) {
            Some(Self::CreatureGrouping(kind))
        } else if let Some(kind) = intermediates::PlayerSpecifier::try_from_span(&span) {
            Some(Self::PlayerSpecifier(kind))
        } else if let Some(kind) = intermediates::CardState::try_from_span(&span) {
            Some(Self::CardState(kind))
        } else if let Some(kind) = intermediates::CardOwnName::try_from_span(&span) {
            Some(Self::CardOwnName(kind))
        } else if let Some(kind) = intermediates::CardProperty::try_from_span(&span) {
            Some(Self::CardProperty(kind))
        } else if let Some(kind) = state::StackObjectState::try_from_span(&span) {
            Some(Self::StackObjectState(kind))
        } else if let Some(kind) = intermediates::SpecialCost::try_from_span(&span) {
            Some(Self::SpecialCost(kind))
        } else if let Some(kind) = terminals::Phase::try_from_span(&span) {
            Some(Self::Phase(kind))
        } else if let Some(kind) = intermediates::PartnerKind::try_from_span(&span) {
            Some(Self::PartnerKind(kind))
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
        } else if let Some(kind) = terminals::FlavorWord::try_from_span(&span) {
            Some(Self::FlavorWord(kind))
        } else if let Some(kind) = crate::ability_tree::time::BackwardDuration::try_from_span(&span) {
            Some(Self::BackwardDuration(kind))
        } else if let Some(kind) = terminals::NamedCard::try_from_span(&span) {
            Some(Self::NamedCard(kind))
        } else if let Some(kind) = terminals::NamedChoice::try_from_span(&span) {
            Some(Self::NamedChoice(kind))
        } else if let Some(kind) = terminals::NamedExpansion::try_from_span(&span) {
            Some(Self::NamedExpansion(kind))
        } else if let Some(kind) = terminals::NamedPartners::try_from_span(&span) {
            Some(Self::NamedPartners(kind))
        } else if let Some(kind) = terminals::NamedToken::try_from_span(&span) {
            Some(Self::NamedToken(kind))
        } else if let Some(kind) = zone::OwnableZone::try_from_span(&span) {
            Some(Self::OwnableZone(kind))
        } else if let Some(kind) = terminals::Color::try_from_span(&span) {
            Some(Self::Color(kind))
        } else if let Some(kind) = intermediates::AbilityWord::try_from_span(&span) {
            Some(Self::AbilityWord(kind))
        } else if let Some(kind) = intermediates::KeywordAbility::try_from_span(&span) {
            Some(Self::KeywordAbility(kind))
        } else if let Some(kind) = intermediates::KeywordAction::try_from_span(&span) {
            Some(Self::KeywordAction(kind))
        } else if let Some(mana) = terminals::Mana::try_from_span(&span) {
            Some(Self::Mana { mana })
        } else if let Some(kind) = intermediates::MayChooseTheSameModeMoreThanOnce::try_from_span(&span) {
            Some(Self::MayChooseTheSameModeMoreThanOnce(kind))
        } else if let Some(kind) = intermediates::ControlFlow::try_from_span(&span) {
            Some(Self::ControlFlow(kind))
        } else if let Some(kind) = intermediates::TapUntapCost::try_from_span(&span) {
            Some(Self::TapUntapCost(kind))
        } else if let Some(kind) = intermediates::TheSameIsTrueFor::try_from_span(&span) {
            Some(Self::TheSameIsTrueFor(kind))
        } else if let Some(kind) = terminals::NamedTransformation::try_from_span(&span) {
            Some(Self::NamedTransformation(kind))
        } else if let Some(kind) = terminals::NamedVotes::try_from_span(&span) {
            Some(Self::NamedVotes(kind))
        } else if let Some(kind) = intermediates::EnglishKeyword::try_from_span(&span) {
            Some(Self::EnglishKeyword(kind))
        } else if let Some(kind) = intermediates::Number::try_from_span(&span) {
            Some(Self::Number(kind))
        } else if let Some(not) = intermediates::NotOfAKind::try_from_span(&span) {
            Some(Self::NotOfAKind { not })
        } else if let Some(kind) = intermediates::ActionKeyword::try_from_span(&span) {
            Some(Self::ActionKeyword(kind))
        } else if let Some(kind) = terminals::DamageKind::try_from_span(&span) {
            Some(Self::DamageKind(kind))
        } else if let Some(kind) = intermediates::DayNight::try_from_span(&span) {
            Some(Self::DayNight(kind))
        } else if let Some(kind) = intermediates::DieRoll::try_from_span(&span) {
            Some(Self::Die(kind))
        } else if let Some(kind) = intermediates::Direction::try_from_span(&span) {
            Some(Self::Direction(kind))
        } else if let Some(kind) = intermediates::PlayerAction::try_from_span(&span) {
            Some(Self::PlayerAction(kind))
        } else if let Some(kind) = intermediates::PlayerDesignation::try_from_span(&span) {
            Some(Self::PlayerDesignation(kind))
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
        } else if let Some(kind) = intermediates::CoinFlip::try_from_span(&span) {
            Some(Self::CoinFlip(kind))
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
        } else if let Some(kind) = terminals::ArtifactSubtype::try_from_span(&span) {
            Some(Self::ArtifactSubtype(kind))
        } else if let Some(kind) = terminals::BattleSubtype::try_from_span(&span) {
            Some(Self::BattleSubtype(kind))
        } else if let Some(kind) = terminals::CardType::try_from_span(&span) {
            Some(Self::CardType(kind))
        } else if let Some(kind) = terminals::CreatureSubtype::try_from_span(&span) {
            Some(Self::CreatureSubtype(kind))
        } else if let Some(kind) = terminals::EnchantmentSubtype::try_from_span(&span) {
            Some(Self::EnchantmentSubtype(kind))
        } else if let Some(kind) = terminals::LandSubtype::try_from_span(&span) {
            Some(Self::LandSubtype(kind))
        } else if let Some(kind) = terminals::PlaneswalkerSubtype::try_from_span(&span) {
            Some(Self::PlaneswalkerSubtype(kind))
        } else if let Some(kind) = terminals::InstantSorcerySubtype::try_from_span(&span) {
            Some(Self::InstantSorcerySubtype(kind))
        } else if let Some(kind) = terminals::Supertype::try_from_span(&span) {
            Some(Self::Supertype(kind))
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
            Self::AttachedObject(child) => child.span(),
            Self::BackwardDuration(child) => child.node_span(),
            Self::Bid(child) => child.span(),
            Self::CardActions(child) => child.span(),
            Self::CardFace(child) => child.span(),
            Self::CreatureGrouping(child) => child.span(),
            Self::CardOwnName(child) => child.span,
            Self::Choice(child) => child.span(),
            Self::ChoiceReference(child) => child.span(),
            Self::CoinFlip(child) => child.span(),
            Self::Color(child) => child.span,
            Self::ControlFlow(child) => child.span(),
            Self::CountSpecifier(child) => child.span(),
            Self::Counter(child) => child.span,
            Self::DamageKind(child) => child.node_span(),
            Self::DayNight(child) => child.span(),
            Self::Die(child) => child.span(),
            Self::Direction(child) => child.span(),
            Self::EnglishKeyword(child) => child.span(),
            Self::ForwardDuration(child) => child.node_span(),
            Self::FlavorWord(child) => child.node_span(),
            Self::GlobalZone(child) => child.span(),
            Self::InAdditionToPayingItsOtherCost(child) => child.span,
            Self::KeywordAbility(child) => child.span,
            Self::KeywordAction(child) => child.span,
            Self::Mana { mana } => mana.node_span(),
            Self::MayChooseTheSameModeMoreThanOnce(child) => child.span,
            Self::NamedCard(child) => child.node_span(),
            Self::NamedChoice(child) => child.node_span(),
            Self::NamedExpansion(child) => child.node_span(),
            Self::NamedPartners(child) => child.node_span(),
            Self::NamedToken(child) => child.node_span(),
            Self::NonKind(child) => child.span(),
            Self::NotOfAKind { not } => not.span,
            Self::Number(child) => child.span(),
            Self::NumberOfTimes(child) => child.span(),
            Self::NumberOperation(child) => child.span(),
            Self::Order(child) => child.node_span(),
            Self::OwnableZone(child) => child.node_span(),
            Self::OwnerSpecifier(child) => child.node_span(),
            Self::CardProperty(child) => child.span(),
            Self::CardState(child) => child.span(),
            Self::PartnerKind(child) => child.span(),
            Self::Phase(child) => child.node_span(),
            Self::PlayerAction(child) => child.span(),
            Self::PlayerDesignation(child) => child.span(),
            Self::PlayerProperties(child) => child.span(),
            Self::PlayerSpecifier(child) => child.span(),
            Self::PowerToughnessModElements(child) => child.span(),
            Self::PowerToughness { pt } => pt.span,
            Self::SagaChapterNumber { chapter } => chapter.span,
            Self::StackObjectState(child) => child.node_span(),
            Self::SpecialCost(child) => child.span(),
            Self::Step(child) => child.node_span(),
            Self::TapUntapCost(child) => child.span(),
            Self::TheSameIsTrueFor(child) => child.span,
            Self::NamedTransformation(child) => child.node_span(),
            Self::NamedVotes(child) => child.node_span(),
            Self::UnderControl(child) => child.span(),
            Self::VhyToSortLater(child) => child.span(),
            Self::WinLoseClause(child) => child.span(),
            Self::ArtifactSubtype(child) => child.node_span(),
            Self::BattleSubtype(child) => child.node_span(),
            Self::CardType(child) => child.node_span(),
            Self::CreatureSubtype(child) => child.node_span(),
            Self::EnchantmentSubtype(child) => child.node_span(),
            Self::LandSubtype(child) => child.node_span(),
            Self::PlaneswalkerSubtype(child) => child.node_span(),
            Self::InstantSorcerySubtype(child) => child.node_span(),
            Self::Supertype(child) => child.node_span(),
        }
    }
}
