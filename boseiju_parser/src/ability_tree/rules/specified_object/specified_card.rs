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
        /* "<card kind>" makes a specified card  */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::CardKind {
                card: Default::default(),
            }
            .id()]),
            merged: ParserNode::SpecifiedCard {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CardKind { card }] => Ok(ParserNode::SpecifiedCard {
                    card: object::specified_object::SpecifiedCard {
                        kind: card.clone(),
                        specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: card.span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<card kind> card" makes a specified card  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CardKind {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::SpecifiedCard {
                card: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CardKind { card },
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Card {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::SpecifiedCard {
                    card: object::specified_object::SpecifiedCard {
                        kind: card.clone(),
                        specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: card.span().merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
