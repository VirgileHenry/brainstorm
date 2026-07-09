//! Intermediates token have no meaning on their own,
//! they exist to create more complex ability structures.

mod ability_word;
mod action_keywords;
mod ambiguous_tokens;
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
mod keyword_ability;
mod keyword_action;
mod non_kind;
mod number;
mod number_operation;
mod partner_kind;
mod player_action;
mod player_designation;
mod player_properties;
mod player_specifier;
mod special_costs;
mod tap_untap_cost;
mod win_lose_clauses;

pub use ability_word::AbilityWord;
pub use action_keywords::TensedActionKeyword;
pub use ambiguous_tokens::AmbiguousToken;
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
pub use keyword_ability::KeywordAbility;
pub use keyword_action::TensedKeywordAction;
pub use non_kind::NonKind;
pub use number::Number;
pub use number_operation::NumberOperation;
pub use partner_kind::PartnerKind;
pub use player_action::TensedPlayerAction;
pub use player_designation::PlayerDesignation;
pub use player_properties::PlayerProperties;
pub use player_specifier::PlayerSpecifier;
pub use special_costs::SpecialCost;
pub use tap_untap_cost::TapUntapCost;
pub use win_lose_clauses::WinLoseClause;

#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VhyToSortLater {
    AnyTime {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NextTime {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Life {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Source {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Card {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FlipACoin {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Cost {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ActivationCost {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Permanent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Player {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Spell {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Modal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Turn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Mana {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Ability {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TriggeredAbility {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ActivatedAbility {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FinalChapterAbility {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LoyaltyAbility {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Effect {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ChaosEnsue {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Triggers {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Devotion {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartingWithYou {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RoundedUp {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RoundedDown {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    InAnyCombinationOfColors {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Step {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Phase {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unspent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Perpetually {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Team {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Teammate {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PlanarDeck {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FollowedBy {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LegendRule {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PlayingArea {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Playing {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Ante {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AtTheBeginningOfTheGame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ManaSymbol {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Mode {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheGame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Continuously {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BoosterPack {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    UnopenedBoosterPack {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ExtraTurn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    WorthOfModes {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    DraftRound {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Including {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OneTimeBoon {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheLastTime {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartTheGame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheSameWay {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TurnOrder {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AsPartOf {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    SoOn {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Choice {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Affect {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    HeightOfAtLeastOneFoot {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LethalDamage {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    WithThoseCharacteristics {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheStack {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    JustBeneath {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Marked {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Door {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    UnlockedDoor {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    CardPool {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    GenericManaCost {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Received {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ColorPair {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Multiple {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PhyrexianSymbol {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ChangedTo {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MagicSubgame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Previously {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheValueOf {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Radiation {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Immediatly {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Heal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Legal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Illegal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OriginalSpell {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Determined {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Label {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Circled {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unchanged {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Order {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PointOfBushido {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    True {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for VhyToSortLater {
    fn span(&self) -> boseiju_span::Span {
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
            Self::True { span } => *span,
        }
    }
}

impl<'src> TryFrom<&crate::LexerSpan<'src>> for VhyToSortLater {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "any time" => Ok(Self::AnyTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the next time" => Ok(Self::NextTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ability" | "abilities" => Ok(Self::Ability {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "triggered ability" | "triggered abilities" => Ok(Self::TriggeredAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "activated ability" | "activated abilities" => Ok(Self::ActivatedAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "final chapter ability" | "final chapter abilities" => Ok(Self::FinalChapterAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "loyalty ability" => Ok(Self::LoyaltyAbility {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "card" | "cards" => Ok(Self::Card {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "flip a coin" => Ok(Self::FlipACoin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cost" | "costs" => Ok(Self::Cost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "activation cost" | "activation costs" => Ok(Self::ActivationCost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "effect" | "effects" => Ok(Self::Effect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "life" => Ok(Self::Life {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana" => Ok(Self::Mana {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "permanent" | "permanents" => Ok(Self::Permanent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "player" => Ok(Self::Player {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "spell" | "spells" => Ok(Self::Spell {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "modal" => Ok(Self::Modal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "source" | "sources" => Ok(Self::Source {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            /* Fixme: what a mess */
            "turn" | "turns" | "turned" | "turning" | "most recent turn" => Ok(Self::Turn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chaos ensue" | "chaos ensues" => Ok(Self::ChaosEnsue {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "trigger" | "triggers" => Ok(Self::Triggers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "devotion" => Ok(Self::Devotion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting with you" => Ok(Self::StartingWithYou {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rounded up" => Ok(Self::RoundedUp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rounded down" => Ok(Self::RoundedDown {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "in any combination of colors" => Ok(Self::InAnyCombinationOfColors {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "step" | "steps" => Ok(Self::Step {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phase" | "phases" => Ok(Self::Phase {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unspent" => Ok(Self::Unspent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "perpetually" => Ok(Self::Perpetually {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "team" => Ok(Self::Team {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "teammate" | "teammates" => Ok(Self::Teammate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "planar deck" => Ok(Self::PlanarDeck {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "followed by" => Ok(Self::FollowedBy {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legend rule" => Ok(Self::LegendRule {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "playing area" => Ok(Self::PlayingArea {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "playing" => Ok(Self::Playing {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ante" => Ok(Self::Ante {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "at the beginning of the game" => Ok(Self::AtTheBeginningOfTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mana symbol" | "mana symbols" => Ok(Self::ManaSymbol {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mode" => Ok(Self::Mode {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the game" => Ok(Self::TheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "continuously" => Ok(Self::Continuously {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "booster pack" | "booster packs" => Ok(Self::BoosterPack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unopened booster pack" => Ok(Self::UnopenedBoosterPack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "extra turn" | "extra turns" => Ok(Self::ExtraTurn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "worth of modes" => Ok(Self::WorthOfModes {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "draft round" => Ok(Self::DraftRound {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "including" => Ok(Self::Including {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "one-time boon" => Ok(Self::OneTimeBoon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the last time" => Ok(Self::TheLastTime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "start the game" | "started the game" => Ok(Self::StartTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the same way" => Ok(Self::TheSameWay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "turn order" => Ok(Self::TurnOrder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "as part of" => Ok(Self::AsPartOf {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "so on" => Ok(Self::SoOn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "choice" | "choices" => Ok(Self::Choice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "affect" => Ok(Self::Affect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "height of at least one foot" => Ok(Self::HeightOfAtLeastOneFoot {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lethal damage" => Ok(Self::LethalDamage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "with those characteristics" => Ok(Self::WithThoseCharacteristics {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the stack" => Ok(Self::TheStack {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "just beneath" => Ok(Self::JustBeneath {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "marked" => Ok(Self::Marked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "door" => Ok(Self::Door {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unlocked door" | "unlocked doors" => Ok(Self::UnlockedDoor {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "card pool" => Ok(Self::CardPool {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "generic mana costs" => Ok(Self::GenericManaCost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "received" => Ok(Self::Received {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "color pair" | "color pairs" => Ok(Self::ColorPair {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "multiple" => Ok(Self::Multiple {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "{h}" => Ok(Self::PhyrexianSymbol {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "changed to" => Ok(Self::ChangedTo {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "magic subgame" | "subgame" => Ok(Self::MagicSubgame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "previously" => Ok(Self::Previously {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the value of" => Ok(Self::TheValueOf {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "radiation" => Ok(Self::Radiation {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "healed" => Ok(Self::Heal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "legal" => Ok(Self::Legal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "illegal" => Ok(Self::Illegal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "original spell" => Ok(Self::OriginalSpell {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "determined" => Ok(Self::Determined {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "label" => Ok(Self::Label {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "circled" => Ok(Self::Circled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unchanged" => Ok(Self::Unchanged {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "order" => Ok(Self::Order {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "point of bushido" => Ok(Self::PointOfBushido {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "true" => Ok(Self::True {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for VhyToSortLater {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
