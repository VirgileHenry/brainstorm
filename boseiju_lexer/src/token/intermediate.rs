//! Intermediates token have no meaning on their own,
//! they exist to create more complex ability structures.

mod ability_kind;
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
mod coin_flip;
mod control_flow;
mod count_specifier;
mod creature_grouping;
mod day_night;
mod die_roll;
mod direction;
mod english_keywords;
mod format_specific;
mod game_term;
mod global_zone;
mod keyword_ability;
mod keyword_action;
mod legality;
mod misc_object_name;
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
mod uncard_special_term;
mod win_lose_clauses;

pub use ability_kind::AbilityKind;
pub use ability_word::AbilityWord;
pub use action_keywords::ActionKeyword;
pub use action_keywords::TensedActionKeyword;
pub use ambiguous_tokens::AmbiguousToken;
pub use attached_permanent::AttachedObject;
pub use bid::Bid;
pub use card_actions::CardActions;
pub use card_face::CardFace;
pub use card_own_name::CardOwnName;
pub use card_property::CardProperty;
pub use card_state::CardState;
pub use coin_flip::CoinFlip;
pub use control_flow::ControlFlow;
pub use count_specifier::CountSpecifier;
pub use creature_grouping::CreatureGrouping;
pub use day_night::DayNight;
pub use die_roll::DieRoll;
pub use direction::Direction;
pub use english_keywords::EnglishKeyword;
pub use format_specific::FormatSpecific;
pub use game_term::GameTerm;
pub use global_zone::GlobalZone;
pub use keyword_ability::KeywordAbility;
pub use keyword_action::KeywordAction;
pub use keyword_action::TensedKeywordAction;
pub use legality::Legality;
pub use misc_object_name::MiscObjectName;
pub use non_kind::NonKind;
pub use number::Number;
pub use number_operation::NumberOperation;
pub use partner_kind::PartnerKind;
pub use player_action::PlayerAction;
pub use player_action::TensedPlayerAction;
pub use player_designation::PlayerDesignation;
pub use player_properties::PlayerProperties;
pub use player_specifier::PlayerSpecifier;
pub use special_costs::SpecialCost;
pub use tap_untap_cost::TapUntapCost;
pub use uncard_special_term::UncardSpecialTerm;
pub use win_lose_clauses::WinLoseClause;

#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VhyToSortLater {
    ActivationCost {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartingWithYou {
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
    FollowedBy {
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
    Continuously {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    WorthOfModes {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Including {
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
    WithThoseCharacteristics {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    JustBeneath {
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
    Previously {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheValueOf {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Immediatly {
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
            Self::ActivationCost { span } => *span,
            Self::StartingWithYou { span } => *span,
            Self::Unspent { span } => *span,
            Self::Perpetually { span } => *span,
            Self::FollowedBy { span } => *span,
            Self::AtTheBeginningOfTheGame { span } => *span,
            Self::ManaSymbol { span } => *span,
            Self::Mode { span } => *span,
            Self::Continuously { span } => *span,
            Self::WorthOfModes { span } => *span,
            Self::Including { span } => *span,
            Self::StartTheGame { span } => *span,
            Self::TheSameWay { span } => *span,
            Self::AsPartOf { span } => *span,
            Self::SoOn { span } => *span,
            Self::Choice { span } => *span,
            Self::Affect { span } => *span,
            Self::WithThoseCharacteristics { span } => *span,
            Self::JustBeneath { span } => *span,
            Self::Received { span } => *span,
            Self::ColorPair { span } => *span,
            Self::Multiple { span } => *span,
            Self::PhyrexianSymbol { span } => *span,
            Self::ChangedTo { span } => *span,
            Self::Previously { span } => *span,
            Self::TheValueOf { span } => *span,
            Self::Immediatly { span } => *span,
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
            "activation cost" | "activation costs" => Ok(Self::ActivationCost {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "starting with you" => Ok(Self::StartingWithYou {
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
            "followed by" => Ok(Self::FollowedBy {
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
            "continuously" => Ok(Self::Continuously {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "worth of modes" => Ok(Self::WorthOfModes {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "including" => Ok(Self::Including {
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
            "with those characteristics" => Ok(Self::WithThoseCharacteristics {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "just beneath" => Ok(Self::JustBeneath {
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
            "previously" => Ok(Self::Previously {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the value of" => Ok(Self::TheValueOf {
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
