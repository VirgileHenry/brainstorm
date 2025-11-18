pub mod non_terminals;

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

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "number: {num}"),
            Self::Counter(counter) => write!(f, "counter: {counter}"),
            Self::CountSpecifier(count) => write!(f, "count specifier: {count}"),
            Self::ControlSpecifier(control) => write!(f, "control specifier: {control}"),
            Self::OwnerSpecifier(owner) => write!(f, "owner specifier: {owner}"),
            Self::Order(order) => write!(f, "order: {order}"),
            Self::Appartenance(appartenance) => write!(f, "appartenance: {appartenance}"),
            Self::CardActions(action) => write!(f, "card action: {action}"),
            Self::PlayerSpecifier(player) => write!(f, "player specifier: {player}"),
            Self::PermanentState(state) => write!(f, "permanent state: {state}"),
            Self::PermanentProperty(property) => write!(f, "permanent property: {property}"),
            Self::Phase(phase) => write!(f, "phase: {phase}"),
            Self::Step(step) => write!(f, "step: {step}"),
            Self::SpellProperty(prop) => write!(f, "spell property: {prop}"),
            Self::PowerToughness(pt) => write!(f, "power toughness: {pt}"),
            Self::PowerToughnessModifier(modifier) => write!(f, "power toughness modifier: {modifier}"),
            Self::PlaneswalkerAbilityCost(cost) => write!(f, "planeswalker ability cost: {cost}"),
            Self::SagaChapterNumber(chapter) => write!(f, "saga chapter number: {chapter}"),
            Self::ContinuousEffectDuration(duration) => write!(f, "continuous effect duration: {duration}"),
            Self::Zone(zone) => write!(f, "zone: {zone}"),
            Self::Color(color) => write!(f, "color: {color}"),
            Self::KeywordAbility(ability) => write!(f, "keyword ability: {ability}"),
            Self::Mana(mana) => write!(f, "mana: {mana}"),
            Self::CardType(card_type) => write!(f, "card type: {card_type}"),
            Self::CreatureType(creature_type) => write!(f, "creature type: {creature_type}"),
            Self::EnchantmentType(enchantment_type) => write!(f, "enchantment type: {enchantment_type}"),
            Self::LandType(land_type) => write!(f, "land type: {land_type}"),
            Self::PlaneswalkerType(planeswalker_type) => write!(f, "planeswalker type: {planeswalker_type}"),
            Self::BattleType(battle_type) => write!(f, "battle type: {battle_type}"),
            Self::ArtifactType(artifact_type) => write!(f, "artifact type: {artifact_type}"),
            Self::SpellType(spell_type) => write!(f, "spell type: {spell_type}"),
            Self::Supertype(supertype) => write!(f, "supertype: {supertype}"),
            Self::ControlFlow(flow) => write!(f, "control flow: {flow}"),
            Self::TapUntapCost(cost) => write!(f, "tap/untap cost: {cost}"),
            Self::EnglishKeywords(keywords) => write!(f, "english keywords: {keywords}"),
            Self::SelfReferencing(ref_self) => write!(f, "self referencing: {ref_self}"),
            Self::NumberReference(num_ref) => write!(f, "number reference: {num_ref}"),
            Self::NotOfAKind(not_kind) => write!(f, "not of a kind: {not_kind}"),
            Self::ActionKeywords(action_keywords) => write!(f, "action keywords: {action_keywords}"),
            Self::DamageKind(damage) => write!(f, "damage kind: {damage}"),
            Self::PlayerActions(player_actions) => write!(f, "player actions: {player_actions}"),
            Self::VhyToSortLater(_) => write!(f, "todo"),
        }
    }
}

macro_rules! impl_into_token_kind {
    ($ty:path, $variant:tt) => {
        impl Into<TokenKind> for $ty {
            fn into(self) -> TokenKind {
                TokenKind::$variant(self)
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
    Zone, zone::Zone;
    Color, mtg_data::Color;
    KeywordAbility, mtg_data::KeywordAbility;
    Mana, mtg_data::Mana;
    CardType, mtg_data::CardType;
    CreatureType, mtg_data::CreatureType;
    EnchantmentType, mtg_data::EnchantmentType;
    LandType, mtg_data::LandType;
    PlaneswalkerType, mtg_data::PlaneswalkerType;
    BattleType, mtg_data::BattleType;
    ArtifactType, mtg_data::ArtifactType;
    SpellType, mtg_data::SpellType;
    Supertype, mtg_data::Supertype;
    ControlFlow, non_terminals::ControlFlow;
    TapUntapCost, non_terminals::TapUntapCost;
    EnglishKeywords, non_terminals::EnglishKeywords;
    SelfReferencing, non_terminals::SelfReferencing;
    NumberReference, non_terminals::NumberReference;
    NotOfAKind, non_terminals::NotOfAKind;
    ActionKeywords, non_terminals::ActionKeywords;
    DamageKind, non_terminals::DamageKind;
    PlayerActions, non_terminals::PlayerActions;
    VhyToSortLater, non_terminals::VhyToSortLater;
);
