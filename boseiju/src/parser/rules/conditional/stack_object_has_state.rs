use crate::ability_tree::conditional;
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
    /* "<spell> is <stack object state>" condition */
    crate::ability_tree::state::StackObjectState::all().map(|state| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::Spell { spell: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::StackObjectState(state)).id(),
        ]),
        merged: ParserNode::Condition { condition: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Spell { spell },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is { .. })),
                ParserNode::LexerToken(Token::StackObjectState(state)),
            ] => Ok(ParserNode::Condition {
                condition: conditional::Condition::StackObjectHasState(conditional::ConditionStackObjectHasState {
                    stack_obj: spell.clone(),
                    state: state.clone(),
                    has_state: true,
                    #[cfg(feature = "spanned_tree")]
                    span: spell.node_span().merge(&state.node_span()),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
