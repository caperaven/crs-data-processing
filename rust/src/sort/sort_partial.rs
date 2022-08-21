use std::cmp::Ordering;
use js_sys::Array;
use wasm_bindgen::JsValue;
use crate::console_log;

pub fn sort_partial(data: &Array, intent: &JsValue, mut rows: Vec<usize>) -> Result<Vec<usize>, JsValue> {
    rows.sort_by(|a, b| evaluate(*a as i32, *b as i32, data, intent));
    Ok(rows)
}

fn evaluate(a: i32, b: i32, data: &Array, intent: &JsValue) -> Ordering {
    let obj_a = data.at(a);
    let obj_b = data.at(b);

    Ordering::Equal
}