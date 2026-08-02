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
    /* Search you library for <card> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                token: intermediate::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Search,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::BaseForm,
            }))
            .id(),
            ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::For {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Card {
                card: Default::default(),
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
                            keyword_action: mtg_data::KeywordAction::Search,
                            #[cfg(feature = "spanned_tree")]
                                span: search_span,
                        },
                    tense: boseiju_lexer::Tense::BaseForm,
                })),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your { .. })),
                ParserNode::LexerToken(Token::OwnableZone(boseiju_lexer::terminal::OwnableZone::Library { .. })),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::For { .. })),
                ParserNode::Card { card },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::KeywordAction(
                    boseiju_tree::ability_tree::imperative::KeywordAction {
                        keyword: boseiju_tree::ability_tree::imperative::ExpandedKeywordAction::Search(
                            boseiju_tree::ability_tree::imperative::search::SearchKeywordAction {
                                card: card.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: card.span().merge(search_span),
                            },
                        ),
                        ability: boseiju_tree::ability_tree::imperative::search::ability(
                            card,
                            #[cfg(feature = "spanned_tree")]
                            card.span().merge(search_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: card.span().merge(search_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
