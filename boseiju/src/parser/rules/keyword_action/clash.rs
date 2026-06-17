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
    /* Clash with <player> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Clash,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Player { player: dummy() }.id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Clash,
                    #[cfg(feature = "spanned_tree")]
                        span: clash_span,
                })),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With { .. })),
                ParserNode::Player { player },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                    crate::ability_tree::imperative::KeywordAction {
                        keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Clash(
                            crate::ability_tree::imperative::clash::ClashKeywordAction {
                                opponent: player.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: player.node_span().merge(clash_span),
                            },
                        ),
                        ability: crate::ability_tree::imperative::clash::ability(
                            player,
                            #[cfg(feature = "spanned_tree")]
                            player.node_span().merge(clash_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(clash_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
