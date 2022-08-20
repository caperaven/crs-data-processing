import { assertEquals } from "https://deno.land/std@0.148.0/testing/asserts.ts";
import {data} from "./data/simple-data.js"
import init, {get_value} from "./../bin/wasm_lib.js";

await init();

Deno.test("get_value - simple", () => {
    let result = get_value({
        name: "John"
    }, "name");

    assertEquals(result, "John");
})

Deno.test("get_value - path", () => {
    let result = get_value({
        person: {
            identity: {
                name: "John"
            }
        }
    }, "person.identity.name");

    assertEquals(result, "John");
})

Deno.test("get_value - broken path", () => {
    let result = get_value({
        person: {
        }
    }, "person.identity.name");

    assertEquals(result, null);
})