use super::ParserNode;

use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;

macro_rules! count_and_object_to_ref {
    ( $count:path ) => {
        crate::make_parser_rule!(
            [
                ParserNode::LexerToken(TokenKind::CountSpecifier($count)),
                ParserNode::ObjectSpecifiers(specifiers)
            ] => Some(ParserNode::ObjectReference( {
                crate::ability_tree::object::ObjectReference::SpecifiedObj {
                    amount: $count,
                    specifiers: specifiers.clone(),
                }
            } ))
        )
    };
}

#[rustfmt::skip]
pub const OBJECT_REFERENCES_RULES: &[super::ParserRule] = &[

    /* ==================================================== */
    /* ========== Conversion to Object Specifier ========== */
    /* ==================================================== */

    /* Some cases, there will be no count specifier, there is an implicit "all". */
    crate::make_parser_rule!(
        [ParserNode::ObjectSpecifiers(specifiers)] => Some(ParserNode::ObjectReference( {
            crate::ability_tree::object::ObjectReference::SpecifiedObj {
                amount: crate::ability_tree::terminals::CountSpecifier::All,
                specifiers: specifiers.clone(),
            }
        } ))
    ),

    /* Self Referencing reference */
    crate::make_parser_rule!(
        [ParserNode::LexerToken(TokenKind::SelfReferencing(non_terminals::SelfReferencing))] => Some(ParserNode::ObjectReference( {
            crate::ability_tree::object::ObjectReference::SelfReferencing
        } ))
    ),

    /* A count specifier as well as specifiers can be merged into an object reference */
    count_and_object_to_ref!(terminals::CountSpecifier::All),
    count_and_object_to_ref!(terminals::CountSpecifier::Target),
    count_and_object_to_ref!(terminals::CountSpecifier::AnyNumberOfTargets),
    count_and_object_to_ref!(terminals::CountSpecifier::Others),

    /* Manual rule creation, since we want to be able to match all "up to" */
    crate::parser::rules::ParserRule::create(
        crate::state_id!( [
            ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::UpTo(0))),
            ParserNode::ObjectSpecifiers(specifiers)
        ] ),
        crate::parser::node::ParserNodeKind::ObjectReference.id(),
        |tokens| match tokens {
            [
                ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::UpTo(up_to))),
                ParserNode::ObjectSpecifiers(specifiers)
            ] => {
                Some(ParserNode::ObjectReference(
                    crate::ability_tree::object::ObjectReference::SpecifiedObj {
                        amount: terminals::CountSpecifier::UpTo(*up_to),
                        specifiers: specifiers.clone(),
                    },
                ))
            },
            _ => None,
        },
        crate::parser::rules::ParserRuleDeclarationLocation {
            file: std::file!(),
            line: std::line!(),
        },
    ),
];
