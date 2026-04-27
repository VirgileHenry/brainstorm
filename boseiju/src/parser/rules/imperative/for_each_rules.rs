use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<spell ability> for each <game state number>" is a "for each" imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::SpellAbility { ability: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ForEach {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::GameStateNumber { number: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::SpellAbility { ability },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ForEach { .. })),
                    ParserNode::GameStateNumber { number },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::ForEach(
                        crate::ability_tree::imperative::ForEachImperative {
                            ability: ability.clone(),
                            for_each: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: ability.node_span().merge(&number.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
