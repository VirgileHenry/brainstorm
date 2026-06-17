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
    /* Discard <card> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Discard,
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
                    keyword_action: mtg_data::KeywordAction::Discard,
                    #[cfg(feature = "spanned_tree")]
                        span: discard_span,
                })),
                ParserNode::Card { card },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                    crate::ability_tree::imperative::KeywordAction {
                        keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Discard(
                            crate::ability_tree::imperative::discard::DiscardKeywordAction {
                                card: card.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: card.node_span().merge(discard_span),
                            },
                        ),
                        ability: crate::ability_tree::imperative::discard::ability(
                            card,
                            #[cfg(feature = "spanned_tree")]
                            card.node_span().merge(discard_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: card.node_span().merge(discard_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
