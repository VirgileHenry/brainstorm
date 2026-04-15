use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Surveil <mana cost> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Surveil,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Surveil,
                    #[cfg(feature = "spanned_tree")]
                        span: surveil_span,
                })),
                ParserNode::Number { number },
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::KeywordAction(
                    crate::ability_tree::imperative::KeywordAction {
                        keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Surveil(
                            crate::ability_tree::imperative::SurveilKeywordAbility {
                                amount: number.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: number.node_span().merge(surveil_span),
                            },
                        ),
                        /* Fixme */
                        ability: crate::ability_tree::ability::spell::SpellAbility {
                            effects: crate::utils::HeapArrayVec::new(),
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: number.node_span().merge(surveil_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
