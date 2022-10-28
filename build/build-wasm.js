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

Deno.remove("./src/.gitignore");

Deno.exit(0);