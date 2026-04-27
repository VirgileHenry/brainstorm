mod adapt;
mod bolster;
mod mill;
mod scry;
mod support;
mod surveil;

use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let keyword_actions_to_imperatives = crate::ability_tree::terminals::StandaloneKeywordAction::all()
        .map(|keyword_action| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: keyword_action.into(),
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::KeywordAction(keyword))] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                        crate::ability_tree::imperative::keyword_action_to_abilities(*keyword)?,
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [
        keyword_actions_to_imperatives,
        adapt::rules().collect::<Vec<_>>(),
        bolster::rules().collect::<Vec<_>>(),
        mill::rules().collect::<Vec<_>>(),
        scry::rules().collect::<Vec<_>>(),
        support::rules().collect::<Vec<_>>(),
        surveil::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
