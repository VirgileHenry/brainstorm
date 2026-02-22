use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    crate::ability_tree::ability::keyword::all_standalone_kw_abilities().map(|keyword| super::ParserRule {
        from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::KeywordAbility(keyword)).id()]),
        result: ParserNode::Ability { ability: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(TokenKind::KeywordAbility(keyword))] => Some(ParserNode::KeywordAbility {
                ability: crate::ability_tree::ability::keyword::keyword_to_abilities(*keyword)?,
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
}
