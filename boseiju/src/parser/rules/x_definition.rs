use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::ability_tree::AbilityTreeNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "where X is the number of <permanent reference> on the battlefield" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Where {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediates::Number::NumberOf {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::GameStateNumber { number: dummy() }.id(),
            ]),
            merged: ParserNode::XDefinition { definition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Where {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::X { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The { .. })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::NumberOf { .. })),
                    ParserNode::GameStateNumber { number },
                ] => Ok(ParserNode::XDefinition {
                    definition: crate::ability_tree::number::XDefinition::FromGameState(
                        crate::ability_tree::number::XFromGameState {
                            x_value: number.clone(),
                            span: number.node_span().merge(start_span),
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
