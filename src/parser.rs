use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub enum Object {
    LineBreak,
    Bold,
    Underline,
    /// A paragraph, contains the text
    Text(String),
    /// A header, contains the level and the text
    Head(u8, Vec<Object>),
    /// A link, contains (text, link)
    Link(Vec<Object>, String)
}

#[derive(Clone, Default, Debug)]
pub struct ObjectStyle {
    bold: bool,
    underline: bool,
    head: bool
}

pub(crate) fn parse(source: String) -> Vec<Object> {
    let mut objects = Vec::new();

    let lines = source.split('\n');
    let mut style = ObjectStyle::default();

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with('#') {
            // header
            // get the number of #
            let mut level = 0;
            for c in line.chars() {
                if c == '#' {
                    level += 1;
                } else {
                    break;
                }
            }

            // remove the # and the space
            let text = line.trim_start_matches('#').trim_start();
            objects.push(Object::Head(level, parse_text(&mut style, text)));
        } else if trimmed.is_empty() {
            // line break
            objects.push(Object::LineBreak);
        } else {
            // paragraph
            for o in parse_text(&mut style, trimmed) {
                objects.push(o);
            }
        }
    }

    objects
}

fn parse_text(style: &mut ObjectStyle, line: &str) -> Vec<Object> {
    let mut objects = Vec::new();

    let mut buf = String::new();
    let mut last_char = ' ';

    let mut is_in_link = false;
    let mut is_link_text = false;
    let mut link_text = String::new();

    for c in line.chars() {
        match c {
            '*' if last_char == '*' => {
                if style.bold {
                    // If already in bold, end bold
                    if !buf.is_empty() && buf != "*" {
                        // remove first '*' and last '*' if there is
                        if buf.starts_with('*') { buf.remove(0); }
                        if buf.ends_with('*') { buf.pop(); }

                        objects.push(Object::Text(buf.clone()));
                        buf.clear();
                    }

                    objects.push(Object::Bold);
                    style.bold = false;
                } else {
                    // If not in bold, start bold
                    if !buf.is_empty() && buf != "*" {
                        // remove the last '*'
                        buf.pop();

                        objects.push(Object::Text(buf.clone()));
                        buf.clear();
                    }

                    objects.push(Object::Bold);
                    style.bold = true;
                }
            }
            // [text](link)
            '[' => {
                if !is_in_link {
                    // Start capturing link text
                    is_in_link = true;
                    is_link_text = true;
                    if !buf.is_empty() {
                        // Add non-link text to objects
                        objects.push(Object::Text(buf.clone()));
                        buf.clear();
                    }
                } else {
                    // Close the link if we encounter another '[' within the link
                    objects.push(Object::Text(format!("[{}]", link_text)));
                    is_in_link = false;
                    is_link_text = false;
                    link_text.clear();
                }
            }
            '(' if is_in_link && last_char == ']' => {
                let _ = link_text.pop();
                is_link_text = false;
                buf.clear();
            }
            ')' if is_in_link => {
                // End link processing and create Object::Link
                objects.push(Object::Link(parse_text(style, link_text.as_str()), buf.clone()));
                is_in_link = false;
                link_text.clear();
                buf.clear();
            }
            _ => {
                if is_in_link && is_link_text {
                    // Append characters to link text
                    link_text.push(c);
                } else {
                    // Append characters to regular buffer
                    buf.push(c);
                }
            }
        }
        last_char = c;
    }

    if !buf.is_empty() {
        objects.push(Object::Text(buf.clone()));
    }

    objects
}
