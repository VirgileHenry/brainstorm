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
    /* Goad <creature> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Goad,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Creature { creature: dummy() }.id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Goad,
                    #[cfg(feature = "spanned_tree")]
                        span: goad_span,
                })),
                ParserNode::Creature { creature },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                    crate::ability_tree::imperative::KeywordAction {
                        keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Goad(
                            crate::ability_tree::imperative::goad::GoadKeywordAction {
                                creature: creature.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.node_span().merge(goad_span),
                            },
                        ),
                        ability: crate::ability_tree::imperative::goad::ability(
                            creature,
                            #[cfg(feature = "spanned_tree")]
                            creature.node_span().merge(goad_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(goad_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
