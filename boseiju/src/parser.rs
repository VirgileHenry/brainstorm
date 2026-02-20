pub mod error;
mod node;
mod rule_map;
mod rules;

pub use node::ParserNode;

/// The parser state represents a single node in a graph where all nodes are arrays of tokens.
/// Nodes are connected to each other when a single rule application allows to fuse a sub slice
/// of the tokens together, creating a smaller array of tokens. For example:
/// with the rule `B, C -> E`, we have the two nodes `[A, B, C, D] -> [A, E, D]` connected.
///
/// The goal is to explore this graph, and find the final node as quickly as possible.
/// Or if there is no path to a terminal node, fail as fast as possible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserState {
    nodes: Vec<ParserNode>,
}

/// Display implementation thats is use for the petgraph debugging, no real prod use case.
impl std::fmt::Display for ParserState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for node in self.nodes.iter().take(self.nodes.len().saturating_sub(1)) {
            writeln!(f, "{node:?}")?;
        }
        if let Some(node) = self.nodes.last() {
            writeln!(f, "{node:?}")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

/// Attempts to parse a sequence of nodes into an ability tree, exploring the graph of [ParserState] as fast as possible.
///
/// With our heuristic plugged into the Ord impl of the struct, we can use a BTreeSet.
/// The core idea is to keep track of every unexplored node, and explore it.
/// The BTreeSet allows to cheaply pop the node with the smallest heuristic, so the "closer" to completion.
/// We explore the node, and add every new node to the list of nodes to explore.
/// Again, the BTreeSet makes the insertion into the sorted list fast.
///
/// Furthermore, we can keep track of explored nodes to avoid exploring them again.
/// It's unlikely the algorithm will loop, but multiple paths can lead to the same nodes.
fn parse_impl<F: FnMut(&ParserState, &ParserState), G: FnMut(&[ParserNode])>(
    tokens: &[crate::lexer::tokens::Token],
    mut on_node_explored: F,
    mut on_fuse_attempt: G,
) -> Result<crate::AbilityTree, error::ParserError> {
    /* Rule map, all our parsing rules in a single struct */
    lazy_static::lazy_static!(
        static ref rules: rule_map::RuleMap = rule_map::RuleMap::default().expect("Default Rule Map shall be OK");
    );

    /* We don't need to do any parsing if there are no tokens */
    if tokens.is_empty() {
        return Ok(crate::AbilityTree::empty());
    }

    /* Initialize the nodes from the tokens */
    let nodes: Vec<ParserNode> = tokens.iter().cloned().map(ParserNode::from).collect();
    token_precedence_check(&rules, &nodes)?;

    let mut best_error: Option<error::ParserError> = None;

    /* The state to explore starts with the list of provided tokens */
    let mut states_to_explore = Vec::new();
    states_to_explore.push(ParserState { nodes });
    /* And the list of explored states starts empty */
    let mut states_explored = std::collections::HashSet::new();

    /* Keep looping until we have nodes that need to be explored */
    while let Some(to_explore) = states_to_explore.pop() {
        /* Get all next possible states */
        let mut next_states = Vec::new();

        /* At most, we iterate over the biggest number of tokens in any rule */
        let max_tokens_per_rule = to_explore.nodes.len().min(rules.max_rule_size());

        for token_count in (1..=max_tokens_per_rule).rev() {
            for (offset, window) in to_explore.nodes.windows(token_count).enumerate() {
                on_fuse_attempt(window);
                if let Some(fused) = rules.fuse(window) {
                    /* Create the concatenated node array */
                    let mut nodes = Vec::<_>::with_capacity(to_explore.nodes.len());
                    nodes.extend(to_explore.nodes.iter().take(offset).cloned());
                    nodes.push(fused);
                    nodes.extend(to_explore.nodes.iter().skip(offset + token_count).cloned());
                    let next_node = ParserState { nodes };

                    /* New node explored, call the user function */
                    on_node_explored(&to_explore, &next_node);

                    /* Exit condition: there is only a single token, the full tree */
                    match next_node.nodes.as_slice() {
                        [ParserNode::AbilityTree { tree }] => return Ok(*tree.clone()),
                        _ => {}
                    }

                    if states_explored.contains(&next_node) {
                        continue;
                    }

                    /* If anywhere in the new node, two consecutive tokens are not allowed, stop */
                    if let Err(error) = token_precedence_check(&rules, &next_node.nodes) {
                        best_error = Some(error.keep_best_error(best_error))
                    }

                    next_states.push(next_node);
                }
            }
        }

        states_to_explore.append(&mut next_states);

        /* sort the nodes to explore ? We would need a nice heuristic for this */
        states_to_explore.sort_by(|a, b| b.nodes.len().cmp(&a.nodes.len()));

        /* Anyway, mark the node as explored */
        states_explored.insert(to_explore);
    }

    Err(match best_error {
        Some(error) => error,
        None => error::ParserError::UnparsableInput {
            nodes: tokens.iter().map(|t| t.kind.clone()).collect(),
        },
    })
}

fn token_precedence_check(rules: &rule_map::RuleMap, nodes: &[ParserNode]) -> Result<(), error::ParserError> {
    for window in nodes.windows(2) {
        let [current, next] = window else { unreachable!() };
        if !rules.can_succeed(current, next) {
            return Err(error::ParserError::UnexpectedFollowingToken {
                current: current.clone(),
                next: next.clone(),
                current_best: nodes.to_vec(),
            });
        }
    }
    Ok(())
}

/// Parser function without artifacts, to get the result straight out.
/// See [parse_impl] for a detailed explanation of the algorithm.
pub fn parse(tokens: &[crate::lexer::tokens::Token]) -> Result<crate::AbilityTree, error::ParserError> {
    parse_impl(tokens, |_, _| {}, |_| {})
}

pub struct Edge(usize);

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Attemtps to parse a card, but also generates the pentgraph of explored nodes for debbugging.
pub fn parse_and_generate_graph_vis(tokens: &[crate::lexer::tokens::Token]) -> petgraph::Graph<ParserState, Edge> {
    /* Initialize the graphs, add first node */
    let mut graph = petgraph::Graph::new();
    let nodes: Vec<ParserNode> = tokens.iter().cloned().map(ParserNode::from).collect();
    graph.add_node(ParserState { nodes: nodes.clone() });

    /* Use a counter to keep track of the order of exploration */
    let mut exploration_counter = 0;

    /* Call the parser, with a function that will update the graph as it runs */
    let _ = parse_impl(
        tokens,
        |from_node, to_node| {
            let from_index = graph.node_indices().find(|&i| graph[i].eq(from_node)).unwrap();
            let to_index = match graph.node_indices().find(|&i| graph[i].eq(to_node)) {
                Some(index) => index,
                None => graph.add_node(to_node.clone()),
            };
            exploration_counter += 1;
            graph.add_edge(from_index, to_index, Edge(exploration_counter));
        },
        |_| {},
    );

    graph
}

/// Parse an ability tree, but counts the number of successeful token fusion performed.
/// This gives an indicator on how "good" our parsing is: the lesser the better.
pub fn parse_and_count_iters(tokens: &[crate::lexer::tokens::Token]) -> (Result<crate::AbilityTree, error::ParserError>, usize) {
    let mut fuse_count = 0;
    let result = parse_impl(tokens, |_, _| {}, |_| fuse_count += 1);
    (result, fuse_count)
}
