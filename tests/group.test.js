import {people} from "./data/simple-data.js";
import init, * as wasm from "./../bin/wasm_lib.js";

await init();

// {
//     "root": {
//     "child_count": 1,
//         "children": {
//         "Angel": {
//             "child_count": 1,
//                 "children": {
//                 "Ashton"
//             }
//         }
//     }
//     }
// }

Deno.test("group - simple", () => {
    let result = wasm.group(people, ["details.lastName"]);
    console.log(result);
})