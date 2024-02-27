use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Object {
    LineBreak,
    Bold,
    Italic,
    Underline,
    StrikeThrough,
    /// A paragraph, contains the text
    Text(String),
    /// A header, contains the level and the text
    Head(u8, Vec<Object>),
    /// A link, contains (text, link)
    Link(Vec<Object>, String),
    /// An image, contains (url, alt?)
    Image(String, Option<String>)
}

#[derive(Clone, Default, Debug)]
pub struct ObjectStyle {
    bold: bool,
    underline: bool,
    italic: bool,
    strike_through: bool,
    head: bool,
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
    // Used when two data need to be kept in memory, such as a link (with its URL and text) or an image
    let mut second_buf = String::new();

    let mut last_char = ' ';

    let mut is_in_link = false;
    let mut is_link_text = false;

    let mut is_image = false;
    let mut is_image_alt = false;

    let mut chars = line.chars();

    #[allow(clippy::while_let_on_iterator)]
    while let Some(c) = chars.next() {
        match c {
            // If not in bold, start bold
            '*' if last_char == '*' && !style.bold => {
                if !buf.is_empty() && buf != "*" {
                    // remove the last '*'
                    buf.pop();

                    objects.push(Object::Text(buf.clone()));
                    buf.clear();
                }

                objects.push(Object::Bold);
                style.bold = true;
            }
            // If already in bold, end bold
            '*' if last_char == '*' && style.bold => {
                if !buf.is_empty() && buf != "*" {
                    // remove first '*' and last '*' if there is
                    if buf.starts_with('*') { buf.remove(0); }
                    if buf.ends_with('*') { buf.pop(); }

                    objects.push(Object::Text(buf.clone()));
                    buf.clear();
                }

                objects.push(Object::Bold);
                style.bold = false;
            }

            '*' if !style.italic => {
                // check if the next char is '*', if so, it's a bold and not an italic, so we don't start italic
                if let Some(next) = chars.clone().next() {
                    if next == '*' {
                        buf.push(c);
                        last_char = c;
                        continue;
                    }
                }

                if !buf.is_empty() {
                    objects.push(Object::Text(buf.clone()));
                    buf.clear();
                }
                objects.push(Object::Italic);
                style.italic = true;
            }
            '*' if style.italic && last_char != '*' => {
                if !buf.is_empty() {
                    objects.push(Object::Text(buf.clone()));
                    buf.clear();
                }

                objects.push(Object::Italic);
                style.italic = false;
            }

            '_' if last_char == '_' && !style.underline => {
                if !buf.is_empty() && buf != "_" {
                    // remove the last '_'
                    buf.pop();

                    objects.push(Object::Text(buf.clone()));
                    buf.clear();
                }

                objects.push(Object::Underline);
                style.underline = true;
            }
            '_' if last_char == '_' && style.underline => {
                if !buf.is_empty() && buf != "_" {
                    // remove the last '_'
                    buf.pop();

                    objects.push(Object::Text(buf.clone()));
                    buf.clear();
                }

                objects.push(Object::Underline);
                style.underline = false;
            }

            '~' if last_char == '~' && !style.strike_through => {
                if !buf.is_empty() && buf != "~" {
                    // remove the last '~'
                    buf.pop();

                    objects.push(Object::Text(buf.clone()));
                    buf.clear();
                }

                objects.push(Object::StrikeThrough);
                style.underline = true;
            }
            '~' if last_char == '_' && style.strike_through => {
                if !buf.is_empty() && buf != "~" {
                    // remove the last '~'
                    buf.pop();

                    objects.push(Object::Text(buf.clone()));
                    buf.clear();
                }

                objects.push(Object::StrikeThrough);
                style.underline = false;
            }

            // [text](link) || ![alt](url)
            '[' => {
                if !is_in_link && !is_image && last_char != '!' {
                    // Start capturing link text
                    is_in_link = true;
                    is_link_text = true;
                    if !buf.is_empty() {
                        // Add non-link text to objects
                        objects.push(Object::Text(buf.clone()));
                        buf.clear();
                    }
                } else if !is_link_text && !is_image && last_char == '!' {
                    is_image = true;
                    is_image_alt = true;

                    buf.pop();

                    if !buf.is_empty() {
                        // Add non-image text to objects
                        objects.push(Object::Text(buf.clone()));
                        buf.clear();
                    }
                } else {
                    // Close the link if we encounter another '[' within the link
                    objects.push(Object::Text(format!("[{}]", second_buf)));
                    is_in_link = false;
                    is_link_text = false;
                    second_buf.clear();
                }
            }
            '(' if (is_in_link || is_image) && last_char == ']' => {
                let _ = second_buf.pop();
                is_link_text = false;
                is_image_alt = false;
                buf.clear();
            }
            ')' if is_in_link => {
                // End link processing and create Object::Link
                objects.push(Object::Link(parse_text(style, second_buf.as_str()), buf.clone()));
                is_in_link = false;
                second_buf.clear();
                buf.clear();
            }
            ')' if is_image => {
                if !second_buf.is_empty() {
                    let mut alt = None;
                    if !second_buf.is_empty() { alt = Some(second_buf.clone()); }

                    objects.push(Object::Image(buf.clone(), alt));
                    is_image = false;
                    second_buf.clear();
                    buf.clear();
                }
            }
            _ => {
                if is_in_link && is_link_text {
                    // Append characters to link text
                    second_buf.push(c);
                } else if is_image && is_image_alt {
                    // Append characters to image alt
                    second_buf.push(c)
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
