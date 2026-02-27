mod error;
mod node;
mod rule_map;
mod rules;

pub use error::ParserError;
pub use node::ParserNode;

use std::collections::HashSet;
use std::sync::Arc;

#[derive(Clone)]
enum EarleyBackpointer<'r> {
    Scanned(usize),
    Complete(Arc<EarleyItem<'r>>),
}

/// Earley Item.
///
/// From the Earley algorithm, an Earley item is an object that contains:
/// - A rule to match for. This item is responsible for trying to match that rule
/// on the given input stream, at the given start position.
/// - A start index, this is where the item started to match the tokens with
/// the given rule.
/// - a position index, indicating how many of the rule tokens have been matched already.
///
/// The rule is generally noted `(A -> a b . c, 2)` where the dot represent the position index.
/// In this example, the rule is `A -> a b c`, and we already matched `a b` from the token 2 and 3
/// of the input stream, and we are expecting a token `c` from the stream.
///
/// The debug implementation of the rule display it with this format.
#[derive(Clone)]
struct EarleyItem<'r> {
    pub rule: &'r rules::ParserRule,
    pub start_index: usize,
    pub position_index: usize,
    pub backpointers: Vec<EarleyBackpointer<'r>>,
}

impl<'r> EarleyItem<'r> {
    /// Construct a new Earley item with no backpointers.
    fn new(rule: &'r rules::ParserRule, start_index: usize, position_index: usize) -> Self {
        Self {
            rule,
            start_index,
            position_index,
            backpointers: Vec::with_capacity(rule.expanded.length.get()),
        }
    }

    /// This function gets the token that this rule is awaiting for progress.
    ///
    /// For this item of the form (A -> a . B b, i) get the B token id.
    fn expecting_token(&self) -> Option<usize> {
        self.rule.expanded.get(self.position_index).cloned()
    }

    /// Checks wheteher this item is of the form (A -> a . , i).
    /// In other terms, whether this rule fully matched the input or not.
    fn rule_complete(&self) -> bool {
        self.rule.expanded.length.get() == self.position_index
    }

    /// Checks whether this Earley Item is complete regarding a given node id.
    ///
    /// An item is considered complete if the input token stream is fully matched,
    /// that means that for a given rule that produces the target node id:
    /// - The rule result is the required node
    /// - The rule awaits no tokens, or the position index is at the end of the expanded tokens,
    /// - the rule starts at 0
    fn is_complete(&self, target_node_id: usize) -> bool {
        let is_axiom = self.rule.merged == target_node_id;
        let rule_complete = self.rule_complete();
        let start_at_0 = self.start_index == 0;
        is_axiom && rule_complete && start_at_0
    }

    /// Use the item rule to reduce the parser nodes.
    ///
    /// If the item contains any backpointers, they will be used to recursively
    /// call the rules required to merge everything together.
    fn reduce(&self, nodes: &[ParserNode]) -> Result<ParserNode, ParserError> {
        let rule_token_count = self.rule.expanded.length.get();

        let mut tokens_for_reduction = Vec::with_capacity(rule_token_count);
        for backpointer in self.backpointers.iter().take(rule_token_count) {
            match backpointer {
                EarleyBackpointer::Scanned(token_index) => tokens_for_reduction.push(nodes[*token_index].clone()),
                EarleyBackpointer::Complete(backpointer) => tokens_for_reduction.push(backpointer.reduce(nodes)?),
            }
        }

        match (self.rule.reduction)(&tokens_for_reduction) {
            Ok(node) => Ok(node),
            Err(merge_error) => Err(ParserError::FailedToApplyRule {
                merge_error,
                for_rule: self.rule.creation_loc.clone(),
            }),
        }
    }
}

impl<'r> std::fmt::Debug for EarleyItem<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use idris::Idris;
        write!(f, "({} -> ", ParserNode::name_from_id(self.rule.merged))?;
        for i in 0..self.position_index {
            write!(f, "{} ", ParserNode::name_from_id(self.rule.expanded[i]))?;
        }
        write!(f, ".")?;
        for i in self.position_index..self.rule.expanded.length.get() {
            write!(f, " {}", ParserNode::name_from_id(self.rule.expanded[i]))?;
        }
        write!(f, ", {})", self.start_index)?;
        Ok(())
    }
}

impl<'r> PartialEq for EarleyItem<'r> {
    fn eq(&self, other: &Self) -> bool {
        self.rule == other.rule && self.start_index == other.start_index && self.position_index == other.position_index
    }
}

impl<'r> Eq for EarleyItem<'r> {}

impl<'r> std::hash::Hash for EarleyItem<'r> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rule.hash(state);
        self.start_index.hash(state);
        self.position_index.hash(state);
    }
}

