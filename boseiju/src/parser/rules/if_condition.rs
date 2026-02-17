use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [super::ParserRule {
        from: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)).id(),
            ParserNode::Event { event: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ThisTurn).id(),
        ]),
        result: ParserNode::IfCondition { condition: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)),
                ParserNode::Event { event },
                ParserNode::LexerToken(TokenKind::ThisTurn),
            ] => Some(ParserNode::IfCondition {
                condition: crate::ability_tree::if_condition::IfCondition::EventOccured {
                    timeframe: (),
                    event: event.clone(),
                },
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
