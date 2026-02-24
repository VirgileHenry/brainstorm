use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* A single Ability can be turned into an ability tree with a single element */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::WrittenOrKeywordAbilty { ability: dummy() }.id()]),
            merged: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::WrittenOrKeywordAbilty { ability }] => Some(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = arrayvec::ArrayVec::new_const();
                        abilities.push(*ability.clone());
                        Box::new(crate::AbilityTree { abilities })
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Abilities separated by new lines can be merged into a single ability tree */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::AbilityTree { tree: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)).id(),
                ParserNode::WrittenOrKeywordAbilty { ability: dummy() }.id(),
            ]),
            merged: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AbilityTree { tree },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
                    ParserNode::WrittenOrKeywordAbilty { ability },
                ] => Some(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = tree.abilities.clone();
                        abilities.push(*ability.clone());
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
