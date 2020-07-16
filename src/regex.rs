#[derive(Debug)]
pub enum Regex {
    Concat(Box<Regex>, Box<Regex>),
    Or(Box<Regex>, Box<Regex>),
    Kleene(Box<Regex>),
    Char(char),
}
