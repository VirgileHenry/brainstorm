use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "land" is the basic land kind */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CardType(terminals::CardType {
            card_type: mtg_data::CardType::Land,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }))
        .id()]),
        merged: ParserNode::LandKind { land: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::CardType(terminals::CardType {
                    card_type: mtg_data::CardType::Land,
                    #[cfg(feature = "spanned_tree")]
                    span,
                })),
            ] => Ok(ParserNode::LandKind {
                land: object::kind::LandKind::Land {
                    #[cfg(feature = "spanned_tree")]
                    span: *span,
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
