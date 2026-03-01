use idris::Idris;

use crate::parser::ParserNode;

/// Errors that can be thrown by the parser.
#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedToken {
        found: FoundToken,
        expecting: Vec<PossibleExpectedToken>,
    },
    FailedToApplyRule {
        merge_error: &'static str,
        for_rule: crate::parser::rules::ParserRuleDeclarationLocation,
    },
    InvalidEarleyTable,  /* Fixme: this shall never happen I think */
    AmbiguousCandidates, /* Fixme: is this ever appears, we have problems ? */
}

impl ParserError {
    pub(super) fn from_earley_table(table: &super::EarleyTable, tokens: &[crate::lexer::tokens::Token]) -> Self {
        let error_row = table.table.iter().enumerate().rev().find(|(_, row)| !row.is_empty());
        let (stuck_index, last_non_empty_row) = match error_row {
            Some(error) => error,
            None => return Self::InvalidEarleyTable,
        };

        let stuck_on_token = match tokens.get(stuck_index) {
            Some(token) => FoundToken {
                name: ParserNode::name_from_id(ParserNode::from(*token).id()),
                position: token.span.start,
                length: token.span.length,
                text: token.span.text.to_string(),
            },
            None => FoundToken {
                name: "EOF",
                position: match tokens.last() {
                    Some(last) => last.span.start + last.span.length,
                    None => 0,
                },
                length: 0,
                text: "EOI".to_string(),
            },
        };

        /* Build a list of possible expected tokens (use a set to avoid repetitions) */
        let mut expecting = std::collections::HashSet::new();
        for (expecting_token, for_nodes) in last_non_empty_row.uncompleted_items.iter() {
            /* Only take in terminal tokens ? */
            use idris::Idris;
            if *expecting_token < crate::lexer::tokens::TokenKind::COUNT {
                expecting.insert(PossibleExpectedToken {
                    expected: *expecting_token,
                    for_nodes: for_nodes
                        .iter()
                        .map(|item| (item.rule.merged, item.rule.creation_loc.clone()))
                        .collect(),
                });
            }
        }

        Self::UnexpectedToken {
            found: stuck_on_token,
            expecting: expecting.into_iter().collect(),
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken { found, expecting } => {
                write!(
                    f,
                    "Unexpected token \"{}\" at position {}, length {}: \"{}\"",
                    found.name, found.position, found.length, found.text
                )?;
                if !expecting.is_empty() {
                    write!(f, "\nExpecting one of:")?;
                    for expecting in expecting.iter().take(10) {
                        let node_name = <ParserNode as idris::Idris>::name_from_id(expecting.expected);
                        write!(f, "\n - token \"{node_name}\" to create nodes")?;
                        for (i, (for_node, for_rule)) in expecting.for_nodes.iter().enumerate() {
                            let node_name = <ParserNode as idris::Idris>::name_from_id(*for_node);
                            if i == 0 {
                                write!(f, " \"{node_name}\" (at {for_rule})")?;
                            } else {
                                write!(f, ", \"{node_name}\" (at {for_rule})")?;
                            }
                        }
                    }
                    if expecting.len() > 10 {
                        write!(f, "\nAnd {} others", expecting.len() - 10)?;
                    }
                } else {
                    write!(f, "\nNo tokens were expected !")?;
                }
            }
            Self::FailedToApplyRule { merge_error, for_rule } => {
                write!(f, "Failed to use rule (declared at: {}): {}", for_rule, merge_error)?;
            }
            Self::InvalidEarleyTable => write!(f, "Empty Earley table !")?,
            Self::AmbiguousCandidates => write!(f, "Multiple candidates for rule completion !")?,
        }

        Ok(())
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PossibleExpectedToken {
    expected: usize,
    for_nodes: Vec<(usize, crate::parser::rules::ParserRuleDeclarationLocation)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FoundToken {
    name: &'static str,
    position: usize,
    length: usize,
    text: String,
}
