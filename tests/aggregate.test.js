import init, {aggregate} from "./../bin/wasm_lib.js";

await init();

Deno.test("aggregate", () => {
    const result = aggregate([{value: 1}, {value: 2}, {value: 3}], ["value"]);
    console.log(result);
})