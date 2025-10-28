pub mod error;
mod node;
mod rules;

/// The parser state represents a single node in a graph where all nodes are arrays of tokens.
/// Nodes are connected to each other when a single rule application allows to fuse a sub slice
/// of the tokens together, creating a smaller array of tokens. For example:
/// with the rule `B, C -> E`, we have the two nodes `[A, B, C, D] -> [A, E, D]` connected.
///
/// The goal is to explore this graph, and find the final node as quickly as possible.
/// Or if there is no path to a terminal node, fail as fast as possible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserState {
    nodes: arrayvec::ArrayVec<node::ParserNode, 128>,
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

struct StateBag(std::collections::BTreeMap<usize, Vec<ParserState>>);

impl StateBag {
    fn new() -> Self {
        StateBag(std::collections::BTreeMap::new())
    }

    fn insert(&mut self, state: ParserState) {
        match self.0.get_mut(&state.nodes.len()) {
            Some(bucket) => bucket.push(state),
            None => {
                self.0.insert(state.nodes.len(), vec![state]);
            }
        }
    }

    fn contains(&self, state: &ParserState) -> bool {
        match self.0.get(&state.nodes.len()) {
            Some(bucket) => bucket.contains(state),
            None => false,
        }
    }

    fn pop_first(&mut self) -> Option<ParserState> {
        let mut entry = self.0.first_entry()?;
        let result = entry.get_mut().pop()?;
        if entry.get().is_empty() {
            entry.remove();
        }
        Some(result)
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
pub fn parse(tokens: &[crate::lexer::tokens::Token]) -> (Result<crate::AbilityTree, error::ParserError>, usize) {
    /* Initialize the nodes from the tokens */
    let nodes: arrayvec::ArrayVec<node::ParserNode, 128> = tokens.iter().cloned().map(node::ParserNode::from).collect();

    let mut best_error = error::ParserError {
        nodes: nodes.clone(),
        best_attempt: nodes.clone(),
    };
    let mut iters = 0;

    /* The state to explore starts with the list of provided tokens */
    let mut states_to_explore = Vec::new();
    states_to_explore.push(ParserState { nodes });
    /* And the list of explored states starts empty */
    let mut states_explored = std::collections::HashSet::new();

    /* Keep looping until we have nodes that need to be explored */
    while let Some(to_explore) = states_to_explore.pop() {
        /* Flag to keep track of whetever the node is a sink or not */
        let mut explorable = false;
        /* Get all next possible states */
        for token_count in (1..=to_explore.nodes.len()).rev() {
            for (offset, window) in to_explore.nodes.windows(token_count).enumerate() {
                iters += 1;
                if let Some(fused) = rules::fuse(window) {
                    /* We managed to fuse the window into a new token! */
                    explorable = true;

                    /* Create the concatenated node array */
                    let mut nodes = arrayvec::ArrayVec::<_, 128>::new();
                    nodes.extend(to_explore.nodes.iter().take(offset).cloned());
                    nodes.push(fused);
                    nodes.extend(to_explore.nodes.iter().skip(offset + token_count).cloned());
                    let next_node = ParserState { nodes };

                    /* Skip if we already explored the node */
                    if states_explored.contains(&next_node) {
                        continue;
                    }

                    /* If the resulting node is a single ability tree, we found the result, return */
                    match next_node.nodes.as_slice() {
                        [node::ParserNode::AbilityTree(tree)] => return (Ok(*tree.clone()), iters),
                        _ => {
                            states_to_explore.push(next_node);
                        }
                    }
                }
            }
        }

        /* If the node could not be explored at all, it's a potential error */
        if !explorable {
            if to_explore.nodes.len() <= best_error.best_attempt.len() {
                best_error.best_attempt = to_explore.nodes.clone();
            }
        }

        /* Anyway, mark the node as explored */
        states_explored.insert(to_explore);
    }

    (Err(best_error), iters)
}

pub struct Edge;

impl std::fmt::Display for Edge {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// Attemtps to parse a card, but also generates the pentgraph of explored nodes for debbugging.
/// This is not meant for production, only for debug purpuses!
pub fn parse_and_generate_graph_vis(tokens: &[crate::lexer::tokens::Token]) -> petgraph::Graph<ParserState, Edge> {
    /* Initialize the nodes from the tokens */
    let nodes: arrayvec::ArrayVec<node::ParserNode, 128> = tokens.iter().cloned().map(node::ParserNode::from).collect();

    let mut best_error = error::ParserError {
        nodes: nodes.clone(),
        best_attempt: nodes.clone(),
    };

    let mut graph = petgraph::Graph::new();
    graph.add_node(ParserState { nodes: nodes.clone() });

    /* The state to explore starts with the list of provided tokens */
    let mut states_to_explore = Vec::new();
    states_to_explore.push(ParserState { nodes });
    /* And the list of explored states starts empty */
    let mut states_explored = std::collections::HashSet::new();

    /* Keep looping until we have nodes that need to be explored */
    while let Some(to_explore) = states_to_explore.pop() {
        /* Flag to keep track of whetever the node is a sink or not */
        let mut explorable = false;
        /* Get all next possible states */
        for token_count in (1..=to_explore.nodes.len()).rev() {
            for (offset, window) in to_explore.nodes.windows(token_count).enumerate() {
                if let Some(fused) = rules::fuse(window) {
                    /* We managed to fuse the window into a new token! */
                    explorable = true;

                    /* Create the concatenated node array */
                    let mut nodes = arrayvec::ArrayVec::<_, 128>::new();
                    nodes.extend(to_explore.nodes.iter().take(offset).cloned());
                    nodes.push(fused);
                    nodes.extend(to_explore.nodes.iter().skip(offset + token_count).cloned());
                    let next_node = ParserState { nodes };

                    let from_index = graph.node_indices().find(|&i| graph[i] == to_explore).unwrap();
                    let to_index = match graph.node_indices().find(|&i| graph[i] == next_node) {
                        Some(index) => index,
                        None => graph.add_node(next_node.clone()),
                    };
                    graph.add_edge(from_index, to_index, Edge);

                    /* Skip if we already explored the node */
                    if states_explored.contains(&next_node) {
                        continue;
                    }

                    /* If the resulting node is a single ability tree, we found the result, return */
                    match next_node.nodes.as_slice() {
                        [node::ParserNode::AbilityTree(_)] => return graph,
                        _ => {
                            states_to_explore.push(next_node);
                        }
                    }
                }
            }
        }

        /* If the node could not be explored at all, it's a potential error */
        if !explorable {
            if to_explore.nodes.len() <= best_error.best_attempt.len() {
                best_error.best_attempt = to_explore.nodes.clone();
            }
        }

        /* Anyway, mark the node as explored */
        states_explored.insert(to_explore);
    }

    graph
}
