use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<active creature> attacks" is an active attack deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureActive {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::AmbiguousToken(intermediate::AmbiguousToken::Attack {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::DeedEtb {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureActive { creature },
                    ParserNode::LexerToken(boseiju_lexer::Token::AmbiguousToken(intermediate::AmbiguousToken::Attack {
                        #[cfg(feature = "spanned_tree")]
                            span: attack_span,
                    })),
                ] => Ok(ParserNode::DeedAttackActive {
                    deed: boseiju_tree::ability_tree::deed::attack::Attack::CreatureAttack(
                        boseiju_tree::ability_tree::deed::attack::CreatureAttack {
                            creature: creature.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().merge(attack_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive creature> attacks" is a passive attack deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreaturePassive {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::AmbiguousToken(intermediate::AmbiguousToken::Attack {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::DeedEtb {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreaturePassive { creature },
                    ParserNode::LexerToken(boseiju_lexer::Token::AmbiguousToken(intermediate::AmbiguousToken::Attack {
                        #[cfg(feature = "spanned_tree")]
                            span: attack_span,
                    })),
                ] => Ok(ParserNode::DeedAttackPassive {
                    deed: boseiju_tree::ability_tree::deed::attack::Attack::CreatureAttack(
                        boseiju_tree::ability_tree::deed::attack::CreatureAttack {
                            creature: creature.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().merge(attack_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active player> attacks" is an active player attack deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PlayerActive {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::AmbiguousToken(intermediate::AmbiguousToken::Attack {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::DeedEtb {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PlayerActive { player },
                    ParserNode::LexerToken(boseiju_lexer::Token::AmbiguousToken(intermediate::AmbiguousToken::Attack {
                        #[cfg(feature = "spanned_tree")]
                            span: attack_span,
                    })),
                ] => Ok(ParserNode::DeedAttackActive {
                    deed: boseiju_tree::ability_tree::deed::attack::Attack::PlayerAttack(
                        boseiju_tree::ability_tree::deed::attack::PlayerAttack {
                            player: player.clone(),
                            attack_with: None,
                            #[cfg(feature = "spanned_tree")]
                            span: player.span().merge(attack_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active player> attacks" is an active player attack deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PlayerPassive {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::AmbiguousToken(intermediate::AmbiguousToken::Attack {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::DeedEtb {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PlayerPassive { player },
                    ParserNode::LexerToken(boseiju_lexer::Token::AmbiguousToken(intermediate::AmbiguousToken::Attack {
                        #[cfg(feature = "spanned_tree")]
                            span: attack_span,
                    })),
                ] => Ok(ParserNode::DeedAttackPassive {
                    deed: boseiju_tree::ability_tree::deed::attack::Attack::PlayerAttack(
                        boseiju_tree::ability_tree::deed::attack::PlayerAttack {
                            player: player.clone(),
                            attack_with: None,
                            #[cfg(feature = "spanned_tree")]
                            span: player.span().merge(attack_span),
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
