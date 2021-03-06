use crate::nfa::NodeHandle;

#[derive(Debug, Clone)]
pub struct Transition {
    pub consumes: Option<char>,
    pub to: NodeHandle,
}
