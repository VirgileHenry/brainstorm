use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [/* Return any object from a zone to another */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Return {
                span: Default::default(),
            }))
            .id(),
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From {
                span: Default::default(),
            }))
            .id(),
            ParserNode::ZoneReference { zone: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To {
                span: Default::default(),
            }))
            .id(),
            ParserNode::ZoneReference { zone: dummy() }.id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Return { span: start_span })),
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From { .. })),
                ParserNode::ZoneReference { zone: from_zone },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To { .. })),
                ParserNode::ZoneReference { zone: to_zone },
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::Return(
                    crate::ability_tree::imperative::ReturnImperative {
                        object: reference.clone(),
                        from: from_zone.clone(),
                        to: to_zone.clone(),
                        span: start_span.merge(&to_zone.span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
