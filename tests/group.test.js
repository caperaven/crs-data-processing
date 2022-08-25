import init, * as wasm from "./../bin/wasm_lib.js";

await init();

Deno.test("group - simple", () => {
    let result = wasm.group([], []);
    let group = wasm.groupdata_new(result.ptr);
    console.log(group);
})