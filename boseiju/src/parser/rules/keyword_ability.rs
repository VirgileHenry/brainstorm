mod affinity;
mod afterlife;
mod bestow;
mod blitz;
mod bloodthirst;
mod bushido;
mod cleave;
mod crew;
mod cumulative_upkeep;
mod cycling;
mod dash;
mod disguise;
mod echo;
mod enchant;
mod equip;
mod flashback;
mod freerunning;
mod kicker;
mod megamorph;
mod morph;
mod multiple_keyword_abilities;
mod ninjutsu;
mod outlast;
mod prototype;
mod reconfigure;
mod reinforce;
mod renown;
mod ripple;
mod surge;
mod suspend;
mod vanishing;
mod ward;
mod warp;

use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let standalone_keyword_abilities = crate::ability_tree::terminals::StandaloneKeywordAbility::all()
        .map(|keyword_ability| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                keyword_ability: keyword_ability.into(),
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::KeywordAbility {
                keyword_ability: dummy(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::KeywordAbility(keyword))] => Ok(ParserNode::KeywordAbility {
                    keyword_ability: crate::ability_tree::ability::keyword_ability::keyword_to_abilities(*keyword)?,
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [
        standalone_keyword_abilities,
        affinity::rules().collect::<Vec<_>>(),
        afterlife::rules().collect::<Vec<_>>(),
        bestow::rules().collect::<Vec<_>>(),
        blitz::rules().collect::<Vec<_>>(),
        bloodthirst::rules().collect::<Vec<_>>(),
        bushido::rules().collect::<Vec<_>>(),
        cleave::rules().collect::<Vec<_>>(),
        crew::rules().collect::<Vec<_>>(),
        cumulative_upkeep::rules().collect::<Vec<_>>(),
        cycling::rules().collect::<Vec<_>>(),
        dash::rules().collect::<Vec<_>>(),
        disguise::rules().collect::<Vec<_>>(),
        echo::rules().collect::<Vec<_>>(),
        enchant::rules().collect::<Vec<_>>(),
        equip::rules().collect::<Vec<_>>(),
        flashback::rules().collect::<Vec<_>>(),
        freerunning::rules().collect::<Vec<_>>(),
        kicker::rules().collect::<Vec<_>>(),
        megamorph::rules().collect::<Vec<_>>(),
        morph::rules().collect::<Vec<_>>(),
        multiple_keyword_abilities::rules().collect::<Vec<_>>(),
        ninjutsu::rules().collect::<Vec<_>>(),
        prototype::rules().collect::<Vec<_>>(),
        outlast::rules().collect::<Vec<_>>(),
        reinforce::rules().collect::<Vec<_>>(),
        reconfigure::rules().collect::<Vec<_>>(),
        renown::rules().collect::<Vec<_>>(),
        ripple::rules().collect::<Vec<_>>(),
        surge::rules().collect::<Vec<_>>(),
        suspend::rules().collect::<Vec<_>>(),
        vanishing::rules().collect::<Vec<_>>(),
        ward::rules().collect::<Vec<_>>(),
        warp::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
