//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn parse_header(){
    let source = "# Header".to_string();
    let obj = markdown_rs::parse(source);

    assert_eq!(obj, "<h1>Header</h1>")
}

#[wasm_bindgen_test]
fn parse_paragraph(){
    let source = "Paragraph".to_string();
    let obj = markdown_rs::parse(source);

    assert_eq!(obj, "<p>Paragraph</p>")
}

#[wasm_bindgen_test]
fn parse_header_and_paragraph(){
    let source = r#"
# Header
Paragraph

## Header 2
Paragraph 2
"#.trim().to_string();
    let obj = markdown_rs::parse(source);

    assert_eq!(obj, "<h1>Header</h1><p>Paragraph<br></p><h2>Header 2</h2><p>Paragraph 2</p>")
}

#[wasm_bindgen_test]
fn parse_paragraph_with_style(){
    let source = r#"**Lorem ipsum** dolor **sit** amet, consectetur adipiscing elit        "#.to_string();
    let obj = markdown_rs::parse(source);

    assert_eq!(obj, "<p><strong>Lorem ipsum</strong> dolor <strong>sit</strong> amet, consectetur adipiscing elit</p>")
}

#[wasm_bindgen_test]
fn parse_link(){
    let source = "Click [here](https://google.com) to go to the google home page.".to_string();
    let obj = markdown_rs::parse(source);

    assert_eq!(obj, "<p>Click <a href=\"https://google.com\" target=\"_blank\">here</a> to go to the google home page.</p>")
}