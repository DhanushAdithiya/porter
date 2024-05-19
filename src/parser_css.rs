use crate::parser::Parser;

struct Stylesheet {
    rules: Vec<Rule>,
}

struct Rule {
    selectors: Vec<Selector>,
    arguments: Vec<Argument>,
}

enum Selector {
    Simple(SimpleSelector),
}

struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Option<String>,
}

struct Argument {
    key: String,
    values: Value,
}

enum Value {
    Keyword(String),
    Size(f32, Unit),
    ColorValue(Color),
}

enum Unit {
    Px,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct CssParser {
    pos: usize,
    input: String,
}

impl Parser for CssParser {
    fn next_char(&self) -> char {
        return self.input[self.pos..].chars().next().unwrap();
    }

    fn starts_with(&self, s: &str) -> bool {
        return self.input[self.pos..].starts_with(s);
    }

    fn eof(&self) -> bool {
        return self.pos >= self.input.len();
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));

        self.pos += next_pos;
        return cur_char;
    }

    fn consume_while<T>(&mut self, test: T) -> String
    where
        T: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}

impl CssParser {}
