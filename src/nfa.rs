use crate::{regex::Regex, transition::Transition};
use std::fmt;

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
                self.starting_node, trans.consumes, trans.to
            );
        }
        for (idx, node) in self.nodes.iter().enumerate() {
            if idx != self.starting_node {
                for trans in node.transitions.iter() {
                    write!(fmtr, "{} -'{}'> {}\n", idx, trans.consumes, trans.to,);
                }
                if node.is_final {
                    write!(fmtr, "{} ->", idx);
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
                    consumes: c,
                    to: self.new_node(NFANode::new()),
                });
                self.new_node(node)
            }
            Regex::Concat(left, right) => {
                let left = self.to_nfa_internal(*left);
                let right = self.to_nfa_internal(*right);
                // remove this clone
                for trans in self.nodes[left].clone().transitions.iter() {
                    let to = &mut self.nodes[trans.to];
                    if to.is_final {
                        to.insert(Transition {
                            consumes: '\0',
                            to: right,
                        });
                    }
                }
                left
            }
            Regex::Or(left, right) => match *left {
                Regex::Char(c) => {
                    let mut node = NFANode::new();
                    node.insert(Transition {
                        consumes: c,
                        to: self.to_nfa_internal(*right),
                    });
                    self.new_node(node)
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
    fn new_node(&mut self, node: NFANode) -> NodeHandle {
        self.nodes.push(node);
        self.nodes.len() - 1
    }
}
