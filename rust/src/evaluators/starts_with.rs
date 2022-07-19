use wasm_bindgen::JsValue;

pub fn evaluate(value1: &JsValue, value2: &JsValue) -> Result<bool, JsValue> {
    let value1_str = value1.as_string().unwrap();
    let value2_str = value2.as_string().unwrap();
    let result = value1_str.find(&value2_str);

    return match result {
        None => Ok(false),
        Some(index) => Ok(index == 0)
    }
}