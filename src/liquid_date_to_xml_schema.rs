use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter, Runtime, Value, ValueView};
use crate::time_utils::parse_site_datetime;

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "date_to_xmlschema",
    description = "TODO",
    parsed(DateToXmlSchemaFilter)
)]
pub struct DateToXmlSchema;

#[derive(Debug, Default, Display_filter)]
#[name = "date_to_xmlschema"]
struct DateToXmlSchemaFilter;

impl Filter for DateToXmlSchemaFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> liquid_core::Result<Value> {
        let s = input.to_kstr();

        let parsed = parse_site_datetime(&s)
            .map_err(|e| liquid_core::Error::with_msg(format!("invalid date `{s}`: {e}")))?;

        Ok(Value::scalar(parsed.to_rfc3339()))
    }
}