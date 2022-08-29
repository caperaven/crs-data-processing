use std::borrow::BorrowMut;
use std::collections::HashMap;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;

struct Field {
    pub value: String,
    pub child_count: i32,
    pub children: HashMap<String, Field>,
    pub rows: Option<Array>
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

    pub fn to_value(&mut self, parent: &Object) -> Result<JsValue, JsValue> {
        let result = Object::new();

        Reflect::set(&result, &JsValue::from("child_count"), &JsValue::from(self.child_count))?;

        if self.children.len() > 0 {
            let children = Object::new();

            for child in self.children.iter_mut() {
                child.1.to_value(&children)?;
            }

            Reflect::set(&result, &JsValue::from("children"), &children)?;
        }

        if self.rows.is_some() {
            Reflect::set(&result, &JsValue::from("rows"), &self.rows.as_ref().unwrap());
        }

        Reflect::set(&parent, &JsValue::from(&self.value), &result)?;

        Ok(JsValue::NULL)
    }
}

pub fn group_data_partial(data: &Array, fields: &Array, rows: Vec<usize>) -> Result<Object, JsValue> {
    let mut root = Field::new(String::from("root"));

    for row in rows.iter() {
        let row_data = data.at(*row as i32);
        process_row(&mut root, &row_data, &fields, 0, row);
    }

    let result = Object::new();
    root.to_value(&result)?;

    Ok(result)
}

fn process_row(parent: &mut Field, row: &JsValue, fields: &Array, field_index: u32, row_index: &usize) {
    if field_index >= fields.length() {
        return;
    }

    let is_last_field = field_index == fields.length() - 1;

    // Create the group structure
    let field: JsValue = fields.get(field_index);
    let field_name: String = field.as_string().unwrap();

    let value = crate::utils::get_value(row, &field_name);
    let process_value: String = match value {
        None => String::from("none"),
        Some(value) => value.as_string().unwrap().clone()
    };

    set_group_count(parent, process_value, is_last_field, row_index);
}

fn set_group_count(parent: &mut Field, value: String, is_last_field: bool, row_index: &usize) {
    match parent.children.get_mut(value.as_str()) {
        None => {
            let mut children = Field::new(value.clone());

            if is_last_field == true {
                addRowIndex(&mut children, *row_index);
            }

            parent.children.insert(value.clone(), children);
            parent.child_count += 1;

        }
        Some(children) => {
            if is_last_field == true {
                addRowIndex(children, *row_index);
            }
        }
    }
}

fn addRowIndex(parent: &mut Field, row_index: usize) {
    match parent.rows.borrow_mut() {
        None => {
            let rows = Array::new();
            rows.push(&JsValue::from(row_index));
            parent.rows = Some(rows);
        }
        Some(collection) => {
            collection.push(&JsValue::from(row_index));
        }
    }
}