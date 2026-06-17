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
    /* Exert <permanent> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Exert,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Permanent { permanent: dummy() }.id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Exert,
                    #[cfg(feature = "spanned_tree")]
                        span: exert_span,
                })),
                ParserNode::Permanent { permanent },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                    crate::ability_tree::imperative::KeywordAction {
                        keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Exert(
                            crate::ability_tree::imperative::exert::ExertKeywordAction {
                                permanent: permanent.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.node_span().merge(exert_span),
                            },
                        ),
                        ability: crate::ability_tree::imperative::exert::ability(
                            permanent,
                            #[cfg(feature = "spanned_tree")]
                            permanent.node_span().merge(exert_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span().merge(exert_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
