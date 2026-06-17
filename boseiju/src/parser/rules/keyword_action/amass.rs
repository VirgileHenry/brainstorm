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
    /* Amass <creature subtype> <amount> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                keyword_action: mtg_data::KeywordAction::Amass,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::CreatureSubtype { subtype: dummy() }.id(),
            ParserNode::Number { number: dummy() }.id(),
        ]),
        merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Amass,
                    #[cfg(feature = "spanned_tree")]
                        span: amass_span,
                })),
                ParserNode::CreatureSubtype { subtype },
                ParserNode::Number { number },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: crate::ability_tree::imperative::ImperativeKind::KeywordAction(
                    crate::ability_tree::imperative::KeywordAction {
                        keyword: crate::ability_tree::imperative::ExpandedKeywordAction::Amass(
                            crate::ability_tree::imperative::amass::AmassKeywordAction {
                                creature_subtype: subtype.clone(),
                                amount: number.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: number.node_span().merge(amass_span),
                            },
                        ),
                        ability: crate::ability_tree::imperative::amass::ability(
                            subtype,
                            number,
                            #[cfg(feature = "spanned_tree")]
                            number.node_span().merge(amass_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: number.node_span().merge(amass_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
