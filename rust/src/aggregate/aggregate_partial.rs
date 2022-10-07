use std::slice::Iter;
use hashbrown::HashMap;
use js_sys::Array;
use wasm_bindgen::JsValue;
use crate::traits::AggregateNumber;
use crate::aggregate::numbers::Sum;
use crate::{console_log, get_property};

pub fn aggregate_partial(data: &Array, intent: &JsValue, mut _rows: Vec<usize>) -> Result<Vec<JsValue>, JsValue> {
    let result: Vec<JsValue> = Vec::new();

    let keys = js_sys::Reflect::own_keys(&intent).unwrap();
    let mut aggregates: Vec<Box<dyn AggregateNumber>> = Vec::new();

    let mut map: HashMap<String, String> = HashMap::new();

    // populate aggregates with aggregate features
    for key in keys.iter() {
        match key.as_string().unwrap().as_str() {
            "sum" => aggregates.push(Box::new(Sum::new())),
            _ => {}
        }

        let intent_value = get_property!(&intent, &key);
        map.insert(key.as_string().unwrap(), intent_value.as_string().unwrap());
    }

    // process the rows
    let data_iter = data.iter();
    for row in data_iter {
        process_row(&row, &keys, &map);
    }

    Ok(result)
}

pub fn process_row(row: &JsValue, keys: &Array, map: &HashMap<String, String>) {
    for key in keys.iter() {
        let property = map.get(&key.as_string().unwrap()).unwrap();
        let value = get_property!(&row, JsValue::from(property));
    }
}