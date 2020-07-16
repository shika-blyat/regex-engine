pub mod nfa;
pub mod regex;
pub mod transition;

use nfa::{NFABuilder, NFA};
use regex::{Quantifier, Regex};

fn main() {
    let regex = Regex::Concat(
        Box::new(Regex::Quantified(
            Quantifier::Kleene,
            Box::new(Regex::Char('a')),
        )),
        Box::new(Regex::Char('b')),
    );
    /*
    let regex = Regex::Concat(
        Box::new(Regex::Char('a')),
        Box::new(Regex::Concat(
            Box::new(Regex::Char('b')),
            Box::new(Regex::Concat(
                Box::new(Regex::Char('c')),
                Box::new(Regex::Char('d')),
            )),
        )),
    );*/
    let builder = NFABuilder::new();
    let node = builder.to_nfa(regex);
    //  println!("{:#?}", node);
    println!("{}", node);
}
