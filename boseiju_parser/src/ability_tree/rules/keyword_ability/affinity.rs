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
    /* Affinity <number> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                keyword_ability: mtg_data::KeywordAbility::Affinity,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::For {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::KeywordAbility {
            keyword_ability: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAbility(intermediate::KeywordAbility {
                    keyword_ability: mtg_data::KeywordAbility::Affinity,
                    #[cfg(feature = "spanned_tree")]
                        span: affinity_span,
                })),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::For { .. })),
                ParserNode::Permanent { permanent },
            ] => Ok(ParserNode::KeywordAbility {
                keyword_ability: boseiju_tree::ability_tree::ability::KeywordAbility {
                    keyword: boseiju_tree::ability_tree::ability::keyword_ability::ExpandedKeywordAbility::Affinity(
                        boseiju_tree::ability_tree::ability::keyword_ability::AffinityKeywordAbility {
                            for_object: permanent.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().merge(affinity_span),
                        },
                    ),
                    /* Fixme */
                    ability: boseiju_tree::ability_tree::ability::WrittenAbility::Spell(
                        boseiju_tree::ability_tree::ability::spell::SpellAbility {
                            effects: boseiju_tree::HeapArrayVec::new(),
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        },
                    ),
                    #[cfg(feature = "spanned_tree")]
                    span: permanent.span().merge(affinity_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
