use crate::parser::Object;

#[derive(Debug, Clone, Default)]
struct FormattingInformations {
    is_head: bool,
    is_paragraph: bool,
    in_paragraph: bool
}

pub(crate) fn format(objects: Vec<Object>) -> String {
    web_sys::console::debug_1(&format!("{objects:#?}").into());

    let mut infos = FormattingInformations::default();
    let mut r = format_internal(objects, &mut infos);

    if infos.in_paragraph {
        r.push_str("</p>");
    }

    r
}

fn format_internal(objects: Vec<Object>, infos: &mut FormattingInformations) -> String {
    let mut result = String::new();

    let mut iter = objects.iter();

    while let Some(obj) = iter.next() {
        match obj {
            Object::Text(text) if infos.is_head || infos.in_paragraph => { result.push_str(text); }
            Object::Text(text) => {
                result.push_str(&format!("<p>{}", text));
                infos.is_paragraph = true;
                infos.in_paragraph = true;
                infos.is_head = false;
            }
            Object::Link(t, l) => {
                result.push_str(
                    &format!(
                        r#"<a href="{l}" target="_blank">{}</a>"#,
                        format_internal(t.clone(), infos)
                    )
                )
            }
            Object::Head(level, text) => {
                if infos.in_paragraph {
                    result.push_str("</p>");
                    infos.in_paragraph = false;
                }

                infos.is_head = true;
                infos.is_paragraph = false;
                result.push_str(&format!("<h{l}>{f}</h{l}>", l = level, f = format_internal(text.to_vec(), infos)));
                infos.is_head = false;
                infos.is_paragraph = true;
            },
            Object::LineBreak => { result.push_str("<br>"); }
            Object::Bold => {
                if !infos.in_paragraph {
                    result.push_str("<p>");
                    infos.in_paragraph = true;
                }

                let mut objects = Vec::new();
                let mut founded = false;

                for o in iter.by_ref() {
                    if !matches!(o, Object::Bold) {
                        objects.push(o.clone())
                    } else {
                        founded = true;
                        break;
                    }
                }

                if founded {
                    result.push_str(&format!("<strong>{f}</strong>", f = format_internal(objects, infos)));
                } else {
                    result.push_str(format_internal(objects, infos).as_str())
                }
            }
            Object::Underline => {
                if !infos.in_paragraph {
                    result.push_str("<p>");
                    infos.in_paragraph = true;
                }

                let mut objects = Vec::new();
                let mut founded = false;

                for o in iter.by_ref() {
                    if !matches!(o, Object::Underline) {
                        objects.push(o.clone())
                    } else {
                        founded = true;
                        break;
                    }
                }

                if founded {
                    result.push_str(&format!("<u>{f}</u>", f = format_internal(objects, infos)));
                } else {
                    result.push_str(format_internal(objects, infos).as_str())
                }
            }
        }
    }

    result
}