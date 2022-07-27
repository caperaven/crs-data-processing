mod evaluators;
mod traits;
mod macros;

use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

#[wasm_bindgen]
pub fn in_filter(intent: &JsValue, row: &JsValue, case_sensitive: bool) -> Result<bool, JsValue> {
    evaluators::evaluate_object(intent, row, case_sensitive)
}

#[wasm_bindgen]
pub fn filter(data: &JsValue, intent: &JsValue, case_sensitive: bool) -> Result<Array, JsValue> {
    let result = Array::new();

    let iterator = js_sys::try_iter(data)?.ok_or_else(|| {
        "filter expected an array of record objects"
    })?;

    let mut index = 0;
    for row in iterator {
        let row = row?;
        let pass = in_filter(intent, &row, case_sensitive)?;

        if pass == true {
            result.push(&JsValue::from(index));
        }

        index += 1;
    }

    Ok(result)
}