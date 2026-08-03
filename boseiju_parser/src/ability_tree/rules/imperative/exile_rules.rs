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
        /* "exile <permanent reference>" -> move object from battlfield to exile */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Exile {
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
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Exile {
                        #[cfg(feature = "spanned_tree")]
                            span: exile_span,
                    })),
                    ParserNode::Permanent { permanent },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::ChangeZone(
                        boseiju_tree::ability_tree::imperative::ChangeZoneImperative {
                            object: permanent.to_card(),
                            from: boseiju_tree::ability_tree::zone::ZoneReference::TheBattlefield {
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().empty_at_end(),
                            },
                            to: boseiju_tree::ability_tree::zone::ZoneReference::Exile {
                                #[cfg(feature = "spanned_tree")]
                                span: *exile_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(exile_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "exile <card reference> from <zone>" means to move from <zone> to <exile> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Exile {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
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
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Exile {
                        #[cfg(feature = "spanned_tree")]
                            span: exile_span,
                    })),
                    ParserNode::Card { card },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From { .. })),
                    ParserNode::ZoneReference { zone },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::ChangeZone(
                        boseiju_tree::ability_tree::imperative::ChangeZoneImperative {
                            object: card.clone(),
                            from: zone.clone(),
                            to: boseiju_tree::ability_tree::zone::ZoneReference::Exile {
                                #[cfg(feature = "spanned_tree")]
                                span: *exile_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: zone.span().merge(exile_span),
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
