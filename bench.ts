import { $ } from 'bun'
import { join } from 'path'

await $`cargo build --release`

let process

console.log("Elysia")
process = Bun.spawn({
    cmd: ['bun', 'elysia/spawn.ts'],
    env: {
        ...Bun.env,
        NODE_ENV: 'production'
    }
})
await Bun.sleep(1000)
await $`bash ./scripts/ely-wrk.sh`

await process.kill()
await Bun.sleep(2000)

console.log("Axum")
process = Bun.spawn({
    cmd: ['./target/release/axum-hello'],
    cwd: join(import.meta.dirname, 'axum')
})
await Bun.sleep(1000)
await $`bash ./scripts/ely-wrk.sh`

await process.kill()
await Bun.sleep(2000)

console.log("Actix")
process = Bun.spawn({
    cmd: ['./target/release/actix-hello'],
    cwd: join(import.meta.dirname, 'actix')
})
await Bun.sleep(1000)
await $`bash ./scripts/ely-wrk.sh`

await process.kill()
