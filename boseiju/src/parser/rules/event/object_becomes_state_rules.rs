use crate::ability_tree::terminals;
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
    /* "<object reference> becomes <state>" makes an object becomes state event */
    terminals::CardState::all().map(|state| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Become {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::CardState(state)).id(),
        ]),
        merged: ParserNode::Event { event: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Become { .. })),
                ParserNode::LexerToken(Token::CardState(state)),
            ] => Ok(ParserNode::Event {
                event: crate::ability_tree::event::Event::ObjectBecomesState(
                    crate::ability_tree::event::ObjectBecomesStateEvent {
                        object: reference.clone(),
                        state: state.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.node_span().merge(&state.node_span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
