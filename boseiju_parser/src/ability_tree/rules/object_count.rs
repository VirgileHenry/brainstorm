use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [/* "target" is a single target object count */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::Target {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::CountSpecifier {
            count: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::Target {
                    #[cfg(feature = "spanned_tree")]
                        span: target_span,
                })),
            ] => Ok(ParserNode::CountSpecifier {
                count: boseiju_tree::ability_tree::object::CountSpecifier::Target(
                    boseiju_tree::ability_tree::number::Number::Number(boseiju_tree::ability_tree::number::FixedNumber {
                        number: 1,
                        #[cfg(feature = "spanned_tree")]
                        span: *target_span,
                    }),
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
