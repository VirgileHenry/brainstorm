use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* Endure <number> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                token: intermediate::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Endure,
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
        ]),
        merged: ParserNode::ImperativeKind {
            imperative: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                    token:
                        intermediate::KeywordAction {
                            keyword_action: mtg_data::KeywordAction::Endure,
                            #[cfg(feature = "spanned_tree")]
                                span: endure_span,
                        },
                    tense: boseiju_lexer::Tense::BaseForm,
                })),
                ParserNode::Number { number },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::KeywordAction(
                    boseiju_tree::ability_tree::imperative::KeywordAction {
                        keyword: boseiju_tree::ability_tree::imperative::ExpandedKeywordAction::Endure(
                            boseiju_tree::ability_tree::imperative::endure::EndureKeywordAction {
                                amount: number.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: number.span().merge(endure_span),
                            },
                        ),
                        ability: boseiju_tree::ability_tree::imperative::endure::ability(
                            number,
                            #[cfg(feature = "spanned_tree")]
                            number.span().merge(endure_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: number.span().merge(endure_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
