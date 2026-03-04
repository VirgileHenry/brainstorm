use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [/* Exile follow up that return the card directly */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Return {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::PreviouslyMentionnedObject { object: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::ZoneReference { zone: dummy() }.id(),
        ]),
        merged: ParserNode::ExileFollowUp { follow_up: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Return { .. })),
                ParserNode::PreviouslyMentionnedObject { object },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To { .. })),
                ParserNode::ZoneReference { zone },
            ] => Err("unimplemented"),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
