use wasm_bindgen::JsValue;
use crate::evaluators::{starts_with, ends_with, contains};

pub fn evaluate(value1: &JsValue, value2: &JsValue) -> Result<bool, JsValue> {
    let s1 = value1.as_string().unwrap();

    let has_start = s1.starts_with("%");
    let has_end = s1.ends_with("%");

    if has_start && has_end {
        return contains::evaluate(value1, value2);
    }

    if has_start {
        return starts_with::evaluate(value1, value2);
    }

    if has_end {
        return ends_with::evaluate(value1, value2);
    }

    Ok(false)
}