/// Earley row.
///
/// An Earley row is a set of [`EarleyItem`] that are stored at a single row
/// in the Early algorithm. That is, any T\[i\] is an Earley row.
///
/// The core idea of the algorithm is that in the Earley table at index j,
/// the Earley row j contains all the rules that are potential matches for the
/// incoming token stream.
#[derive(Clone)]
struct EarleyRow<'r> {
    pub items: HashSet<Arc<EarleyItem<'r>>>,
}

impl<'r> EarleyRow<'r> {
    /// Creates a new, empty row.
    fn new() -> Self {
        Self { items: HashSet::new() }
    }

    /// Create the start row of the Earley Table for an algorithm that is targetting
    /// the Ability Tree node.
    fn start_row(rules: &'static rule_map::RuleMap) -> EarleyRow<'static> {
        use crate::utils::dummy;
        use idris::Idris;

        let mut start_row = EarleyRow::new();

        let target_node = ParserNode::AbilityTree { tree: dummy() };
        let node_id = target_node.id();

        for rule in rules.get_rules_for_token(node_id) {
            start_row.items.insert(Arc::new(EarleyItem::new(rule, 0, 0)));
        }

        let mut queue = start_row.items.iter().cloned().collect::<Vec<_>>();

        while let Some(item) = queue.pop() {
            for new_item in predictor_step(&rules, 0, &item).into_iter() {
                if start_row.items.insert(Arc::new(new_item.clone())) {
                    queue.push(Arc::new(new_item.clone()));
                }
            }
        }

        start_row
    }
}

impl<'r> std::fmt::Debug for EarleyRow<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.items.is_empty() {
            write!(f, "[]")?;
        } else {
            writeln!(f, "[")?;
            for item in self.items.iter() {
                writeln!(f, "  {item:?}")?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

/// Earley Table.
///
/// The Earley table is the object constructed with the Earley parsing algorithm.
/// It contains as many rows as there are tokens (plus one) and keep track of the rules
/// that can match the provided input stream.
#[derive(Clone)]
struct EarleyTable<'r> {
    pub table: Vec<EarleyRow<'r>>,
}

impl<'r> EarleyTable<'r> {
    pub fn new(node_count: usize, start_row: EarleyRow<'r>) -> Self {
        let mut table = Vec::with_capacity(node_count + 1);
        table.push(start_row);

        Self { table }
    }
}

impl<'r> std::fmt::Debug for EarleyTable<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.table.iter().enumerate() {
            writeln!(f, "T[{i}] = {row:?}")?;
        }
        Ok(())
    }
}

impl<'r> std::ops::Deref for EarleyTable<'r> {
    type Target = [EarleyRow<'r>];
    fn deref(&self) -> &Self::Target {
        self.table.as_slice()
    }
}

/// Attempts to parse a sequence of nodes into an ability tree, using the Earley parsing algorithm.
///
/// The algorithm reference can be found here: https://en.wikipedia.org/wiki/Earley_parser
/// The algorithm used for the implementation was: https://fr.wikipedia.org/wiki/Analyse_Earley (cocorico)
fn parse_impl(tokens: &[crate::lexer::tokens::Token]) -> Result<crate::AbilityTree, error::ParserError> {
    use crate::utils::dummy;
    use idris::Idris;

    /* Rule map, all our parsing rules in a single struct */
    lazy_static::lazy_static!(
        /// The rule map contains all the rules to parse the MTG cards.
        static ref rules: rule_map::RuleMap = rule_map::RuleMap::default().expect("Default Rule Map shall be OK");

        /// A first row for the earley parsing algorithm with the target of parsing a full ability tree.
        ///
        /// Since the row depends only on the rule map, we can create a static instance of
        /// it and clone it whenever we start a new parsing, instead of rebuilding it each time.
        ///
        /// Fixme: I think we can construct it each time it's fine, and it gives us control over the target
        static ref earley_start_row: EarleyRow<'static> = EarleyRow::start_row(&rules);
    );

    /* Implementation of the Earley parser */
    let target_node_id = ParserNode::AbilityTree { tree: dummy() }.id();
    let nodes: Vec<ParserNode> = tokens.iter().cloned().map(ParserNode::from).collect();

    /* First step: init the table row 0 with all rules that can create the final token */
    let node_count = nodes.len();
    let mut earley_table = EarleyTable::new(node_count, earley_start_row.clone());

    for (node_index, node) in nodes.iter().enumerate() {
        use idris::Idris;
        let node_id = node.id();
        let j = node_index + 1;

        /* Create the next Earley table entry, T[j]  */
        let mut next_table_row = EarleyRow::new();

        /* Scanner step */
        /* for all items in the previous row, add them to this row if they match the current token */
        scanner_step(&earley_table.table[node_index], node_id, node_index, &mut next_table_row);

        /* The queue allows to process each item once, seeing what other items are added */
        let mut queue = next_table_row.items.iter().cloned().collect::<Vec<_>>();

        /* Saturate Predictor + Completor steps */
        while let Some(item) = queue.pop() {
            /* Predictor step */
            for new_item in predictor_step(&rules, j, &item).into_iter() {
                /* Add the new item in the queue for eploring only if it is unknown (not in the row) */
                if next_table_row.items.insert(Arc::new(new_item.clone())) {
                    queue.push(Arc::new(new_item.clone()));
                }
            }
            /* Completor step */
            for new_item in completor_step(&earley_table, item).into_iter() {
                /* Add the new item in the queue for eploring only if it is unknown (not in the row) */
                if next_table_row.items.insert(Arc::new(new_item.clone())) {
                    queue.push(Arc::new(new_item.clone()));
                }
            }
        }

        earley_table.table.push(next_table_row);
    }

    /* Look for an item of kind (S -> a . , 0) for parser completion */
    let completed_items = earley_table.table[node_count]
        .items
        .iter()
        .filter(|item| item.is_complete(target_node_id))
        .collect::<Vec<_>>();

    match completed_items.as_slice() {
        /* No item completed, create a parse error from the earley table */
        &[] => Err(error::ParserError::from_earley_table(&earley_table, tokens)),
        /* A single item is complete: we have a condidate for merging */
        &[complete_item] => match complete_item.reduce(&nodes)? {
            ParserNode::AbilityTree { tree } => return Ok(tree),
            _ => unreachable!(),
        },
        _ => Err(ParserError::AmbiguousCandidates),
    }
}

/// Scanner step of the Earley Algorithm.
///
/// The goal of this step is to consume a new token from the input token stream,
/// and feed it to all the rules that are current matches to advance those rules.
///
/// All the rules in the row T\[j-1\] that were expecting the token are bumped
/// into T\[j\] with their position index advanced.
///
/// The awaited non terminals are handled in the predictor step.
fn scanner_step<'r>(prev_row: &EarleyRow<'r>, token: usize, token_index: usize, next_row: &mut EarleyRow<'r>) {
    for item in prev_row.items.iter() {
        if item.expecting_token() == Some(token) {
            use std::ops::Deref;

            let mut new_item: EarleyItem = item.deref().clone();
            new_item.position_index += 1;
            new_item.backpointers.push(EarleyBackpointer::Scanned(token_index));

            next_row.items.insert(Arc::new(new_item));
        }
    }
}

