use std::collections::HashMap;

use crate::parser::Parser;

#[derive(Debug)]
pub struct Stylesheet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    selectors: Vec<Selector>,
    arguments: Vec<Argument>,
}

#[derive(Debug)]
enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
struct SimpleSelector {
    html_tag: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

type Argument = HashMap<String, Value>;

#[derive(Debug, PartialEq)]
pub enum Value {
    Keyword(String),
    Size(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Unit {
    Px,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub struct CssParser {
    pub pos: usize,
    pub input: String,
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

pub fn parse(source: String) -> Stylesheet {
    let mut parser = CssParser {
        pos: 0,
        input: source,
    };

    Stylesheet {
        rules: parser.parse_rules(),
    }
}

impl CssParser {
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();

        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }

            rules.push(self.parse_rule());
        }

        return rules;
    }

    fn parse_rule(&mut self) -> Rule {
        return Rule {
            selectors: self.parse_selectors(),
            arguments: self.parse_arguments(),
        };
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {}
                '{' => break,
                c => panic!("CSS Parse Error: Invalid character {}", c),
            }
        }

        // TODO: Sort the selectors accoring to specificity
        return selectors;
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut simple_selector = SimpleSelector {
            html_tag: None,
            id: None,
            class: Vec::new(),
        };

        while !self.eof() {
            match self.next_char() {
                '.' => {
                    self.consume_char();
                    assert_eq!(self.consume_char(), '{');
                    simple_selector.class.push(self.parse_identifier());
                }
                '#' => {
                    self.consume_char();
                    simple_selector.id = Some(self.parse_identifier());
                }

                '*' => {
                    self.consume_char();
                }
                c if is_valid_tag(c) => {
                    simple_selector.html_tag = Some(self.parse_identifier());
                }
                _ => break,
            }
        }

        return simple_selector;
    }

    fn parse_identifier(&mut self) -> String {
        return self.consume_while(|c| c != '{');
    }

    fn parse_arguments(&mut self) -> Vec<Argument> {
        assert_eq!(self.consume_char(), '{');
        let mut arguments = Vec::new();

        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            arguments.push(self.parse_argument());
        }

        return arguments;
    }

    pub fn parse_argument(&mut self) -> Argument {
        let mut argument = HashMap::new();
        self.consume_whitespace();

        loop {
            let key = self.parse_key();
            let value = self.parse_value();
            argument.insert(key, value);
            if self.next_char() == ';' {
                self.consume_char();
                break;
            }
        }

        return argument;
    }

    pub fn parse_key(&mut self) -> String {
        let key = self.consume_while(is_valid_tag);
        return key;
    }

    pub fn parse_value(&mut self) -> Value {
        self.consume_char();
        self.consume_whitespace();
        match self.next_char() {
            c if c.is_digit(10) => {
                let digit = self.consume_while(|c| c != ';');
                Value::Size(
                    digit.parse::<f32>().expect("Could not parse size"),
                    Unit::Px,
                )
            }
            c if c.is_alphabetic() => Value::Keyword(self.consume_while(|c| c != ';')),
            '(' => {
                self.consume_char();
                let r = self
                    .consume_while(|c| c != ',')
                    .parse::<u8>()
                    .expect("Could not parse color");

                let g = self
                    .consume_while(|c| c != ',')
                    .parse::<u8>()
                    .expect("Could not parse color");

                let b = self
                    .consume_while(|c| c != ')')
                    .parse::<u8>()
                    .expect("Could not parse color");

                let color = Color { r, g, b };

                return Value::ColorValue(color);
            }
            _ => panic!(
                "Css Parse Error found {}",
                self.input.chars().nth(self.pos).unwrap()
            ),
        }
    }
}

fn is_valid_tag(c: char) -> bool {
    match c {
        'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '-' => true,
        _ => false,
    }
}
