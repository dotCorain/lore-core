/// from literals to concepts as lines
pub trait Lexer {
    type Node;
    fn parse_line(&self, line: &str) -> Self::Node;
}

pub trait Parser {
    type Node;
    type Output;
    fn parse
}