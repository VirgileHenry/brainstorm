use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* Counter <spell> */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                token: intermediate::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Counter,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::BaseForm,
            }))
            .id(),
            ParserNode::Spell {
                spell: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::ImperativeKind {
            imperative: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::KeywordAction(intermediate::TensedKeywordAction {
                    token:
                        intermediate::KeywordAction {
                            keyword_action: mtg_data::KeywordAction::Counter,
                            #[cfg(feature = "spanned_tree")]
                                span: counter_span,
                        },
                    tense: boseiju_lexer::Tense::BaseForm,
                })),
                ParserNode::Spell { spell },
            ] => Ok(ParserNode::ImperativeKind {
                imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::KeywordAction(
                    boseiju_tree::ability_tree::imperative::KeywordAction {
                        keyword: boseiju_tree::ability_tree::imperative::ExpandedKeywordAction::Counter(
                            boseiju_tree::ability_tree::imperative::counter::CounterKeywordAction {
                                spell: spell.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: spell.span().merge(counter_span),
                            },
                        ),
                        ability: boseiju_tree::ability_tree::imperative::counter::ability(
                            spell,
                            #[cfg(feature = "spanned_tree")]
                            spell.span().merge(counter_span),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: spell.span().merge(counter_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
