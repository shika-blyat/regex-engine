use crate::{
    regex::{Quantifier, Regex},
    transition::Transition,
};
use std::{collections::HashSet, fmt};

pub type NodeHandle = usize;

#[derive(Debug)]
pub struct NFA {
    starting_node: NodeHandle,
    nodes: Vec<NFANode>,
}

impl fmt::Display for NFA {
    fn fmt(&self, fmtr: &'_ mut fmt::Formatter<'_>) -> fmt::Result {
        for trans in self.nodes[self.starting_node].transitions.iter() {
            write!(
                fmtr,
                "{} -'{}'> {}\n",
                self.starting_node,
                trans.consumes.unwrap_or('\0'),
                trans.to
            );
        }
        for (idx, node) in self.nodes.iter().enumerate() {
            if idx != self.starting_node {
                for trans in node.transitions.iter() {
                    write!(
                        fmtr,
                        "{} -'{}'> {}\n",
                        idx,
                        trans.consumes.unwrap_or('\0'),
                        trans.to,
                    );
                }
                if node.is_final {
                    write!(fmtr, "{} ->\n", idx);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct NFANode {
    pub transitions: Vec<Transition>,
    pub is_final: bool,
}

impl NFANode {
    pub fn new() -> Self {
        Self {
            transitions: vec![],
            is_final: true,
        }
    }
    pub fn insert(&mut self, trans: Transition) {
        self.is_final = false;
        self.transitions.push(trans)
    }
}

pub struct NFABuilder {
    nodes: Vec<NFANode>,
}

impl NFABuilder {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn to_nfa(mut self, regex: Regex) -> NFA {
        NFA {
            starting_node: self.to_nfa_internal(regex),
            nodes: self.nodes,
        }
    }

    fn to_nfa_internal(&mut self, regex: Regex) -> NodeHandle {
        match regex {
            Regex::Char(c) => {
                let mut node = NFANode::new();
                node.insert(Transition {
                    consumes: Some(c),
                    to: self.new_node(),
                });
                self.insert_node(node)
            }
            Regex::Concat(left, right) => {
                let left_handle = self.to_nfa_internal(*left);
                let right = self.to_nfa_internal(*right);
                // remove this clone
                let left = &mut self.nodes[left_handle];
                if left.is_final {
                    left.insert(Transition {
                        consumes: None,
                        to: right,
                    });
                }
                for node in self.get_all_final_states(left_handle) {
                    self.nodes[node].insert(Transition {
                        consumes: None,
                        to: right,
                    });
                }
                left_handle
            }
            Regex::Or(left, right) => {
                let left = self.to_nfa_internal(*left);
                let right = self.to_nfa_internal(*right);
                let new_node_handle = self.new_node();
                let new_node = &mut self.nodes[new_node_handle];
                new_node.insert(Transition {
                    consumes: None,
                    to: left,
                });
                new_node.insert(Transition {
                    consumes: None,
                    to: right,
                });
                new_node_handle
            }
            Regex::Quantified(Quantifier::Kleene, regex) => {
                let regex_handle = self.to_nfa_internal(*regex);
                let regex = &mut self.nodes[regex_handle];
                regex.is_final = true;
                for node in self.get_all_final_states(regex_handle) {
                    self.nodes[node].insert(Transition {
                        consumes: None,
                        to: regex_handle,
                    })
                }
                regex_handle
            }
        }
    }

    fn get_all_final_states(&self, handle: NodeHandle) -> HashSet<NodeHandle> {
        let mut final_states = HashSet::new();
        let mut function = |handle: NodeHandle| {};
        function(handle);
        final_states
    }
    fn get_all_final_states_internal(
        &self,
        final_states: &mut HashSet<NodeHandle>,
        handle: NodeHandle,
    ) {
        for trans in &self.nodes[handle].transitions {
            if self.nodes[trans.to].is_final && !final_states.contains(&trans.to) {
                final_states.insert(trans.to);
            }
            self.get_all_final_states_internal(final_states, handle);
        }
    }
    fn new_node(&mut self) -> NodeHandle {
        self.insert_node(NFANode::new())
    }

    fn insert_node(&mut self, node: NFANode) -> NodeHandle {
        self.nodes.push(node);
        self.nodes.len() - 1
    }
}
