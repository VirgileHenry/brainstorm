mod descend;

use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let standalone_ability_words = crate::ability_tree::terminals::StandaloneAbilityWord::all()
        .map(|ability_word| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AbilityWord(intermediates::AbilityWord {
                ability_word: ability_word.into(),
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::AbilityWord { ability_word: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::AbilityWord(word))] => Ok(ParserNode::AbilityWord {
                    ability_word: crate::ability_tree::ability::ability_word::keyword_to_ability_word(*word)?,
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [standalone_ability_words, descend::rules().collect::<Vec<_>>()]
        .into_iter()
        .flatten()
}
