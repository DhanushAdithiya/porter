use std::fs::read_to_string;

pub mod dom;
pub mod parser;
pub mod parser_css;
pub mod parser_html;
pub mod tests;

fn main() {
    let css = read_to_string("style.css").unwrap();

    let stylesheet = parser_css::parse(css);
    println!("{:#?}", stylesheet);
}
