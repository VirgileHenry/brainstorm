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
    /* Attach <permanent> to <permanent> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                token: intermediate::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Attach,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::BaseForm,
            }))
            .id(),
            ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::ImperativeKind {
            imperative: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                    token:
                        intermediate::KeywordAction {
                            keyword_action: mtg_data::KeywordAction::Attach,
                            #[cfg(feature = "spanned_tree")]
                                span: attach_span,
                        },
                    tense: boseiju_lexer::Tense::BaseForm,
                })),
                ParserNode::Permanent { permanent: object },
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To { .. })),
                ParserNode::Permanent { permanent: to },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::KeywordAction(
                    boseiju_tree::ability_tree::imperative::KeywordAction {
                        keyword: boseiju_tree::ability_tree::imperative::ExpandedKeywordAction::Attach(
                            boseiju_tree::ability_tree::imperative::attach::AttachKeywordAction {
                                object: object.clone(),
                                to: to.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: to.span().merge(attach_span),
                            },
                        ),
                        ability: boseiju_tree::ability_tree::imperative::attach::ability(
                            object,
                            to,
                            #[cfg(feature = "spanned_tree")]
                            to.span().merge(attach_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: to.span().merge(attach_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
