use crate::ability_tree;

/// Since this can carry entire ability trees, we need to box the biggest variants.
/// Otherwise, this can easily blow up the stack when attempting to store multiple of them.
/// Current size is 112 bytes, let's try to keep it around here ?
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParserNode {
    LexerToken(crate::lexer::tokens::TokenKind),
    Ability {
        ability: Box<ability_tree::ability::Ability>,
    },
    AbilityTree {
        tree: Box<ability_tree::AbilityTree>,
    },
    CharacteristicDefiningAbility {
        ability: ability_tree::ability::statik::charasteristic_defining_ability::CharacteristicDefiningAbility,
    },
    ContinuousEffect {
        effect: ability_tree::ability::statik::continuous_effect::ContinuousEffect,
    },
    ContinuousEffectKind {
        kind: ability_tree::ability::statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind,
    },
    Cost {
        inner: ability_tree::cost::Cost,
    },
    CostModification {
        cost_modification: ability_tree::ability::statik::cost_modification_effect::CostModification,
    },
    CostModificationEffect {
        cost_modification: ability_tree::ability::statik::cost_modification_effect::CostModificationEffect,
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
        source: ability_tree::event::replacement::source_ref::EventSourceReference,
    },
    IfCondition {
        condition: ability_tree::if_condition::IfCondition,
    },
    Imperative {
        imperative: ability_tree::imperative::Imperative,
    },
    ManaCost {
        mana_cost: ability_tree::terminals::ManaCost,
    },
    Number {
        number: ability_tree::number::Number,
    },
    ObjectKind {
        kind: ability_tree::object::ObjectKind,
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
    ReplacedCounterKind {
        kind: ability_tree::event::replacement::counter_on_permanent::ReplacedCounterKind,
    },
    ReplacedPermanentKind {
        kind: ability_tree::event::replacement::counter_on_permanent::ReplacedPermanentKind,
    },
    ReplacedTokenKind {
        kind: ability_tree::event::replacement::token_creation::ReplacedTokenKind,
    },
    Statement {
        statement: ability_tree::statement::Statement,
    },
    TriggerCondition {
        condition: ability_tree::ability::triggered::trigger_cond::TriggerCondition,
    },
    Zone {
        zone: ability_tree::zone::ZoneReference,
    },
}

impl<'src> From<crate::lexer::tokens::Token<'src>> for ParserNode {
    fn from(token: crate::lexer::tokens::Token<'src>) -> Self {
        ParserNode::LexerToken(token.kind)
    }
}

/// Trait to create a meaningless instance of an object.
/// Since all parser nodes need to be constructible for id(), this allows to fill in their fields.
pub trait DummyInit {
    fn dummy_init() -> Self;
}

impl<T: DummyInit> DummyInit for Box<T> {
    fn dummy_init() -> Self {
        Box::new(T::dummy_init())
    }
}

impl<T: DummyInit> DummyInit for Vec<T> {
    fn dummy_init() -> Self {
        Vec::with_capacity(0)
    }
}

impl<T: DummyInit, const N: usize> DummyInit for arrayvec::ArrayVec<T, N> {
    fn dummy_init() -> Self {
        Self::new()
    }
}

impl DummyInit for mtg_data::KeywordAbility {
    fn dummy_init() -> Self {
        Self::Absorb
    }
}

impl DummyInit for mtg_data::Mana {
    fn dummy_init() -> Self {
        Self::Any { number: 0 }
    }
}

impl DummyInit for mtg_data::Color {
    fn dummy_init() -> Self {
        Self::Colorless
    }
}

impl DummyInit for mtg_data::KeywordAction {
    fn dummy_init() -> Self {
        Self::Abandon
    }
}

impl DummyInit for ability_tree::ability::Ability {
    fn dummy_init() -> Self {
        Self::Keyword(ability_tree::ability::keyword::KeywordAbility::SingleKeyword(
            DummyInit::dummy_init(),
        ))
    }
}

impl DummyInit for ability_tree::AbilityTree {
    fn dummy_init() -> Self {
        Self {
            abilities: arrayvec::ArrayVec::new(),
        }
    }
}

impl DummyInit for ability_tree::ability::statik::charasteristic_defining_ability::CharacteristicDefiningAbility {
    fn dummy_init() -> Self {
        Self::PowerToughnessModifier(DummyInit::dummy_init())
    }
}

impl DummyInit for crate::ability_tree::terminals::PowerToughnessModifier {
    fn dummy_init() -> Self {
        crate::ability_tree::terminals::PowerToughnessModifier::Constant { power: 0, toughness: 0 }
    }
}

impl DummyInit for ability_tree::ability::statik::continuous_effect::ContinuousEffect {
    fn dummy_init() -> Self {
        Self {
            duration: crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfTurn,
            effect: ability_tree::ability::statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind::dummy_init(),
        }
    }
}

impl DummyInit for ability_tree::ability::statik::continuous_effect::continuous_effect_kind::ContinuousEffectKind {
    fn dummy_init() -> Self {
        Self::ObjectGainsAbilies {
            object: DummyInit::dummy_init(),
            abilities: DummyInit::dummy_init(),
        }
    }
}

impl DummyInit for ability_tree::ability::statik::cost_modification_effect::CostModification {
    fn dummy_init() -> Self {
        Self::Set(DummyInit::dummy_init())
    }
}

impl DummyInit for ability_tree::ability::statik::cost_modification_effect::CostModificationEffect {
    fn dummy_init() -> Self {
        Self {
            applies_to: DummyInit::dummy_init(),
            modification: DummyInit::dummy_init(),
            condition: None,
        }
    }
}

impl DummyInit for ability_tree::cost::Cost {
    fn dummy_init() -> Self {
        ability_tree::cost::Cost::ManaCost(DummyInit::dummy_init())
    }
}

impl DummyInit for ability_tree::terminals::ManaCost {
    fn dummy_init() -> Self {
        ability_tree::terminals::ManaCost(DummyInit::dummy_init())
    }
}

impl DummyInit for ability_tree::if_condition::IfCondition {
    fn dummy_init() -> Self {
        Self::EventOccured {
            timeframe: (),
            event: DummyInit::dummy_init(),
        }
    }
}

impl DummyInit for ability_tree::if_condition::condition_timeframe::ConditionTimeframe {
    fn dummy_init() -> Self {
        Self::ThisTurn
    }
}

impl DummyInit for ability_tree::imperative::Imperative {
    fn dummy_init() -> Self {
        Self::Destroy {
            object: DummyInit::dummy_init(),
        }
    }
}

impl DummyInit for ability_tree::object::ObjectKind {
    fn dummy_init() -> Self {
        Self::Card
    }
}

impl DummyInit for ability_tree::object::ObjectReference {
    fn dummy_init() -> Self {
        Self::SelfReferencing
    }
}

impl DummyInit for ability_tree::object::ObjectSpecifier {
    fn dummy_init() -> Self {
        Self::Color(DummyInit::dummy_init())
    }
}

impl DummyInit for ability_tree::object::ObjectSpecifiers {
    fn dummy_init() -> Self {
        Self::Single(DummyInit::dummy_init())
    }
}

impl DummyInit for ability_tree::terminals::PlayerSpecifier {
    fn dummy_init() -> Self {
        Self::You
    }
}

impl DummyInit for ability_tree::statement::Statement {
    fn dummy_init() -> Self {
        Self::Imperative(DummyInit::dummy_init())
    }
}

impl DummyInit for ability_tree::ability::triggered::trigger_cond::TriggerCondition {
    fn dummy_init() -> Self {
        Self::ObjectDoesAction {
            object: DummyInit::dummy_init(),
            action: DummyInit::dummy_init(),
        }
    }
}

impl DummyInit for ability_tree::number::Number {
    fn dummy_init() -> Self {
        Self::Number { num: 0 }
    }
}

impl DummyInit for ability_tree::terminals::CardActions {
    fn dummy_init() -> Self {
        Self::Attacks
    }
}

impl DummyInit for ability_tree::terminals::OwnerSpecifier {
    fn dummy_init() -> Self {
        Self::YouOwn
    }
}

impl DummyInit for ability_tree::zone::Zone {
    fn dummy_init() -> Self {
        Self::Anywhere
    }
}

impl DummyInit for ability_tree::zone::ZoneReference {
    fn dummy_init() -> Self {
        Self::OwnedZone {
            zone: DummyInit::dummy_init(),
            owner: DummyInit::dummy_init(),
        }
    }
}

impl DummyInit for ability_tree::event::Event {
    fn dummy_init() -> Self {
        Self::CreateTokens {
            source: DummyInit::dummy_init(),
            quantity: DummyInit::dummy_init(),
            token_specifiers: None,
        }
    }
}

impl DummyInit for ability_tree::event::replacement::EventReplacement {
    fn dummy_init() -> Self {
        Self::TokenCreationReplacement {
            source_ref: DummyInit::dummy_init(),
            tokens: Vec::with_capacity(0),
        }
    }
}

impl DummyInit for ability_tree::event::replacement::source_ref::EventSourceReference {
    fn dummy_init() -> Self {
        Self::ThatEvent
    }
}

impl DummyInit for ability_tree::event::replacement::token_creation::ReplacedTokenKind {
    fn dummy_init() -> Self {
        Self::PreviouslyMentionnedToken
    }
}

impl DummyInit for ability_tree::event::replacement::counter_on_permanent::ReplacedCounterKind {
    fn dummy_init() -> Self {
        Self::PreviouslyMentionnedCounter
    }
}

impl DummyInit for ability_tree::event::replacement::counter_on_permanent::ReplacedPermanentKind {
    fn dummy_init() -> Self {
        Self::PreviouslyMentionnedPermanent
    }
}

impl DummyInit for ability_tree::event::source::EventSource {
    fn dummy_init() -> Self {
        Self::AnEffect
    }
}
