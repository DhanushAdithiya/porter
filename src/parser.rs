pub trait Parser {
    fn next_char(&self) -> char;
    fn starts_with(&self, s: &str) -> bool;
    fn eof(&self) -> bool;
    fn consume_char(&mut self) -> char;
    fn consume_while<T>(&mut self, test: T) -> String
    where
        T: Fn(char) -> bool;
    fn consume_whitespace(&mut self);
}
