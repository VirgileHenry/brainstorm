use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [/* Exile follow up that return the card directly */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Return)).id(),
            ParserNode::PreviouslyMentionnedObject { object: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)).id(),
            ParserNode::ZoneReference { zone: dummy() }.id(),
        ]),
        merged: ParserNode::ExileFollowUp { follow_up: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Return)),
                ParserNode::PreviouslyMentionnedObject { object },
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)),
                ParserNode::ZoneReference { zone },
            ] => None,
            _ => None,
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
