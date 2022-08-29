import { assertEquals } from "https://deno.land/std@0.148.0/testing/asserts.ts";
import {people} from "./data/simple-data.js";
import init, * as wasm from "./../bin/wasm_lib.js";

await init();
wasm.init_panic_hook();

// {
//     "root": {
//     "child_count": 1,
//         "children": {
//              "Angel": {
//                  "child_count": 1,
//                      "children": {
//                      "Ashton"
//                  }
//              }
//         }
//     }
// }

Deno.test("group - simple", () => {
    let result = wasm.group(people, ["details.lastName"]);

    assertEquals(result["root"]["children"]["Doe"]["rows"][0], 0);
    assertEquals(result["root"]["children"]["Doe"]["rows"][1], 1);
    assertEquals(result["root"]["children"]["Doe"]["rows"][2], 3);
    assertEquals(result["root"]["children"]["Smith"]["rows"][0], 2);
})