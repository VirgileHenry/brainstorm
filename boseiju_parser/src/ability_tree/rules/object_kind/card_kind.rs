use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "card" is the default card kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::CardKind {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::CardKind {
                    card: object::kind::CardKind::Card {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified permanent> card" can be used as a card kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedPermanent {
                permanent: Default::default(),
            }
            .id()]),
            merged: ParserNode::CardKind {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedPermanent { permanent }] => Ok(ParserNode::CardKind {
                    card: object::kind::CardKind::Permanent(permanent.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<card kind> or <card kind>" makes a one among kind */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CardKind {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CardKind {
                    card: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::CardKind {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CardKind { card: c1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or { .. })),
                    ParserNode::CardKind { card: c2 },
                ] => Ok(ParserNode::CardKind {
                    card: object::kind::CardKind::OneAmong(object::OneAmong {
                        references: {
                            let mut references = boseiju_tree::HeapArrayVec::new();
                            references.push(c1.clone());
                            references.push(c2.clone());
                            references
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: c1.span().merge(&c2.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
