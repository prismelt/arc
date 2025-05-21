use super::token::Token;

pub trait LexerTrait {
    fn new(source: String) -> Self;
    fn tokenize(self) -> Vec<Token>;
    fn reminder(&self) -> &str;
    fn advance_n(&mut self, n: usize);
    fn push(&mut self, token: Token);
}
