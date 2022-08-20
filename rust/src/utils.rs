use wasm_bindgen::JsValue;

pub fn get_value(obj: &JsValue, path: &str) -> Option<JsValue> {
    let parts = path.split(".");
    let mut result = JsValue::from(obj);

    for part in parts {
        let has_property = js_sys::Reflect::has(&result, &JsValue::from(part)).unwrap();
        if !has_property {
            return None
        }

        result = js_sys::Reflect::get(&result, &JsValue::from(part)).ok()?;
    }

    Some(result)
}