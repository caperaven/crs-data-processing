import { assertEquals } from "https://deno.land/std@0.148.0/testing/asserts.ts";
import init, {in_filter} from "./../bin/wasm_lib.js";

await init();

Deno.test("in_filter - equals", () => {
    let intent = {
        "field": "value",
        "operator": "eq",
        "value": 10
    }

    let record = {
        value: 10
    }

    let result = in_filter(intent, record, false);
    assertEquals(result, true);
})