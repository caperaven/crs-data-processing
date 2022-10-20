let p = Deno.run({
    cmd: ["cargo", "clippy"],
    cwd:"./rust/data"
});
console.log(await p.status());

p = Deno.run({
    cmd: ["cargo", "clippy"],
    cwd:"./rust/date-time"
});
console.log(await p.status());