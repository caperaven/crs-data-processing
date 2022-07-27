import {assertEquals, assertThrows} from "https://deno.land/std@0.148.0/testing/asserts.ts";
import init, {filter} from "./../bin/wasm_lib.js";

await init();

Deno.test("filter - wrong data sent - throw error", () => {
    assertThrows(() => { filter({}, {}, false) }, "filter expected an array of record objects");
})

Deno.test("filter - simple", () => {
    const result = filter([
        { value: "a" },
        { value: "b" },
        { value: "c" },
        { value: "a" },
        { value: "b" }
    ], { "field": "value", "operator": "eq", "value": "a" }, false);

    assertEquals(result.length, 2);
    assertEquals(result[0], 0);
    assertEquals(result[1], 3);
})