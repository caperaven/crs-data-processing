use js_sys::Array;
use wasm_bindgen::JsValue;

pub fn sort_all(data: &JsValue, intent: &JsValue) -> Result<Vec<usize>, JsValue> {
    let result: Vec<usize> = Vec::new();

    let iterator = js_sys::try_iter(data)?.ok_or_else(|| {
        "filter expected an array of record objects"
    })?;

    let mut index: usize = 0;

    Ok(result)
}