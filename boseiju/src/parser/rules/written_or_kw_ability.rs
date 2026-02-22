use super::ParserNode;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Ability as an written ability */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::Ability { ability: dummy() }.id()]),
            result: ParserNode::WrittenOrKeywordAbilty { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Ability { ability }] => Some(ParserNode::WrittenOrKeywordAbilty {
                    ability: Box::new(crate::ability_tree::ability::WrittenOrKeywordAbilty::Written(
                        *ability.clone(),
                    )),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Continuous effects are static abilities */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::KeywordAbility { ability: dummy() }.id()]),
            result: ParserNode::WrittenOrKeywordAbilty { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::KeywordAbility { ability }] => Some(ParserNode::WrittenOrKeywordAbilty {
                    ability: Box::new(crate::ability_tree::ability::WrittenOrKeywordAbilty::Keyword(ability.clone())),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
