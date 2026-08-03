use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* "Gain <number> life" makes a gain life imperative. */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Gain {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number {
                number: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Life {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::ImperativeKind {
            imperative: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                        span: start_span,
                })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Life {
                    #[cfg(feature = "spanned_tree")]
                        span: end_span,
                })),
            ] => Ok(ParserNode::ImperativeKind {
                imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::GainLife(
                    boseiju_tree::ability_tree::imperative::GainLifeImperative {
                        amount: number.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
