use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* "Draw <number> cards" makes a draw card imperative */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::PlayerAction(intermediate::TensedPlayerAction {
                token: intermediate::PlayerAction::Draw {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::BaseForm,
            }))
            .id(),
            ParserNode::Number {
                number: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
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
                ParserNode::LexerToken(Token::PlayerAction(intermediate::TensedPlayerAction {
                    token:
                        intermediate::PlayerAction::Draw {
                            #[cfg(feature = "spanned_tree")]
                                span: start_span,
                        },
                    tense: boseiju_lexer::Tense::BaseForm,
                })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                        span: end_span,
                })),
            ] => Ok(ParserNode::ImperativeKind {
                imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::Draw(
                    boseiju_tree::ability_tree::imperative::DrawImperative {
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
