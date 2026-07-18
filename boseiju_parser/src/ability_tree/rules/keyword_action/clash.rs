use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* Clash with <player> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                token: intermediate::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Clash,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::BaseForm,
            }))
            .id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::With {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Player {
                player: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::ImperativeKind {
            imperative: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                    token:
                        intermediate::KeywordAction {
                            keyword_action: mtg_data::KeywordAction::Clash,
                            #[cfg(feature = "spanned_tree")]
                                span: clash_span,
                        },
                    tense: boseiju_lexer::Tense::BaseForm,
                })),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::With { .. })),
                ParserNode::Player { player },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::KeywordAction(
                    boseiju_tree::ability_tree::imperative::KeywordAction {
                        keyword: boseiju_tree::ability_tree::imperative::ExpandedKeywordAction::Clash(
                            boseiju_tree::ability_tree::imperative::clash::ClashKeywordAction {
                                opponent: player.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: player.span().merge(clash_span),
                            },
                        ),
                        ability: boseiju_tree::ability_tree::imperative::clash::ability(
                            player,
                            #[cfg(feature = "spanned_tree")]
                            player.span().merge(clash_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(clash_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
