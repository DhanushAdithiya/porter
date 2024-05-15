struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        return self.input[self.pos..].chars().next().unwrap();
    }

    fn starts_with(&self, s: &str) -> bool {
        return self.input[self.pos..].starts_with(s);
    }
}
