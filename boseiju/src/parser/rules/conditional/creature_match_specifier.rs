use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<creature reference> is a <creature specifier>" condition */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureReference { creature: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CreatureSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::Condition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureReference { creature },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                    ParserNode::CreatureSpecifier { specifier },
                ] => Ok(ParserNode::Condition {
                    condition: crate::ability_tree::conditional::Condition::ObjectMatchSpecifiers(
                        crate::ability_tree::conditional::ConditionCreatureMatchSpecifier {
                            creature: creature.clone(),
                            specifier: specifier.clone(),
                            shall_match: true,
                            #[cfg(feature = "spanned_tree")]
                            span: creature.node_span().merge(&specifier.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature reference>'s a <creature specifier>" condition */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureReference { creature: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ApostropheS {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::CreatureSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::Condition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureReference { creature },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ApostropheS { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                    ParserNode::CreatureSpecifier { specifier },
                ] => Ok(ParserNode::Condition {
                    condition: crate::ability_tree::conditional::Condition::ObjectMatchSpecifiers(
                        crate::ability_tree::conditional::ConditionCreatureMatchSpecifier {
                            creature: creature.clone(),
                            specifier: specifier.clone(),
                            shall_match: true,
                            #[cfg(feature = "spanned_tree")]
                            span: creature.node_span().merge(&specifier.node_span()),
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
