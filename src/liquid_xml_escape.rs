use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter, Runtime, Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "xml_escape",
    description = "",
    parsed(XmlEscapeFilter)
)]
pub struct XmlEscape;

#[derive(Debug, Default, Display_filter)]
#[name = "xml_escape"]
struct XmlEscapeFilter;

impl Filter for XmlEscapeFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> liquid_core::Result<Value> {
        let s = input.to_kstr();
        Ok(Value::scalar(xml_escape_str(&s)))
    }
}

fn xml_escape_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}