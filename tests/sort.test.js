import { assertEquals } from "https://deno.land/std@0.148.0/testing/asserts.ts";
import {data} from "./data/simple-data.js"
import init, {init_panic_hook, sort} from "./../bin/wasm_lib.js";

await init();
init_panic_hook();

Deno.test("sort - simple", () => {
    let result = sort(data, ["value:asc"]);
    console.log(result);
})

// Deno.test("sort - width direction", () => {
//     let result = sort(data, {"name": "value"});
// })
//
// Deno.test("sort - sort subset using rows parameter", () => {
//     let result = sort(data, {});
// })