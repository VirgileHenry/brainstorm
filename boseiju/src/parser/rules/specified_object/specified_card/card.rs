use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "card" makes a specified card  */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Card {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::SpecifiedCard { card: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Card {
                    #[cfg(feature = "spanned_tree")]
                        span: creature_span,
                })),
            ] => Ok(ParserNode::SpecifiedCard {
                card: object::specified_object::SpecifiedCard {
                    specifiers: None,
                    #[cfg(feature = "spanned_tree")]
                    span: *creature_span,
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
