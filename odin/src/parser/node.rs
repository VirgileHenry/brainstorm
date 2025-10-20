#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParserNode {
    Ability(crate::ability_tree::ability::Ability),
    AbilityTree(crate::ability_tree::AbilityTree),
    Imperative(crate::ability_tree::imperative::Imperative),
    LexerToken(crate::lexer::tokens::TokenKind),
    ObjectKind(crate::ability_tree::object::ObjectKind),
    ObjectReference(crate::ability_tree::object::ObjectReference),
    ObjectSpecifier(crate::ability_tree::object::ObjectSpecifier),
    ObjectSpecifiers(crate::ability_tree::object::ObjectSpecifiers),
    Statement(crate::ability_tree::statement::Statement),
    TriggerCondition(crate::ability_tree::ability::triggered::trigger_cond::TriggerCondition),
}

impl<'src> From<crate::lexer::tokens::Token<'src>> for ParserNode {
    fn from(token: crate::lexer::tokens::Token<'src>) -> Self {
        ParserNode::LexerToken(token.kind)
    }
}
