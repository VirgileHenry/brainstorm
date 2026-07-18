use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* Descend <number> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::AbilityWord(intermediate::AbilityWord {
                ability_word: mtg_data::AbilityWord::Descend,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number {
                number: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::AbilityWord {
            ability_word: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::AbilityWord(intermediate::AbilityWord {
                    ability_word: mtg_data::AbilityWord::Descend,
                    #[cfg(feature = "spanned_tree")]
                        span: descend_span,
                })),
                ParserNode::Number { number },
            ] => Ok(ParserNode::AbilityWord {
                ability_word: boseiju_tree::ability_tree::ability::ability_word::ExpandedAbilityWord::Descend(
                    boseiju_tree::ability_tree::ability::ability_word::DescendAbilityWord {
                        amount: number.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: number.span().merge(descend_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
