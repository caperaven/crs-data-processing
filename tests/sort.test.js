import { assertEquals } from "https://deno.land/std@0.148.0/testing/asserts.ts";
import {data, data2} from "./data/simple-data.js"
import init, {init_panic_hook, sort} from "./../bin/wasm_lib.js";

await init();
init_panic_hook();

Deno.test("sort - simple", () => {
    let result = sort(data, ["value:asc"]);


    console.log(["value:asc"]);
    console.log(result);
    for (let i = 0; i < result.length; i++) {
        const index = result[i];
        console.log(data[index]);
    }
})

Deno.test("sort - mixed", () => {
    let result = sort(data2, ["v1:asc", "v2:asc"]);

    console.log(["v1:asc", "v2:asc"])
    console.log(result);
    for (let i = 0; i < result.length; i++) {
        const index = result[i];
        console.log(data2[index]);
    }
})

// Deno.test("sort - width direction", () => {
//     let result = sort(data, {"name": "value"});
// })
//
// Deno.test("sort - sort subset using rows parameter", () => {
//     let result = sort(data, {});
// })