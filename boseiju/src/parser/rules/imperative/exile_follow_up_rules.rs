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
    [/* Exile follow up that return the card directly */ ParserRule {
        from: RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Return)).id(),
            ParserNode::PreviouslyMentionnedObject { object: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)).id(),
            ParserNode::Zone { zone: dummy() }.id(),
        ]),
        result: ParserNode::ExileFollowUp { follow_up: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::Zone(crate::ability_tree::zone::Zone::Exile)),
                ParserNode::ObjectReference { reference },
            ] => Some(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::Exile(
                    crate::ability_tree::imperative::ExileImperative {
                        object: reference.clone(),
                        follow_up: None,
                    },
                ),
            }),
            _ => None,
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
