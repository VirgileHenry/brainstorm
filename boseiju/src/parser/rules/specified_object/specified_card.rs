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
        /* "<card kind>" makes a specified card  */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::CardKind { card: dummy() }.id()]),
            merged: ParserNode::SpecifiedCard { card: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CardKind { card }] => Ok(ParserNode::SpecifiedCard {
                    card: object::specified_object::SpecifiedCard {
                        kind: card.clone(),
                        specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: card.node_span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<card kind> card" makes a specified card  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CardKind { card: dummy() }.id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Card {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::SpecifiedCard { card: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CardKind { card },
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Card {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::SpecifiedCard {
                    card: object::specified_object::SpecifiedCard {
                        kind: card.clone(),
                        specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: card.node_span().merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
