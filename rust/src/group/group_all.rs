use js_sys::Array;
use wasm_bindgen::JsValue;
use crate::group::group_partial::group_data_partial;

pub fn group_data_all(data: &Array, intent: &Array) -> JsValue {
    let length = data.length() as usize;
    let mut rows: Vec<usize> = Vec::new();

    for i in 0..length {
        rows.push(i);
    }

    group_data_partial(data, intent, rows)
}
