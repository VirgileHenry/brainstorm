use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::terminal;
use boseiju_tree::ability_tree::object;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* "land" is the basic land kind */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CardType(terminal::CardType {
            card_type: mtg_data::CardType::Land,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }))
        .id()]),
        merged: ParserNode::LandKind {
            land: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::CardType(terminal::CardType {
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
