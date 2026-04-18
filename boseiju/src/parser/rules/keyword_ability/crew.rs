use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Crew <number> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                keyword_ability: mtg_data::KeywordAbility::Crew,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
        ]),
        merged: ParserNode::KeywordAbility {
            keyword_ability: dummy(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAbility(intermediates::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Crew,
                    #[cfg(feature = "spanned_tree")]
                        span: crew_span,
                })),
                ParserNode::Number { number },
            ] => Ok(ParserNode::KeywordAbility {
                keyword_ability: crate::ability_tree::ability::KeywordAbility {
                    keyword: crate::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Crew(
                        crate::ability_tree::ability::keyword_ability::CrewKeywordAbility {
                            amount: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.node_span().merge(crew_span),
                        },
                    ),
                    /* Fixme */
                    ability: crate::ability_tree::ability::Ability::Spell(crate::ability_tree::ability::spell::SpellAbility {
                        effects: crate::utils::HeapArrayVec::new(),
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }),
                    #[cfg(feature = "spanned_tree")]
                    span: number.node_span().merge(crew_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
