mod afterlife;
mod bestow;
mod bloodthirst;
mod bushido;
mod crew;
mod cycling;
mod echo;
mod enchant;
mod equip;
mod kicker;
mod megamorph;
mod ninjutsu;
mod prototype;
mod reinforce;
mod renown;
mod ripple;
mod suspend;
mod vanishing;
mod ward;

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
        afterlife::rules().collect::<Vec<_>>(),
        bestow::rules().collect::<Vec<_>>(),
        bloodthirst::rules().collect::<Vec<_>>(),
        bushido::rules().collect::<Vec<_>>(),
        crew::rules().collect::<Vec<_>>(),
        cycling::rules().collect::<Vec<_>>(),
        echo::rules().collect::<Vec<_>>(),
        enchant::rules().collect::<Vec<_>>(),
        equip::rules().collect::<Vec<_>>(),
        kicker::rules().collect::<Vec<_>>(),
        megamorph::rules().collect::<Vec<_>>(),
        ninjutsu::rules().collect::<Vec<_>>(),
        prototype::rules().collect::<Vec<_>>(),
        reinforce::rules().collect::<Vec<_>>(),
        renown::rules().collect::<Vec<_>>(),
        ripple::rules().collect::<Vec<_>>(),
        suspend::rules().collect::<Vec<_>>(),
        vanishing::rules().collect::<Vec<_>>(),
        ward::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