/// Predictor step of the Earley Algorithm.
///
/// The goal of this step is to satisfy all the rules that are currently awaiting a non terminal token.
/// Since the scanner step can only feed terminals, for each awaited non terminal in a rule,
/// we add that rule to the current row, with the start position set at the current index.
///
/// Then, the rule completion logic is handled in the completor step.
///
/// This function returns whether new items were added to the next row.
fn predictor_step<'r>(
    rules: &'r rule_map::RuleMap,
    j: usize,
    item: &EarleyItem<'r>,
) -> std::collections::HashSet<EarleyItem<'r>> {
    let mut new_items = std::collections::HashSet::new();
    let next_token = match item.expecting_token() {
        Some(token) => token,
        None => return new_items,
    };

    for rule in rules.get_rules_for_token(next_token) {
        let item = EarleyItem::new(rule, j, 0);
        new_items.insert(item);
    }

    new_items
}

/// Completor step of the Earley Algorithm.
///
/// If we complete a rule in the current row, we need to look back to all the rows
/// that were awaiting this rule to complete to advance it.
///
/// For a given rule (A -> a . , i) that is compleated, we can look in T[\i\] (since that
/// rule started at i) for any rule that was awaiting for an A non terminal, and advance
/// it in the current row.
///
/// This function returns whether new items were added to the next row.
fn completor_step<'r>(
    earley_table: &EarleyTable<'r>,
    completed_item: Arc<EarleyItem<'r>>,
) -> std::collections::HashSet<EarleyItem<'r>> {
    let mut new_items = std::collections::HashSet::new();

    if completed_item.rule_complete() {
        /* If so, check in T[i] for all rules awaiting this item */
        for awaiting_item in earley_table.table[completed_item.start_index].items.iter() {
            if awaiting_item.expecting_token() == Some(completed_item.rule.merged) {
                /* And we can bump these rules up */
                use std::ops::Deref;
                let mut item: EarleyItem = awaiting_item.deref().clone();
                item.position_index += 1;
                /* Set the backpointer, since the current row item validated this token */
                item.backpointers.push(EarleyBackpointer::Complete(completed_item.clone()));

                new_items.insert(item);
            }
        }
    }

    new_items
}

/// Parser function without artifacts, to get the result straight out.
/// See [parse_impl] for a detailed explanation of the algorithm.
pub fn parse(tokens: &[crate::lexer::tokens::Token]) -> Result<crate::AbilityTree, error::ParserError> {
    parse_impl(tokens)
}
