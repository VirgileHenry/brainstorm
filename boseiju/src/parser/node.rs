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
        ability: ability_tree::charasteristic_defining_ability::CharacteristicDefiningAbility,
    },
    ContinuousEffect {
        effect: ability_tree::continuous_effect::ContinuousEffect,
    },
    ContinuousEffectKind {
        kind: ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind,
    },
    Cost {
        inner: ability_tree::cost::Cost,
    },
    Imperative {
        imperative: ability_tree::imperative::Imperative,
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
    Statement {
        statement: ability_tree::statement::Statement,
    },
    TriggerCondition {
        condition: ability_tree::ability::triggered::trigger_cond::TriggerCondition,
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

impl DummyInit for ability_tree::charasteristic_defining_ability::CharacteristicDefiningAbility {
    fn dummy_init() -> Self {
        Self::PowerToughnessModifier(DummyInit::dummy_init())
    }
}

impl DummyInit for crate::ability_tree::terminals::PowerToughnessModifier {
    fn dummy_init() -> Self {
        crate::ability_tree::terminals::PowerToughnessModifier::Constant { power: 0, toughness: 0 }
    }
}

impl DummyInit for ability_tree::continuous_effect::ContinuousEffect {
    fn dummy_init() -> Self {
        Self {
            duration: crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfTurn,
            effect: ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind::dummy_init(),
        }
    }
}

impl DummyInit for ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind {
    fn dummy_init() -> Self {
        ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind::ObjectGainsAbilies {
            object: DummyInit::dummy_init(),
            abilities: DummyInit::dummy_init(),
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

impl DummyInit for ability_tree::terminals::CardActions {
    fn dummy_init() -> Self {
        Self::Attacks
    }
}
