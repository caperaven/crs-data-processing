use js_sys::Array;
use wasm_bindgen::JsValue;

pub fn sort_partial(_data: &Array, _intent: &JsValue, rows: Vec<usize>) -> Result<Vec<usize>, JsValue> {
    Ok(rows)
}