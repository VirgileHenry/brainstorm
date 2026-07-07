mod ability_word;
mod action_keywords;
mod ambiguous_tokens;
mod any_number_of_clause;
mod attached_permanent;
mod bid;
mod card_actions;
mod card_face;
mod card_own_name;
mod card_property;
mod card_state;
mod choice;
mod choice_reference;
mod coin_flip;
mod control_flow;
mod count_specifier;
mod creature_grouping;
mod day_night;
mod die_roll;
mod direction;
mod english_keywords;
mod global_zone;
mod in_addition_to_paying_its_other_costs;
mod keyword_ability;
mod keyword_action;
mod may_choose_the_same_mode;
mod non_kind;
mod not_of_a_kind;
mod number;
mod number_of_times;
mod number_operation;
mod partner_kind;
mod player_action;
mod player_designation;
mod player_properties;
mod player_specifier;
mod plus_minus;
mod special_costs;
mod tap_untap_cost;
mod the_same_is_true_for;
mod under_control;
mod win_lose_clauses;

pub use ability_word::AbilityWord;
pub use action_keywords::ActionKeyword;
pub use ambiguous_tokens::AmbiguousToken;
pub use any_number_of_clause::AnyNumberOfClause;
pub use attached_permanent::AttachedObject;
pub use bid::Bid;
pub use card_actions::CardActions;
pub use card_face::CardFace;
pub use card_own_name::CardOwnName;
pub use card_property::CardProperty;
pub use card_state::CardState;
pub use choice::Choice;
pub use choice_reference::ChoiceReference;
pub use coin_flip::CoinFlip;
pub use control_flow::ControlFlow;
pub use count_specifier::CountSpecifier;
pub use creature_grouping::CreatureGrouping;
pub use day_night::DayNight;
pub use die_roll::DieRoll;
pub use direction::Direction;
pub use english_keywords::EnglishKeyword;
pub use global_zone::GlobalZone;
pub use in_addition_to_paying_its_other_costs::InAdditionToPayingItsOtherCost;
pub use keyword_ability::KeywordAbility;
pub use keyword_action::KeywordAction;
pub use may_choose_the_same_mode::MayChooseTheSameModeMoreThanOnce;
pub use non_kind::NonKind;
pub use not_of_a_kind::NotOfAKind;
pub use number::Number;
pub use number_of_times::NumberOfTimes;
pub use number_operation::NumberOperation;
pub use partner_kind::PartnerKind;
pub use player_action::TensedPlayerAction;
pub use player_designation::PlayerDesignation;
pub use player_properties::PlayerProperties;
pub use player_specifier::PlayerSpecifier;
pub use plus_minus::PowerToughnessModElements;
pub use special_costs::SpecialCost;
pub use tap_untap_cost::TapUntapCost;
pub use the_same_is_true_for::TheSameIsTrueFor;
pub use under_control::UnderControl;
pub use win_lose_clauses::WinLoseClause;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VhyToSortLater {
    AnyTime {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NextTime {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Life {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Source {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Card {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FlipACoin {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Cost {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ActivationCost {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Permanent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Player {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Spell {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Modal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Turn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Mana {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Ability {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TriggeredAbility {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ActivatedAbility {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FinalChapterAbility {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LoyaltyAbility {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Effect {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChaosEnsue {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Triggers {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Devotion {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StartingWithYou {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RoundedUp {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RoundedDown {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    InAnyCombinationOfColors {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Step {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Phase {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Unspent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Perpetually {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Team {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Teammate {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PlanarDeck {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FollowedBy {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LegendRule {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PlayingArea {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Playing {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Ante {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AtTheBeginningOfTheGame {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ManaSymbol {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Mode {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheGame {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Continuously {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BoosterPack {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    UnopenedBoosterPack {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ExtraTurn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    WorthOfModes {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DraftRound {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Including {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    OneTimeBoon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheLastTime {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StartTheGame {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheSameWay {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TurnOrder {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AsPartOf {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SoOn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Choice {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Affect {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HeightOfAtLeastOneFoot {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LethalDamage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    WithThoseCharacteristics {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheStack {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    JustBeneath {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Marked {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Door {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    UnlockedDoor {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CardPool {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GenericManaCost {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Received {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ColorPair {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Multiple {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PhyrexianSymbol {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChangedTo {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MagicSubgame {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Previously {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheValueOf {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Radiation {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Immediatly {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Heal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Legal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Illegal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    OriginalSpell {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Determined {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Label {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Circled {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Unchanged {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Order {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PointOfBushido {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl VhyToSortLater {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AnyTime { span } => *span,
            Self::NextTime { span } => *span,
            Self::Life { span } => *span,
            Self::Source { span } => *span,
            Self::Card { span } => *span,
            Self::FlipACoin { span } => *span,
            Self::Cost { span } => *span,
            Self::ActivationCost { span } => *span,
            Self::Permanent { span } => *span,
            Self::Player { span } => *span,
            Self::Spell { span } => *span,
            Self::Modal { span } => *span,
            Self::Turn { span } => *span,
            Self::Mana { span } => *span,
            Self::Ability { span } => *span,
            Self::TriggeredAbility { span } => *span,
            Self::ActivatedAbility { span } => *span,
            Self::FinalChapterAbility { span } => *span,
            Self::LoyaltyAbility { span } => *span,
            Self::Effect { span } => *span,
            Self::ChaosEnsue { span } => *span,
            Self::Triggers { span } => *span,
            Self::Devotion { span } => *span,
            Self::StartingWithYou { span } => *span,
            Self::RoundedUp { span } => *span,
            Self::RoundedDown { span } => *span,
            Self::InAnyCombinationOfColors { span } => *span,
            Self::Step { span } => *span,
            Self::Phase { span } => *span,
            Self::Unspent { span } => *span,
            Self::Perpetually { span } => *span,
            Self::Team { span } => *span,
            Self::Teammate { span } => *span,
            Self::PlanarDeck { span } => *span,
            Self::FollowedBy { span } => *span,
            Self::LegendRule { span } => *span,
            Self::PlayingArea { span } => *span,
            Self::Playing { span } => *span,
            Self::Ante { span } => *span,
            Self::AtTheBeginningOfTheGame { span } => *span,
            Self::ManaSymbol { span } => *span,
            Self::Mode { span } => *span,
            Self::TheGame { span } => *span,
            Self::Continuously { span } => *span,
            Self::BoosterPack { span } => *span,
            Self::UnopenedBoosterPack { span } => *span,
            Self::ExtraTurn { span } => *span,
            Self::WorthOfModes { span } => *span,
            Self::DraftRound { span } => *span,
            Self::Including { span } => *span,
            Self::OneTimeBoon { span } => *span,
            Self::TheLastTime { span } => *span,
            Self::StartTheGame { span } => *span,
            Self::TheSameWay { span } => *span,
            Self::TurnOrder { span } => *span,
            Self::AsPartOf { span } => *span,
            Self::SoOn { span } => *span,
            Self::Choice { span } => *span,
            Self::Affect { span } => *span,
            Self::HeightOfAtLeastOneFoot { span } => *span,
            Self::LethalDamage { span } => *span,
            Self::WithThoseCharacteristics { span } => *span,
            Self::TheStack { span } => *span,
            Self::JustBeneath { span } => *span,
            Self::Marked { span } => *span,
            Self::Door { span } => *span,
            Self::UnlockedDoor { span } => *span,
            Self::CardPool { span } => *span,
            Self::GenericManaCost { span } => *span,
            Self::Received { span } => *span,
            Self::ColorPair { span } => *span,
            Self::Multiple { span } => *span,
            Self::PhyrexianSymbol { span } => *span,
            Self::ChangedTo { span } => *span,
            Self::MagicSubgame { span } => *span,
            Self::Previously { span } => *span,
            Self::TheValueOf { span } => *span,
            Self::Radiation { span } => *span,
            Self::Immediatly { span } => *span,
            Self::Heal { span } => *span,
            Self::Legal { span } => *span,
            Self::Illegal { span } => *span,
            Self::OriginalSpell { span } => *span,
            Self::Determined { span } => *span,
            Self::Label { span } => *span,
            Self::Circled { span } => *span,
            Self::Unchanged { span } => *span,
            Self::Order { span } => *span,
            Self::PointOfBushido { span } => *span,
        }
    }
}

impl VhyToSortLater {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "any time" => Some(Self::AnyTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the next time" => Some(Self::NextTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ability" | "abilities" => Some(Self::Ability {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "triggered ability" | "triggered abilities" => Some(Self::TriggeredAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "activated ability" | "activated abilities" => Some(Self::ActivatedAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "final chapter ability" | "final chapter abilities" => Some(Self::FinalChapterAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "loyalty ability" => Some(Self::LoyaltyAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "card" | "cards" => Some(Self::Card {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "flip a coin" => Some(Self::FlipACoin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cost" | "costs" => Some(Self::Cost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "activation cost" | "activation costs" => Some(Self::ActivationCost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "effect" | "effects" => Some(Self::Effect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "life" => Some(Self::Life {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana" => Some(Self::Mana {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "permanent" | "permanents" => Some(Self::Permanent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "player" => Some(Self::Player {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "spell" | "spells" => Some(Self::Spell {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "modal" => Some(Self::Modal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "source" | "sources" => Some(Self::Source {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            /* Fixme: what a mess */
            "turn" | "turns" | "turned" | "turning" | "most recent turn" => Some(Self::Turn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chaos ensue" | "chaos ensues" => Some(Self::ChaosEnsue {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "trigger" | "triggers" => Some(Self::Triggers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "devotion" => Some(Self::Devotion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting with you" => Some(Self::StartingWithYou {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rounded up" => Some(Self::RoundedUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rounded down" => Some(Self::RoundedDown {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "in any combination of colors" => Some(Self::InAnyCombinationOfColors {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "step" | "steps" => Some(Self::Step {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phase" | "phases" => Some(Self::Phase {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unspent" => Some(Self::Unspent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "perpetually" => Some(Self::Perpetually {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "team" => Some(Self::Team {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "teammate" | "teammates" => Some(Self::Teammate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "planar deck" => Some(Self::PlanarDeck {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "followed by" => Some(Self::FollowedBy {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legend rule" => Some(Self::LegendRule {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "playing area" => Some(Self::PlayingArea {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "playing" => Some(Self::Playing {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ante" => Some(Self::Ante {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "at the beginning of the game" => Some(Self::AtTheBeginningOfTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana symbol" | "mana symbols" => Some(Self::ManaSymbol {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mode" => Some(Self::Mode {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the game" => Some(Self::TheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "continuously" => Some(Self::Continuously {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "booster pack" | "booster packs" => Some(Self::BoosterPack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unopened booster pack" => Some(Self::UnopenedBoosterPack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "extra turn" | "extra turns" => Some(Self::ExtraTurn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "worth of modes" => Some(Self::WorthOfModes {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "draft round" => Some(Self::DraftRound {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "including" => Some(Self::Including {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "one-time boon" => Some(Self::OneTimeBoon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the last time" => Some(Self::TheLastTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "start the game" | "started the game" => Some(Self::StartTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the same way" => Some(Self::TheSameWay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turn order" => Some(Self::TurnOrder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as part of" => Some(Self::AsPartOf {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "so on" => Some(Self::SoOn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "choice" | "choices" => Some(Self::Choice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "affect" => Some(Self::Affect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "height of at least one foot" => Some(Self::HeightOfAtLeastOneFoot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lethal damage" => Some(Self::LethalDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "with those characteristics" => Some(Self::WithThoseCharacteristics {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the stack" => Some(Self::TheStack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "just beneath" => Some(Self::JustBeneath {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "marked" => Some(Self::Marked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "door" => Some(Self::Door {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unlocked door" | "unlocked doors" => Some(Self::UnlockedDoor {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "card pool" => Some(Self::CardPool {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "generic mana costs" => Some(Self::GenericManaCost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "received" => Some(Self::Received {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "color pair" | "color pairs" => Some(Self::ColorPair {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "multiple" => Some(Self::Multiple {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "{h}" => Some(Self::PhyrexianSymbol {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "changed to" => Some(Self::ChangedTo {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "magic subgame" | "subgame" => Some(Self::MagicSubgame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "previously" => Some(Self::Previously {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the value of" => Some(Self::TheValueOf {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "radiation" => Some(Self::Radiation {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "healed" => Some(Self::Heal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legal" => Some(Self::Legal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "illegal" => Some(Self::Illegal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "original spell" => Some(Self::OriginalSpell {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "determined" => Some(Self::Determined {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "label" => Some(Self::Label {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "circled" => Some(Self::Circled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unchanged" => Some(Self::Unchanged {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "order" => Some(Self::Order {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "point of bushido" => Some(Self::PointOfBushido {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

impl std::fmt::Display for VhyToSortLater {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
