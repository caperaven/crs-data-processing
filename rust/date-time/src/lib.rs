mod date_time;

use wasm_bindgen::prelude::*;
use crate::date_time::date_diff_str;

#[wasm_bindgen]
pub fn date_difference_str(date1: String, date2: String) -> String {
    match date_diff_str(&date1, &date2) {
        Ok(s) => s,
        Err(es) => es
    }
}