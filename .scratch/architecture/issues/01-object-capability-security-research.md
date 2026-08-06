Type: research
Status: resolved

## Question

Is LavaMoat well-maintained and the best available way to get transitive object-capability security across dependencies, or is there a better alternative — in the JS/Node ecosystem or another language ecosystem entirely? The answer constrains the language/runtime choice in ticket 02.

## Answer

Full findings with citations: `.scratch/architecture/research/object-capability-security.md` (committed to `main`).

Bottom-line ranking for a project treating transitive per-dependency capability enforcement as a first-class constraint:

1. **JS/TS on Node, with SES/Endo Compartments as the actual enforcement primitive** — LavaMoat's `policy.json`/kernel where its CJS-oriented tooling fits, hand-rolled `@endo/compartment-mapper` Compartments for ESM. The only option where enforcement is mature, lives for the whole process lifetime, and has real production scale (MetaMask, tens of millions of users) plus a second independent driver (Agoric) pushing the spec through TC39. Locks the project into JS/TS/Node.
2. **WebAssembly Component Model + WASI Preview 2 on Wasmtime** — arguably a *stronger* boundary (host/linear-memory enforcement, language-independent, not dependent on guest cooperation), with a strong unrelated production track record (Fastly, Shopify, Fermyon running untrusted code). Much bigger architectural commitment (host+guest-component shape), not a single "just write the app" language.
3. **Deno's permission model alone** — mature but explicitly process-level only; all code on a thread shares one privilege level. Doesn't give per-dependency separation without SES layered on top (at which point you're back in option 1).
4. **Rust + `cap-std`** — not a real contender today: opt-in convention a malicious dependency can simply bypass; default-on sandboxing of `build.rs`/proc-macros is an acknowledged, unshipped Rust Project Goal as of 2026.
5. **Audit/trust tooling** (`cargo-vet`, `cargo-crev`, `cargo-auditable`, Socket.dev) — does **not** satisfy the requirement in any ecosystem. Tells you who reviewed code or blocks known-bad packages at install time; zero runtime restriction on what an allowed dependency can do. Useful as defense-in-depth alongside options 1–2, never as a substitute.

Key caveat: LavaMoat itself is actively maintained (commits through July 2026, coordinated releases June 2026) and MetaMask/Consensys-backed, but its policy-generation is CommonJS-only (no shipped ESM support) and its confirmed production adoption is essentially MetaMask-only. SES/Endo (the underlying primitive LavaMoat is built on) is broader — Agoric is a second major backer, with active TC39 Security working-group participation.
