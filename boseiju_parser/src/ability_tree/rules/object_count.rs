use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [/* "target" is a single target object count */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::CountSpecifier { count: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target {
                    #[cfg(feature = "spanned_tree")]
                        span: target_span,
                })),
            ] => Ok(ParserNode::CountSpecifier {
                count: crate::ability_tree::object::CountSpecifier::Target(crate::ability_tree::number::Number::Number(
                    crate::ability_tree::number::FixedNumber {
                        number: 1,
                        #[cfg(feature = "spanned_tree")]
                        span: *target_span,
                    },
                )),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
