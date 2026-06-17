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
    /* Discover <number> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Discover,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Discover,
                    #[cfg(feature = "spanned_tree")]
                        span: discover_span,
                })),
                ParserNode::Number { number },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                    crate::ability_tree::imperative::KeywordAction {
                        keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Discover(
                            crate::ability_tree::imperative::discover::DiscoverKeywordAction {
                                amount: number.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: number.node_span().merge(discover_span),
                            },
                        ),
                        ability: crate::ability_tree::imperative::discover::ability(
                            number,
                            #[cfg(feature = "spanned_tree")]
                            number.node_span().merge(discover_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: number.node_span().merge(discover_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
