use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Reveal <card> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Reveal,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Card { card: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Reveal,
                        #[cfg(feature = "spanned_tree")]
                            span: reveal_span,
                    })),
                    ParserNode::Card { card },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                        crate::ability_tree::imperative::KeywordAction {
                            keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Reveal(
                                crate::ability_tree::imperative::reveal::RevealKeywordAction {
                                    card: card.clone(),
                                    from: None,
                                    #[cfg(feature = "spanned_tree")]
                                    span: card.node_span().merge(reveal_span),
                                },
                            ),
                            ability: crate::ability_tree::imperative::reveal::ability(
                                card,
                                None,
                                #[cfg(feature = "spanned_tree")]
                                card.node_span().merge(reveal_span),
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: card.node_span().merge(reveal_span),
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
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Reveal,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Card { card: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference { zone: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Reveal,
                        #[cfg(feature = "spanned_tree")]
                            span: reveal_span,
                    })),
                    ParserNode::Card { card },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From { .. })),
                    ParserNode::ZoneReference { zone },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                        crate::ability_tree::imperative::KeywordAction {
                            keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Reveal(
                                crate::ability_tree::imperative::reveal::RevealKeywordAction {
                                    card: card.clone(),
                                    from: Some(zone.clone()),
                                    #[cfg(feature = "spanned_tree")]
                                    span: zone.node_span().merge(reveal_span),
                                },
                            ),
                            ability: crate::ability_tree::imperative::reveal::ability(
                                card,
                                Some(zone),
                                #[cfg(feature = "spanned_tree")]
                                zone.node_span().merge(reveal_span),
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: zone.node_span().merge(reveal_span),
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
