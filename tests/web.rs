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

#[wasm_bindgen_test]
fn parse_image(){
    let url = "https://www.google.com/url?sa=i&url=https%3A%2F%2Ffr.wikipedia.org%2Fwiki%2FGoogle_Images&psig=AOvVaw2iuU2SDmWEqDXDXeTVyA4f&ust=1709134258767000&source=images&cd=vfe&opi=89978449&ved=0CBIQjRxqFwoTCOCL-rTry4QDFQAAAAAdAAAAABAE";
    let source = format!("Here is the logo of google Images: ![google image]({url})");
    let obj = markdown_rs::parse(source);

    assert_eq!(obj, format!(r#"<p>Here is the logo of google Images: </p><img src="{url}" alt="google image">"#, url=url));
}

#[wasm_bindgen_test]
fn parse_bold_italic_underline(){
    let source = "**bold**, __now underlined__, ~~this sentence is bad~~ and *now I talk in italic*".to_string();
    let obj = markdown_rs::parse(source);

    assert_eq!(obj, "<p><strong>bold</strong>, <u>now underlined</u>, <s>this sentence is bad</s> and <i>now I talk in italic</i></p>")
}
