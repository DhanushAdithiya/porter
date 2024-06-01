#![cfg(test)]
use crate::parser_css::*;
use std::{collections::HashMap, fs};

#[test]
fn parsing_keyword_arguments() {
    let input = String::from("display: flex;}");
    let mut css_parser = CssParser { pos: 0, input };
    let arg = css_parser.parse_argument();
    let output = HashMap::from([(String::from("display"), Value::Keyword("flex".to_string()))]);
    assert_eq!(arg, output);
}

#[test]
fn parsing_color_arguments() {
    let input = String::from("color: (200,189,200);}");
    let mut css_parser = CssParser { pos: 0, input };
    let arg = css_parser.parse_argument();
    let op_color = Color {
        r: 200,
        g: 189,
        b: 200,
    };
    let output = HashMap::from([(String::from("color"), Value::ColorValue(op_color))]);
    assert_eq!(arg, output);
}

#[test]
fn parsing_size_arguments() {
    let input = String::from("width: 20px;}");
    let mut css_parser = CssParser { pos: 0, input };
    let arg = css_parser.parse_argument();
    let output = HashMap::from([(String::from("width"), Value::Size(20.0, Unit::Px))]);
    assert_eq!(arg, output);
}

#[test]
fn parsing_key() {
    let input = String::from("display: flex;");
    let mut css_parser = CssParser { pos: 0, input };
    let key = css_parser.parse_key();
    assert_eq!(key, String::from("display"))
}

#[test]
fn parsing_keyword_value() {
    let input = String::from(": flex;");
    let mut css_parser = CssParser { pos: 0, input };
    let value = css_parser.parse_value();
    assert_eq!(value, Value::Keyword(String::from("flex")))
}

#[test]
fn parsing_color_value() {
    let input = String::from(": (200,189,200);");
    let mut css_parser = CssParser { pos: 0, input };
    let value = css_parser.parse_value();
    let op_color = Color {
        r: 200,
        g: 189,
        b: 200,
    };
    assert_eq!(value, Value::ColorValue(op_color));
}

#[test]
fn parsing_size_value() {
    let input = String::from(": 20px;");
    let mut css_parser = CssParser { pos: 0, input };
    let value = css_parser.parse_value();
    assert_eq!(value, Value::Size(20.0, Unit::Px));
}
