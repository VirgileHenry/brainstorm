use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    DummyInit::dummy_init()
}

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
    [
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Destroy)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
            ]),
            result: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Destroy)),
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                ] => Some(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::Destroy {
                        object: reference.clone(),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            from: super::RuleLhs::new(&[
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
                    imperative: crate::ability_tree::imperative::Imperative::Return {
                        object: reference.clone(),
                        from: from_zone.clone(),
                        to: to_zone.clone(),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
