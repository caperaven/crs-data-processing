use hashbrown::HashMap;
use js_sys::{Array};
use wasm_bindgen::JsValue;

pub fn unique_values_partial(data: &Array, intent: &Vec<JsValue>, mut _rows: Vec<usize>) -> Result<JsValue, JsValue> {
    let mut property_map: HashMap<String, HashMap<String, i64>> = HashMap::new();

    for key in intent.iter() {
        let property = key.as_string().unwrap().clone();
        property_map.insert(property.clone(), HashMap::new());
    }

    for row in data.iter() {
        for key in intent.iter() {
            let row_value = js_sys::Reflect::get(&row, &key).unwrap().as_string().unwrap();
            let value_map = property_map.get_mut(&key.as_string().unwrap()).unwrap();
            value_map.entry(row_value)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    Ok(JsValue::NULL)
}