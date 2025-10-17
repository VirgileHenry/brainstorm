#[derive(Debug, Clone)]
pub enum ParserNode {
    Ability(crate::ability_tree::ability::Ability),
    AbilityTree(crate::ability_tree::AbilityTree),
    CountSpecifier(crate::ability_tree::terminals::CountSpecifier),
    Imperative(crate::ability_tree::imperative::Imperative),
    LexerToken(crate::lexer::tokens::TokenKind),
    ObjectKind(crate::ability_tree::object::ObjectKind),
    ObjectReference(crate::ability_tree::object::ObjectReference),
    ObjectSpecifier(crate::ability_tree::object::ObjectSpecifier),
    Statement(crate::ability_tree::statement::Statement),
}

impl<'src> From<crate::lexer::tokens::Token<'src>> for ParserNode {
    fn from(token: crate::lexer::tokens::Token<'src>) -> Self {
        ParserNode::LexerToken(token.kind)
    }
}
