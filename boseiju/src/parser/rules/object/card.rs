use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<count> <card kind>" is a card */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::CardKind { card: dummy() }.id(),
            ]),
            merged: ParserNode::Card { card: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CountSpecifier { count }, ParserNode::CardKind { card }] => Ok(ParserNode::Card {
                    card: object::Card::Reference(object::reference::CardReference {
                        count: count.clone(),
                        kind: card.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.node_span().merge(&card.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <card kind>" can be used as a card reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CardKind { card: dummy() }.id(),
            ]),
            merged: ParserNode::Card { card: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::CardKind { card },
                ] => Ok(ParserNode::Card {
                    card: object::Card::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: card.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<own card name>" is always a self referencing card */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CardOwnName(intermediates::CardOwnName {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Card { card: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CardOwnName(intermediates::CardOwnName {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                ] => Ok(ParserNode::Card {
                    card: object::Card::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: *start_span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<card reference> or <card reference>" makes a one among reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Card { card: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Card { card: dummy() }.id(),
            ]),
            merged: ParserNode::Card { card: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Card { card: c1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::Card { card: c2 },
                ] => Ok(ParserNode::Card {
                    card: object::Card::OneAmong(object::OneAmong {
                        references: {
                            let mut references = crate::utils::HeapArrayVec::new();
                            references.push(c1.clone());
                            references.push(c2.clone());
                            references
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: c1.node_span().merge(&c2.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "it" makes a previously mentionned card */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Card { card: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Card {
                    card: object::Card::PreviouslyMentionned(object::PreviouslyMentionned {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
