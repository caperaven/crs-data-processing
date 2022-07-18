mod equals;

use wasm_bindgen::JsValue;
use crate::{get_property, expression, as_string};

pub struct Evaluator {
}

impl Evaluator {
    pub fn evaluate(intent: &JsValue, row: &JsValue, case_sensitive: bool) -> Result<bool, JsValue> {
        return evaluate_object(intent, row, case_sensitive);
    }
}

fn evaluate_object(intent: &JsValue, row: &JsValue, case_sensitive: bool) -> Result<bool, JsValue> {
    let operator = get_property!(&intent, "operator").as_string().unwrap();

    if operator == "or" || operator == "||" {
        return evaluate_or(expression!(&intent), &row, case_sensitive);
    }

    if operator == "and" || operator == "&&" {
        return evaluate_and(expression!(&intent), &row, case_sensitive);
    }

    if operator == "not" || operator == "!" {
        return evaluate_not(expression!(&intent), &row, case_sensitive);
    }

    let field = as_string!(get_property!(&intent, "field"));
    let mut intent_value = get_property!(&intent, "value");
    let mut row_value = get_property!(&row, field);

    if intent_value.is_string() && case_sensitive == false {
        let intent_string = as_string!(intent_value).to_lowercase();
        let row_value_string = as_string!(row_value).to_lowercase();

        intent_value = JsValue::from(intent_string);
        row_value = JsValue::from(row_value_string);
    }

    return match operator.as_str() {
        "==" | "=" |"eq" => equals::evaluate(&intent_value, &row_value),
        _ => Ok(false)
    }
}

fn evaluate_or(expressions: &JsValue, row: &JsValue, case_sensitive: bool) -> Result<bool, JsValue> {
    // as soon as the expression passes, stop and the row succeeds
    let iter = js_sys::try_iter(expressions)?.ok_or_else(|| "need to pass an array")?;
    for filter in iter {
        let filter = filter?;
        let result = evaluate_object(&filter, &row, case_sensitive);

        if result == Ok(true) {
            return Ok(true);
        }
    }

    // if none of the succeeded this expression batch fails
    Ok(false)
}

fn evaluate_and(expressions: &JsValue, row: &JsValue, case_sensitive: bool) -> Result<bool, JsValue> {
    // as soon as a expression is false, the row fails and we stop the process
    let iter = js_sys::try_iter(expressions)?.ok_or_else(|| "need to pass an array")?;
    for filter in iter {
        let filter = filter?;
        let result = evaluate_object(&filter, &row, case_sensitive);
        if result == Ok(false) {
            return Ok(false);
        }
    }

    Ok(true)
}

fn evaluate_not(expressions: &JsValue, row: &JsValue, case_sensitive: bool) -> Result<bool, JsValue> {
    let result = evaluate_and(expressions, &row, case_sensitive)?;
    Ok(!result)
}