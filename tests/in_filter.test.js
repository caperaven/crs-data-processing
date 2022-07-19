import { assertEquals } from "https://deno.land/std@0.148.0/testing/asserts.ts";
import init, {in_filter} from "./../bin/wasm_lib.js";

await init();

Deno.test("in_filter - equals", () => {
    // true - test
    let result = in_filter({ "field": "value", "operator": "eq", "value": 10 }, { value: 10 }, false);
    assertEquals(result, true);

    // false - test
    result = in_filter({ "field": "value", "operator": "eq", "value": 20 }, { value: 10 }, false);
    assertEquals(result, false);
})

Deno.test("in_filter - not equals", () => {
    // true - test
    let result = in_filter({ "field": "value", "operator": "neq", "value": 20 }, { value: 10 }, false);
    assertEquals(result, true);

    // false - test
    result = in_filter({ "field": "value", "operator": "neq", "value": 10 }, { value: 10 }, false);
    assertEquals(result, false);
})

Deno.test("in_filter - greater than", () => {
    // true - test
    let result = in_filter({ "field": "value", "operator": "gt", "value": 5 }, { value: 10 }, false);
    assertEquals(result, true);

    // false - test
    result = in_filter({ "field": "value", "operator": "gt", "value": 15 }, { value: 10 }, false);
    assertEquals(result, false);
})

Deno.test("in_filter - greater or equal", () => {
    // true - test
    let result = in_filter({ "field": "value", "operator": "ge", "value": 5 }, { value: 10 }, false);
    assertEquals(result, true);

    // equal - test
    result = in_filter({ "field": "value", "operator": "ge", "value": 10 }, { value: 10 }, false);
    assertEquals(result, true);

    // false - test
    result = in_filter({ "field": "value", "operator": "ge", "value": 15 }, { value: 10 }, false);
    assertEquals(result, false);
})

Deno.test("in_filter - less than", () => {
    // true - test
    let result = in_filter({ "field": "value", "operator": "lt", "value": 50 }, { value: 10 }, false);
    assertEquals(result, true);

    // false - test
    result = in_filter({ "field": "value", "operator": "lt", "value": 5 }, { value: 10 }, false);
    assertEquals(result, false);
})

Deno.test("in_filter - greater or equal", () => {
    // true - test
    let result = in_filter({ "field": "value", "operator": "le", "value": 50 }, { value: 10 }, false);
    assertEquals(result, true);

    // equal - test
    result = in_filter({ "field": "value", "operator": "le", "value": 10 }, { value: 10 }, false);
    assertEquals(result, true);

    // false - test
    result = in_filter({ "field": "value", "operator": "le", "value": 5 }, { value: 10 }, false);
    assertEquals(result, false);
})

Deno.test("in_filter - is null", () => {
    let result = in_filter({ "field": "value", "operator": "is_null" }, { value: null }, false);
    assertEquals(result, true);

    result = in_filter({ "field": "value", "operator": "is_null" }, { value: 10 }, false);
    assertEquals(result, false);
})

Deno.test("in_filter - is null", () => {
    let result = in_filter({ "field": "value", "operator": "not_null" }, { value: 10 }, false);
    assertEquals(result, true);

    result = in_filter({ "field": "value", "operator": "not_null" }, { value: null }, false);
    assertEquals(result, false);
})