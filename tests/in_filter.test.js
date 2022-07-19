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