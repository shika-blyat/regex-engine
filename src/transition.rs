use crate::nfa::NodeHandle;

#[derive(Debug, Clone)]
pub struct Transition {
    pub consumes: char,
    pub to: NodeHandle,
}
