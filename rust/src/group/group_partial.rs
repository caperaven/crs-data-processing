use std::collections::HashMap;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;
use crate::console_log;

struct Field {
    // Field Name
    value: String,

    // The rows defined for this group
    child_count: i64,

    // Collection of sub folder
    children: HashMap<String, Field>,

    rows: Option<Vec<i64>>
}

impl Field {
    pub fn new(value: String) -> Field {
        Field {
            value,
            child_count: 0,
            children: HashMap::new(),
            rows: None
        }
    }

    pub fn to_value(&mut self, parent: &Object) {
        let result = Object::new();
        // set result items
        Reflect::set(&result, &JsValue::from("child_count"), &JsValue::from(self.child_count));

        // add result items to parent
        Reflect::set(&parent, &JsValue::from(&self.value), &result);
    }
}

pub fn group_data_partial(data: &Array, intent: &Array, rows: Vec<usize>) -> Object {
    let mut root = Field::new(String::from("root"));

    for row in rows.iter() {
        let row_data = data.at(*row as i32);
        process_row(&mut root, &row_data, &intent, 0);
    }

    let result = Object::new();
    root.to_value(&result);
    result
}

fn process_row(parent: &mut Field, row: &JsValue, fields: &Array, field_index: u32) {
    let field: JsValue = fields.get(field_index);
    let field_name: String = field.as_string().unwrap();

    let value = crate::utils::get_value(row, &field_name);

    match value {
        None => {}
        Some(value) => {
            let _field_group = get_field_group(parent, field_name);
        }
    }
}

fn get_field_group(parent: &mut Field, field_name: String) -> Option<&mut Field> {
    return if parent.children.contains_key(field_name.as_str()) {
        let result = parent.children.get_mut(field_name.as_str()).unwrap();
        Some(result)
    } else {
        let result = Field::new(field_name.clone());
        parent.children.insert(field_name.clone(), result);

        let result = parent.children.get_mut(field_name.as_str()).unwrap();
        Some(result)
    }
}