use crate::parser::ParserNode;

/// Errors that can be thrown by the parser.
#[derive(Debug, Clone)]
pub struct ParserError {
    stuck_at: usize,
    stuck_at_length: usize,
    stuck_on: String,
    expecting: Vec<ExpectedToken>,
}

impl ParserError {
    pub(super) fn from_earley_table(table: &super::EarleyTable, tokens: &[crate::lexer::tokens::Token]) -> Self {
        let error_row = table.table.iter().enumerate().rev().find(|(_, row)| !row.items.is_empty());
        let (stuck_index, last_non_empty_row) = match error_row {
            Some(error) => error,
            None => {
                return Self {
                    stuck_at: 0,
                    stuck_at_length: 0,
                    stuck_on: "Invalid Earley table".to_string(),
                    expecting: Vec::with_capacity(0),
                };
            }
        };

        let stuck_on_token = match tokens.get(stuck_index) {
            Some(token) => token,
            None => {
                return Self {
                    stuck_at: 0,
                    stuck_at_length: 0,
                    stuck_on: "Invalid tokens for given Earley table".to_string(),
                    expecting: Vec::with_capacity(0),
                };
            }
        };

        let mut expecting = Vec::new();
        for item in last_non_empty_row.items.iter() {
            let expecting_token = match item.expecting_token() {
                Some(token) => token,
                None => continue,
            };
            /* Only take in terminal tokens ? */
            use idris::Idris;
            if expecting_token < crate::lexer::tokens::TokenKind::COUNT {
                expecting.push(ExpectedToken {
                    expected: expecting_token,
                    for_node: item.rule.merged,
                    for_rule: item.rule.creation_loc.clone(),
                });
            }
        }

        Self {
            stuck_at: stuck_on_token.span.start,
            stuck_at_length: stuck_on_token.span.length,
            stuck_on: stuck_on_token.span.text.to_string(),
            expecting,
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unexpected token at position {}, length {}: \"{}\"",
            self.stuck_at, self.stuck_at_length, self.stuck_on
        )?;
        if !self.expecting.is_empty() {
            write!(f, "\nExpecting one of:")?;
            for expecting in self.expecting.iter().take(10) {
                write!(
                    f,
                    "\n - token \"{}\" to create node \"{}\" (rule at {})",
                    <ParserNode as idris::Idris>::name_from_id(expecting.expected),
                    <ParserNode as idris::Idris>::name_from_id(expecting.for_node),
                    expecting.for_rule
                )?;
            }
            if self.expecting.len() > 10 {
                write!(f, "\nAnd {} others", self.expecting.len() - 10)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for ParserError {}

#[derive(Debug, Clone)]
struct ExpectedToken {
    expected: usize,
    for_node: usize,
    for_rule: crate::parser::rules::ParserRuleDeclarationLocation,
}
