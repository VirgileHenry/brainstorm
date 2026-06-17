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
    /* <creature> Explore */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::Creature { creature: dummy() }.id(),
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Explore,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Creature { creature },
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Explore,
                    #[cfg(feature = "spanned_tree")]
                        span: explore_span,
                })),
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                    crate::ability_tree::imperative::KeywordAction {
                        keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Explore(
                            crate::ability_tree::imperative::explore::ExploreKeywordAction {
                                creature: creature.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.node_span().merge(explore_span),
                            },
                        ),
                        ability: crate::ability_tree::imperative::explore::ability(
                            creature,
                            #[cfg(feature = "spanned_tree")]
                            creature.node_span().merge(explore_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(explore_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
