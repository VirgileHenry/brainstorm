use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::ability_tree::object;
use crate::ability_tree::object::CardObjectKind;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "Mill <amount> cards" */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Mill,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Card(CardObjectKind {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            })))
            .id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Mill,
                    #[cfg(feature = "spanned_tree")]
                        span: start_span,
                })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Card(CardObjectKind {
                    #[cfg(feature = "spanned_tree")]
                        span: end_span,
                }))),
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                    crate::ability_tree::imperative::KeywordAction {
                        keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Mill(
                            crate::ability_tree::imperative::MillKeywordAction {
                                amount: number.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: start_span.merge(end_span),
                            },
                        ),
                        /* Fixme */
                        ability: crate::ability_tree::ability::spell::SpellAbility {
                            effects: crate::utils::HeapArrayVec::new(),
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
