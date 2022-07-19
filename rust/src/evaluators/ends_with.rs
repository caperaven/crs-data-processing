use wasm_bindgen::JsValue;

pub fn evaluate(value1: &JsValue, value2: &JsValue) -> Result<bool, JsValue> {
    let s1 = value1.as_string().unwrap();
    let s2 = value2.as_string().unwrap();
    let target_index = s1.len() - s2.len();
    let result = s1.find(&s2);

    return match result {
        Some(index) => {
            Ok(index == target_index)
        },
        None => Ok(false)
    }
}