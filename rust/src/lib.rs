mod evaluators;
mod traits;
mod macros;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn in_filter(intent: &JsValue, row: &JsValue, case_sensitive: bool) -> Result<bool, JsValue> {
    evaluators::Evaluator::evaluate(intent, row, case_sensitive)
}