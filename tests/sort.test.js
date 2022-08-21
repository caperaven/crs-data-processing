import { assertEquals } from "https://deno.land/std@0.148.0/testing/asserts.ts";
import {data, data2} from "./data/simple-data.js"
import init, {init_panic_hook, sort} from "./../bin/wasm_lib.js";

await init();
init_panic_hook();

function printResults(result, collection) {
    for (let i = 0; i < result.length; i++) {
        const index = result[i];
        console.log(collection[index]);
    }
}

Deno.test("sort - simple", () => {
    let result = sort(data, ["value"]);

    assertEquals(result[0], 1);
    assertEquals(result[1], 4);
    assertEquals(result[2], 0);
    assertEquals(result[3], 2);
    assertEquals(result[4], 3);

    // printResults(result, data);
})

Deno.test("sort - simple dec", () => {
    let result = sort(data, ["value:dec"]);

    assertEquals(result[0], 3);
    assertEquals(result[1], 2);
    assertEquals(result[2], 0);
    assertEquals(result[3], 4);
    assertEquals(result[4], 1);

    // printResults(result, data);
})

Deno.test("sort - mixed asc -> asc", () => {
    let result = sort(data2, ["v1", "v2"]);

    assertEquals(result[0], 1);
    assertEquals(result[1], 0);
    assertEquals(result[2], 3);
    assertEquals(result[3], 4);
    assertEquals(result[4], 5);
    assertEquals(result[5], 2);

    // printResults(result, data2);
})

Deno.test("sort - mixed asc -> dec", () => {
    let result = sort(data2, ["v1", "v2:dec"]);

    assertEquals(result[0], 0);
    assertEquals(result[1], 1);
    assertEquals(result[2], 4);
    assertEquals(result[3], 3);
    assertEquals(result[4], 2);
    assertEquals(result[5], 5);

    // printResults(result, data2);
})

// Deno.test("sort - width direction", () => {
//     let result = sort(data, {"name": "value"});
// })
//
// Deno.test("sort - sort subset using rows parameter", () => {
//     let result = sort(data, {});
// })