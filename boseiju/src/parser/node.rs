use crate::ability_tree;

/// Since this can carry entire ability trees, we need to box the biggest variants.
/// Otherwise, this can easily blow up the stack when attempting to store multiple of them.
/// Current size is 112 bytes, let's try to keep it around here ?
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParserNode {
    Ability(Box<ability_tree::ability::Ability>),
    AbilityTree(Box<ability_tree::AbilityTree>),
    CharacteristicDefiningAbility(ability_tree::charasteristic_defining_ability::CharasteristicDefiningAbility),
    ContinuousEffect(ability_tree::continuous_effect::ContinuousEffect),
    ContinuousEffectKind(ability_tree::continuous_effect::continuous_effect_kind::ContinuousEffectKind),
    Cost(ability_tree::cost::Cost),
    Imperative(ability_tree::imperative::Imperative),
    LexerToken(crate::lexer::tokens::TokenKind),
    ObjectKind(ability_tree::object::ObjectKind),
    ObjectReference(ability_tree::object::ObjectReference),
    ObjectSpecifier(ability_tree::object::ObjectSpecifier),
    ObjectSpecifiers(ability_tree::object::ObjectSpecifiers),
    Statement(ability_tree::statement::Statement),
    TriggerCondition(ability_tree::ability::triggered::trigger_cond::TriggerCondition),
}

impl<'src> From<crate::lexer::tokens::Token<'src>> for ParserNode {
    fn from(token: crate::lexer::tokens::Token<'src>) -> Self {
        ParserNode::LexerToken(token.kind)
    }
}
