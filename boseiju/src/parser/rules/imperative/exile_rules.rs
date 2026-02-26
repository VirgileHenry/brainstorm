use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Exile object reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Exile)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Exile)),
                    ParserNode::ObjectReference { reference },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::Exile(
                        crate::ability_tree::imperative::ExileImperative {
                            object: reference.clone(),
                            follow_up: None,
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Exile object reference, with a follow up on the exile thing: "then return it..." */
        ParserRule {
            expanded: RuleLhs::new(&[
                /* Fixme: exile is a zone here ? */
                ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Exile)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
                ParserNode::ExileFollowUp { follow_up: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Exile)),
                    ParserNode::ObjectReference { reference },
                    ParserNode::ExileFollowUp { follow_up },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::Exile(
                        crate::ability_tree::imperative::ExileImperative {
                            object: reference.clone(),
                            follow_up: Some(follow_up.clone()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
