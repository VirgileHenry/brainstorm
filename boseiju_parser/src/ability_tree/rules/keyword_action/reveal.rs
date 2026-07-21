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
    [
        /* Reveal <card> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Reveal,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
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
                    ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Reveal,
                                #[cfg(feature = "spanned_tree")]
                                    span: reveal_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::Card { card },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::KeywordAction(
                        boseiju_tree::ability_tree::imperative::KeywordAction {
                            keyword: boseiju_tree::ability_tree::imperative::ExpandedKeywordAction::Reveal(
                                boseiju_tree::ability_tree::imperative::reveal::RevealKeywordAction {
                                    card: card.clone(),
                                    from: None,
                                    #[cfg(feature = "spanned_tree")]
                                    span: card.span().merge(reveal_span),
                                },
                            ),
                            ability: boseiju_tree::ability_tree::imperative::reveal::ability(
                                card,
                                None,
                                #[cfg(feature = "spanned_tree")]
                                card.span().merge(reveal_span),
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: card.span().merge(reveal_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Reveal <card> from <zone> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Reveal,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::Card {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference {
                    zone: Default::default(),
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
                                keyword_action: mtg_data::KeywordAction::Reveal,
                                #[cfg(feature = "spanned_tree")]
                                    span: reveal_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::Card { card },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From { .. })),
                    ParserNode::ZoneReference { zone },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::KeywordAction(
                        boseiju_tree::ability_tree::imperative::KeywordAction {
                            keyword: boseiju_tree::ability_tree::imperative::ExpandedKeywordAction::Reveal(
                                boseiju_tree::ability_tree::imperative::reveal::RevealKeywordAction {
                                    card: card.clone(),
                                    from: Some(zone.clone()),
                                    #[cfg(feature = "spanned_tree")]
                                    span: zone.span().merge(reveal_span),
                                },
                            ),
                            ability: boseiju_tree::ability_tree::imperative::reveal::ability(
                                card,
                                Some(zone),
                                #[cfg(feature = "spanned_tree")]
                                zone.span().merge(reveal_span),
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: zone.span().merge(reveal_span),
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
