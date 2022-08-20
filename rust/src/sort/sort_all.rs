use js_sys::Array;
use wasm_bindgen::JsValue;
use crate::sort::sort_partial;

pub fn sort_all(data: &Array, intent: &JsValue) -> Result<Vec<usize>, JsValue> {
    let mut rows: Vec<usize> = vec![];
    let length = data.length() as usize;
    for i in 0..length {
        rows[i] = i;
    }

    sort_partial(data, intent, rows)
}