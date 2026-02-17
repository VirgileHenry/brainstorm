use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [/* Return any object from a zone to another */ ParserRule {
        from: RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Return)).id(),
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::From)).id(),
            ParserNode::Zone { zone: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)).id(),
            ParserNode::Zone { zone: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
        ]),
        result: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Return)),
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::From)),
                ParserNode::Zone { zone: from_zone },
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)),
                ParserNode::Zone { zone: to_zone },
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
            ] => Some(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::Return(
                    crate::ability_tree::imperative::ReturnImperative {
                        object: reference.clone(),
                        from: from_zone.clone(),
                        to: to_zone.clone(),
                    },
                ),
            }),
            _ => None,
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
