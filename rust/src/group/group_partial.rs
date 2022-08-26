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

        // set children
        let children = Object::new();

        for child in self.children.iter_mut() {
            child.1.to_value(&children);
        }

        Reflect::set(&result, &JsValue::from("children"), &children);

        // add result items to parent
        Reflect::set(&parent, &JsValue::from(&self.value), &result);
    }
}

pub fn group_data_partial(data: &Array, fields: &Array, rows: Vec<usize>) -> Object {
    let mut root = Field::new(String::from("root"));

    for row in rows.iter() {
        let row_data = data.at(*row as i32);
        process_row(&mut root, &row_data, &fields, 0);
    }

    let result = Object::new();
    root.to_value(&result);
    result
}

fn process_row(parent: &mut Field, row: &JsValue, fields: &Array, field_index: u32) {
    let field: JsValue = fields.get(field_index);
    let field_name: String = field.as_string().unwrap();

    let value = crate::utils::get_value(row, &field_name);
    let process_value: String = match value {
        None => String::from("none"),
        Some(value) => value.as_string().unwrap().clone()
    };

    set_group_count(parent, process_value);
}

fn set_group_count(parent: &mut Field, value: String) {
    match parent.children.contains_key(value.as_str()) {
        true => {
            // let field = parent.children.get_mut(value.as_str()).unwrap();
            // field.child_count += 1;
        }
        false => {
            let result = Field::new(value.clone());
            parent.children.insert(value.clone(), result);
        }
    }

    parent.child_count += 1;
}

// fn get_field_group(parent: &mut Field, value: &str) -> Option<&mut Field> {
//     // return if parent.children.contains_key(value) {
//     //     let result = parent.children.get_mut(value).unwrap();
//     //     Some(result)
//     // } else {
//     //     let result = Field::new(value);
//     //     parent.children.insert(value.clone(), result);
//     //
//     //     let result = parent.children.get_mut(value.as_str()).unwrap();
//     //     Some(result)
//     // }
// }