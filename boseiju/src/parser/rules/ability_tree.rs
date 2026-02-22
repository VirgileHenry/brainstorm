use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* A single Ability can be turned into an ability tree with a single element */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::Ability { ability: dummy() }.id()]),
            result: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Ability { ability }] => Some(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = arrayvec::ArrayVec::new_const();
                        abilities.push(crate::ability_tree::ability::WrittenOrKeywordAbilty::Written(
                            *ability.clone(),
                        ));
                        Box::new(crate::AbilityTree { abilities })
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Abilities separated by new lines can be merged into a single ability tree */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::AbilityTree { tree: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)).id(),
                ParserNode::Ability { ability: dummy() }.id(),
            ]),
            result: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AbilityTree { tree },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
                    ParserNode::Ability { ability },
                ] => Some(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = tree.abilities.clone();
                        abilities.push(crate::ability_tree::ability::WrittenOrKeywordAbilty::Written(
                            *ability.clone(),
                        ));
                        Box::new(crate::AbilityTree { abilities })
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
