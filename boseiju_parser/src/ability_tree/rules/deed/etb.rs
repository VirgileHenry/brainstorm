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
        /* "<passive permanent ref> enters the battlefield" is a etb deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentPassive {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::CardActions(intermediate::CardActions::Enters {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::GlobalZone(intermediate::GlobalZone::TheBattlefield {
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
                    ParserNode::PermanentPassive { permanent },
                    ParserNode::LexerToken(boseiju_lexer::Token::CardActions(intermediate::CardActions::Enters { .. })),
                    ParserNode::LexerToken(boseiju_lexer::Token::GlobalZone(intermediate::GlobalZone::TheBattlefield {
                        #[cfg(feature = "spanned_tree")]
                            span: the_bf_span,
                    })),
                ] => Ok(ParserNode::DeedEtb {
                    deed: boseiju_tree::ability_tree::deed::etb::EntersTheBattlefield {
                        permanent: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(the_bf_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive permanent ref> enters" is a etb deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentPassive {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::CardActions(intermediate::CardActions::Enters {
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
                    ParserNode::PermanentPassive { permanent },
                    ParserNode::LexerToken(boseiju_lexer::Token::CardActions(intermediate::CardActions::Enters {
                        #[cfg(feature = "spanned_tree")]
                            span: enters_span,
                    })),
                ] => Ok(ParserNode::DeedEtb {
                    deed: boseiju_tree::ability_tree::deed::etb::EntersTheBattlefield {
                        permanent: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(enters_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
