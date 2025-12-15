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
        let kind = TokenKind::try_from_str(span.text)?;
        Some(Self { kind, span })
    }

    pub const TOKEN_COUNT: usize = 0;
    pub fn token_id(&self) -> usize {
        TokenKind::COUNT
    }
}

macro_rules! token_id_recursive_expension {
    (
        offset = $offset:expr,
        out = { $($out:tt)* },
        rest =
    ) => {
        pub const fn id(&self) -> usize {
            match self { $($out)* }
        }
    };
    (
        offset = $offset:expr,
        out = { $($out:tt)* },
        rest = $variant:ident, $ty:path; $($rest:tt)*
    ) => {
        token_id_recursive_expension!(
            offset = $offset + <$ty>::COUNT,
            out = {
                $($out)*
                Self::$variant(value) => $offset + value.id(),
            },
            rest = $($rest)*
        );
    };
}

macro_rules! create_token_kind {
    ($($variant:tt, $ty:path;)+) => {
        /* Create the TokenKind enum */
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum TokenKind {
            $(
                $variant ( $ty ),
            )+
        }

        /* Implemnt Functions for the Token Kind */
        impl TokenKind {
            /* The number of tokens is the sum of the count for all variant */
            pub const COUNT: usize = { 0 $( + < $ty >::COUNT )+ };
            /* Get the id with the offset from all preceeding variants, and id from current variant */
            token_id_recursive_expension!(offset = 0, out = {}, rest = $($variant, $ty;)+);
            /* Try parse for all variants */
            pub fn try_from_str(source: &str) -> Option<Self>{
                $(
                    if let Some(result) = < $ty >::try_from_str(source) {
                        return Some(Self::$variant(result));
                    }
                )+
                None
            }
        }
    };
}

create_token_kind!(
    Number, terminals::Number;
    Counter, terminals::Counter;
    CountSpecifier, terminals::CountSpecifier;
    ControlSpecifier, terminals::ControlSpecifier;
    OwnerSpecifier, terminals::OwnerSpecifier;
    Order, terminals::Order;
    Appartenance, terminals::Appartenance;
    CardActions, terminals::CardActions;
    PlayerSpecifier, terminals::PlayerSpecifier;
    PermanentState, terminals::PermanentState;
    PermanentProperty, terminals::PermanentProperty;
    SpellProperty, terminals::SpellProperty;
    Phase, terminals::Phase;
    Step, terminals::Step;
    PowerToughness, terminals::PowerToughness;
    PowerToughnessModifier, terminals::PowerToughnessModifier;
    PlaneswalkerAbilityCost, terminals::PlaneswalkerAbilityCost;
    SagaChapterNumber, terminals::SagaChapterNumber;
    ContinuousEffectDuration, terminals::ContinuousEffectDuration;
    NamedToken, terminals::NamedToken;
    Zone, zone::Zone;
    Color, mtg_data::Color;
    AbilityWord, mtg_data::AbilityWord;
    KeywordAbility, mtg_data::KeywordAbility;
    KeywordAction, mtg_data::KeywordAction;
    Mana, mtg_data::Mana;
    ObjectKind, object::ObjectKind;
    ControlFlow, non_terminals::ControlFlow;
    TapUntapCost, non_terminals::TapUntapCost;
    EnglishKeyword, non_terminals::EnglishKeyword;
    SelfReferencing, non_terminals::SelfReferencing;
    NumberReference, non_terminals::NumberReference;
    NotOfAKind, non_terminals::NotOfAKind;
    ActionKeyword, non_terminals::ActionKeyword;
    DamageKind, non_terminals::DamageKind;
    PlayerAction, non_terminals::PlayerAction;
    ThisTurn, non_terminals::ThisTurn;
    NonKind, non_terminals::NonKind;
    AmountReplacement, non_terminals::AmountReplacement;
    UnderControl, non_terminals::UnderControl;
    IntoHand, non_terminals::IntoHand;
    PlayerProperties, non_terminals::PlayerProperties;
    IndirectReference, non_terminals::IndirectReference;
    VhyToSortLater, non_terminals::VhyToSortLater;
);
