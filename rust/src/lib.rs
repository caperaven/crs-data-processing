mod evaluators;
mod traits;
mod macros;
mod sort;
mod utils;

use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

#[wasm_bindgen]
pub fn get_value(obj: &JsValue, path: &str) -> JsValue {
    match utils::get_value(obj, path) {
        None => JsValue::NULL,
        Some(value) => value
    }
}

#[wasm_bindgen]
pub fn in_filter(intent: &JsValue, row: &JsValue, case_sensitive: bool) -> Result<bool, JsValue> {
    evaluators::evaluate_object(intent, row, case_sensitive)
}

#[wasm_bindgen]
pub fn filter(data: &Array, intent: &JsValue, case_sensitive: bool) -> Result<Array, JsValue> {
    let result = Array::new();

    let iterator = data.iter();

    let mut index = 0;
    for row in iterator {
        let pass = in_filter(intent, &row, case_sensitive)?;

        if pass == true {
            result.push(&JsValue::from(index));
        }

        index += 1;
    }

    Ok(result)
}

#[wasm_bindgen]
pub fn sort(data: &Array, intent: &JsValue, rows: Option<Vec<usize>>) -> Result<Vec<usize>, JsValue> {
    let result: Result<Vec<usize>, JsValue> = match rows {
        None => crate::sort::sort_all(data, intent),
        Some(rows) => crate::sort::sort_partial(data, intent, rows)
    };

    result
}