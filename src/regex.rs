#[derive(Debug)]
pub enum Regex {
    Concat(Box<Regex>, Box<Regex>),
    Or(Box<Regex>, Box<Regex>),
    Quantified(Quantifier, Box<Regex>),
    Char(char),
}

#[derive(Debug)]
pub enum Quantifier {
    Kleene,
}
