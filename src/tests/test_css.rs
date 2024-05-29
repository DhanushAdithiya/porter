#![cfg(test)]
use crate::parser_css::*;
use std::{collections::HashMap, fs::read_to_string};

#[test]
fn parsing_arguments() {
    let input = String::from("display: flex;");
    let mut css_parser = CssParser { pos: 0, input };
    let arg = css_parser.parse_argument();
    let hs = HashMap::from([(String::from("display"), Value::Keyword("flex".to_string()))]);
    assert_eq!(arg, hs);
}

#[test]
fn parsing_key() {
    let input = String::from("display: flex;");
    let mut css_parser = CssParser { pos: 0, input };
    let key = css_parser.parse_key();
    assert_eq!(key, String::from("display"))
}

#[test]
fn parsing_value() {
    let input = String::from(": flex;");
    let mut css_parser = CssParser { pos: 0, input };
    let value = css_parser.parse_value();
    assert_eq!(value, Value::Keyword(String::from("flex")))
}
