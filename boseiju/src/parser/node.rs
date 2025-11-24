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

impl ParserNode {
    pub const COUNT: usize = crate::lexer::tokens::TokenKind::COUNT + 13;
    pub const fn id(&self) -> usize {
        const LEXER_TOKEN_COUNT: usize = crate::lexer::tokens::TokenKind::COUNT;
        match self {
            /* Special case for the lexer token: use the lexer token id */
            Self::LexerToken(token) => token.id(),

            /* For all others, they count as a single token */
            Self::Ability(_) => LEXER_TOKEN_COUNT + 0,
            Self::AbilityTree(_) => LEXER_TOKEN_COUNT + 1,
            Self::CharacteristicDefiningAbility(_) => LEXER_TOKEN_COUNT + 2,
            Self::ContinuousEffect(_) => LEXER_TOKEN_COUNT + 3,
            Self::ContinuousEffectKind(_) => LEXER_TOKEN_COUNT + 4,
            Self::Cost(_) => LEXER_TOKEN_COUNT + 5,
            Self::Imperative(_) => LEXER_TOKEN_COUNT + 6,
            Self::ObjectKind(_) => LEXER_TOKEN_COUNT + 7,
            Self::ObjectReference(_) => LEXER_TOKEN_COUNT + 8,
            Self::ObjectSpecifier(_) => LEXER_TOKEN_COUNT + 9,
            Self::ObjectSpecifiers(_) => LEXER_TOKEN_COUNT + 10,
            Self::Statement(_) => LEXER_TOKEN_COUNT + 11,
            Self::TriggerCondition(_) => LEXER_TOKEN_COUNT + 12,
        }
    }
}
