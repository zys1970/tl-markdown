use wasm_bindgen::prelude::*;
use parser::parse_markdown;

mod ast;
mod parser;

#[wasm_bindgen]
pub fn parse_markdown_wasm(input: &str) -> JsValue {
    let ast = parse_markdown(input);
    serde_wasm_bindgen::to_value(&ast).unwrap()
}
