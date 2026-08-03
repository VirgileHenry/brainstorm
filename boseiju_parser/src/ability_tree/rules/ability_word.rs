mod descend;
mod standalone;

use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    let standalone_ability_words = boseiju_lexer::terminal::StandaloneAbilityWord::all()
        .map(|ability_word| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AbilityWord(intermediate::AbilityWord {
                ability_word: ability_word.into(),
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::AbilityWord {
                ability_word: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::AbilityWord(word))] => Ok(ParserNode::AbilityWord {
                    ability_word: standalone::keyword_to_ability_word(*word)?,
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
