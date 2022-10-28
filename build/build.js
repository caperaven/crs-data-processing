import {ensureDir} from "https://deno.land/std@0.149.0/fs/ensure_dir.ts";
import {emptyDir} from "https://deno.land/std@0.149.0/fs/empty_dir.ts";

async function createFolderStructure() {
    await ensureDir("./dist");
    await emptyDir("./dist");
}

await createFolderStructure();
let p = Deno.run({
    cmd: ["wasm-pack", "build", "rust/data", "--target", "web", "--out-dir", "./../../src"]
});
console.log(await p.status());

p = Deno.run({
    cmd: ["wasm-pack", "build", "rust/date-time", "--target", "web", "--out-dir", "./../../src"]
});
console.log(await p.status());

await Promise.all([
    Deno.copyFile("./src/data_processing.js", "./dist/data_processing.js"),
    Deno.copyFile("./src/data_processing_bg.wasm", "./dist/data_processing_bg.wasm"),
    Deno.copyFile("./src/data_processing_date_time.js", "./dist/data_processing_date_time.js"),
    Deno.copyFile("./src/data_processing_date_time_bg.wasm", "./dist/data_processing_date_time_bg.wasm"),
    Deno.copyFile("./src/crs-data-processing-actions.js", "./dist/crs-data-processing-actions.js"),
]);

Deno.exit(0);