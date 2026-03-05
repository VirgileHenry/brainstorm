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
        /* Present form: "whenever you gain life" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain { .. })),
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::LifeGained(crate::ability_tree::event::LifeGainedEvent {
                        player: player.clone(),
                        minimum_amount: None,
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Past form: "if you have gained life" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain { .. })),
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::LifeGained(crate::ability_tree::event::LifeGainedEvent {
                        player: player.clone(),
                        minimum_amount: None,
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Past form + number requirement: "if you have gained 3 or more life" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain { .. })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::LifeGained(crate::ability_tree::event::LifeGainedEvent {
                        player: player.clone(),
                        minimum_amount: Some(number.clone()),
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Conditional form: "if you would gain life" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain { .. })),
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::LifeGained(crate::ability_tree::event::LifeGainedEvent {
                        player: player.clone(),
                        minimum_amount: None,
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
