#![cfg(test)]
use std::{collections::HashMap, fs::read_to_string};

#[test]
fn parsing_arguments1() {
    use crate::parser_css::*;

    let input = read_to_string("src/tests/css/test1.css").expect("could not read input");
    let mut css_parser = CssParser { pos: 0, input };
    let arg = css_parser.parse_argument();
    let mut hs = HashMap::from([(String::from("display"), Value::Keyword("flex".to_string()))]);
    assert_eq!(arg, hs);
}
