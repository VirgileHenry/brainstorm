use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Descend <number> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::AbilityWord(intermediates::AbilityWord {
                ability_word: mtg_data::AbilityWord::Descend,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
        ]),
        merged: ParserNode::AbilityWord { ability_word: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::AbilityWord(intermediates::AbilityWord {
                    ability_word: mtg_data::AbilityWord::Descend,
                    #[cfg(feature = "spanned_tree")]
                        span: descend_span,
                })),
                ParserNode::Number { number },
            ] => Ok(ParserNode::AbilityWord {
                ability_word: crate::ability_tree::ability::ability_word::ExpandedAbilityWord::Descend(
                    crate::ability_tree::ability::ability_word::DescendAbilityWord {
                        amount: number.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: number.node_span().merge(descend_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
