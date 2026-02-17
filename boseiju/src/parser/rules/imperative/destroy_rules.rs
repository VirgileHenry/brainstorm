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
    [/* Destroy any object reference */ ParserRule {
        from: RuleLhs::new(&[
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
                imperative: crate::ability_tree::imperative::Imperative::Destroy(
                    crate::ability_tree::imperative::DestroyImperative {
                        object: reference.clone(),
                    },
                ),
            }),
            _ => None,
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
