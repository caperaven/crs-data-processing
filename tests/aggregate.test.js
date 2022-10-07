import init, {aggregate} from "./../bin/wasm_lib.js";

await init();

Deno.test("aggregate", () => {
    aggregate([{value: 1}, {value: 2}, {value: 3}], {sum: "value"});
})