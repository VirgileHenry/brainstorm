use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "where X is the number of <permanent reference> on the battlefield" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::Where {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Be {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediate::Number::NumberOf {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::GameStateNumber {
                    number: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::XDefinition {
                definition: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishWh(intermediate::EnglishWh::Where {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediate::Number::X { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Be { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::The { .. })),
                    ParserNode::LexerToken(Token::Number(intermediate::Number::NumberOf { .. })),
                    ParserNode::GameStateNumber { number },
                ] => Ok(ParserNode::XDefinition {
                    definition: boseiju_tree::ability_tree::number::XDefinition::FromGameState(
                        boseiju_tree::ability_tree::number::x_definition::XFromGameState {
                            x_value: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.span().merge(start_span),
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
