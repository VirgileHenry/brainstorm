use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    let default_rules = vec![/* The battlefield is a zone reference */ super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(Token::GlobalZone(intermediate::GlobalZone::TheBattlefield {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::ZoneReference {
            zone: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::GlobalZone(intermediate::GlobalZone::TheBattlefield {
                    #[cfg(feature = "spanned_tree")]
                    span,
                })),
            ] => Ok(ParserNode::ZoneReference {
                zone: boseiju_tree::ability_tree::zone::ZoneReference::TheBattlefield {
                    #[cfg(feature = "spanned_tree")]
                    span: *span,
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    }];

    let owned_zone_rules = [
        boseiju_lexer::terminal::OwnableZone::Graveyard {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        boseiju_lexer::terminal::OwnableZone::Hand {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        boseiju_lexer::terminal::OwnableZone::Library {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|zone| {
        [
            /* For zones, ambiguous token "your" is valid */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::OwnableZone(zone)).id(),
                ]),
                merged: ParserNode::ZoneReference {
                    zone: Default::default(),
                }
                .id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Your {
                            #[cfg(feature = "spanned_tree")]
                                span: owner_span,
                        })),
                        ParserNode::LexerToken(Token::OwnableZone(zone)),
                    ] => Ok(ParserNode::ZoneReference {
                        zone: boseiju_tree::ability_tree::zone::ZoneReference::OwnedZone(
                            boseiju_tree::ability_tree::zone::OwnedZone {
                                zone: zone.clone(),
                                owner: boseiju_tree::ability_tree::player::PlayerReference::You(
                                    boseiju_tree::ability_tree::player::You {
                                        #[cfg(feature = "spanned_tree")]
                                        span: *owner_span,
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: owner_span.merge(&zone.span()),
                            },
                        ),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* Otherwise, any player specifier is valid: "enchanted creature's controller graveyard" is valid */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::Player {
                        player: Default::default(),
                    }
                    .id(),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::ApostropheS {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::OwnableZone(zone)).id(),
                ]),
                merged: ParserNode::ZoneReference {
                    zone: Default::default(),
                }
                .id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::Player { player },
                        ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::ApostropheS { .. })),
                        ParserNode::LexerToken(Token::OwnableZone(zone)),
                    ] => Ok(ParserNode::ZoneReference {
                        zone: boseiju_tree::ability_tree::zone::ZoneReference::OwnedZone(
                            boseiju_tree::ability_tree::zone::OwnedZone {
                                zone: zone.clone(),
                                owner: player.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: player.span().merge(&zone.span()),
                            },
                        ),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
        ]
    })
    .flatten()
    .collect::<Vec<_>>();

    [default_rules, owned_zone_rules].into_iter().flatten()
}
