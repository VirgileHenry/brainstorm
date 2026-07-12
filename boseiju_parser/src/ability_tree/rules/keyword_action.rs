mod activate;
mod adapt;
mod airbend;
mod amass;
mod attach;
mod behold;
mod blight;
mod bolster;
mod cast;
mod clash;
mod cloak;
mod collect_evidence;
mod connive;
mod convert;
mod counter;
mod create;
mod destroy;
mod detain;
mod discard;
mod discover;
mod double;
mod earthbend;
mod endure;
mod exchange;
mod exert;
mod exile;
mod explore;
mod fateseal;
mod fight;
mod goad;
mod incubate;
mod manifest;
mod meld;
mod mill;
mod monstrosity;
mod play;
mod plot;
mod regenerate;
mod reveal;
mod sacrifice;
mod scry;
mod search;
mod support;
mod surveil;
mod suspect;
mod tap;
mod transform;
mod untap;
mod vote;
mod waterbend;

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
        airbend::rules().collect::<Vec<_>>(),
        amass::rules().collect::<Vec<_>>(),
        attach::rules().collect::<Vec<_>>(),
        behold::rules().collect::<Vec<_>>(),
        blight::rules().collect::<Vec<_>>(),
        bolster::rules().collect::<Vec<_>>(),
        cast::rules().collect::<Vec<_>>(),
        clash::rules().collect::<Vec<_>>(),
        cloak::rules().collect::<Vec<_>>(),
        collect_evidence::rules().collect::<Vec<_>>(),
        connive::rules().collect::<Vec<_>>(),
        convert::rules().collect::<Vec<_>>(),
        counter::rules().collect::<Vec<_>>(),
        create::rules().collect::<Vec<_>>(),
        destroy::rules().collect::<Vec<_>>(),
        detain::rules().collect::<Vec<_>>(),
        discard::rules().collect::<Vec<_>>(),
        discover::rules().collect::<Vec<_>>(),
        earthbend::rules().collect::<Vec<_>>(),
        endure::rules().collect::<Vec<_>>(),
        exert::rules().collect::<Vec<_>>(),
        exile::rules().collect::<Vec<_>>(),
        explore::rules().collect::<Vec<_>>(),
        fateseal::rules().collect::<Vec<_>>(),
        goad::rules().collect::<Vec<_>>(),
        incubate::rules().collect::<Vec<_>>(),
        manifest::rules().collect::<Vec<_>>(),
        mill::rules().collect::<Vec<_>>(),
        monstrosity::rules().collect::<Vec<_>>(),
        play::rules().collect::<Vec<_>>(),
        plot::rules().collect::<Vec<_>>(),
        regenerate::rules().collect::<Vec<_>>(),
        reveal::rules().collect::<Vec<_>>(),
        sacrifice::rules().collect::<Vec<_>>(),
        scry::rules().collect::<Vec<_>>(),
        search::rules().collect::<Vec<_>>(),
        support::rules().collect::<Vec<_>>(),
        surveil::rules().collect::<Vec<_>>(),
        suspect::rules().collect::<Vec<_>>(),
        tap::rules().collect::<Vec<_>>(),
        transform::rules().collect::<Vec<_>>(),
        untap::rules().collect::<Vec<_>>(),
        waterbend::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
