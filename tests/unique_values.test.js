import {assertEquals} from "https://deno.land/std@0.148.0/testing/asserts.ts";
import init, {init_panic_hook, unique_values} from "./../bin/wasm_lib.js";

await init();
init_panic_hook();

Deno.test("unique values", () => {
    const result = unique_values([{value: 1}, {value: 2}, {value: 3}, {value: 3}], ["value"]);

    assertEquals(result["value"]["1"], 1);
    assertEquals(result["value"]["2"], 1);
    assertEquals(result["value"]["3"], 2);
})