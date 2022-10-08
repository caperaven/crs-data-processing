use hashbrown::HashMap;
use js_sys::Array;
use wasm_bindgen::JsValue;
use crate::aggregate::aggregate_numbers::NumberAggregator;

pub fn aggregate_partial(data: &Array, intent: &JsValue, mut _rows: Vec<usize>) -> Result<JsValue, JsValue> {
    let length = data.length() as usize;

    let mut map: HashMap<String, NumberAggregator> = HashMap::new();
    let keys = js_sys::Reflect::own_keys(&intent).unwrap();

    for key in keys.iter() {
        let key_str = key.as_string().unwrap();
        map.insert(key_str.clone(), NumberAggregator::new(key_str.clone()));
    }

    Ok(JsValue::NULL)
}
