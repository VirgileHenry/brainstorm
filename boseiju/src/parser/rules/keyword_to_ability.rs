use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::parser::node::DummyInit;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    mtg_data::KeywordAbility::all().map(|keyword| super::ParserRule {
        from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::KeywordAbility(keyword)).id()]),
        result: ParserNode::Ability {
            ability: DummyInit::dummy_init(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(TokenKind::KeywordAbility(keyword))] => Some(ParserNode::Ability {
                ability: Box::new(crate::ability_tree::ability::Ability::Keyword(
                    crate::ability_tree::ability::keyword::KeywordAbility::SingleKeyword(*keyword),
                )),
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
}
