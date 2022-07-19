use js_sys::Array;
use wasm_bindgen::JsValue;

pub fn evaluate(value1: &JsValue, value2: &JsValue) -> Result<bool, JsValue> {
    let collection = Array::from(value1);
    let index = Array::index_of(&collection, value2, 0);

    if index == -1 {
        return Ok(false);
    }

    Ok(true)
}
