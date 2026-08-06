# Transitive object-capability security across dependencies: research

Date: 2026-08-05
Scope: inform the language/framework choice for exocortex (local-first journal/calendar/task-manager/PKM app). The constraint under test: a dependency (and everything it transitively pulls in) can only access the specific capabilities/APIs it's explicitly granted — not ambient access to filesystem, network, process, globals, etc. LavaMoat (MetaMask/Consensys, built on SES/Hardened JavaScript) was named as the reference example.

---

## 1. LavaMoat, concretely

### What it is and how it works

LavaMoat is "a toolkit for mitigating software supply chain attacks in JavaScript projects by sandboxing dependencies and restricting their access to platform APIs and JavaScript primitives," made by MetaMask and funded by Consensys, built on Agoric's SES sandbox. ([github.com/LavaMoat/LavaMoat](https://github.com/LavaMoat/LavaMoat))

It works in three layers, per the project's own description ([github.com/LavaMoat/LavaMoat README](https://github.com/LavaMoat/LavaMoat/blob/main/README.md), [metamask.io — Using LavaMoat](https://metamask.io/news/security/using-lavamoat-to-solve-software-supply-chain-security/)):

1. **Install-time**: `@lavamoat/allow-scripts` disables npm/yarn lifecycle install scripts by default. It "configures your project to disable running install scripts by default and gives you a configuration section in package.json where the allowed ones can be listed." This closes the most common real-world attack vector (a compromised package running arbitrary code the moment it's installed). ([LavaMoat README](https://github.com/LavaMoat/LavaMoat/blob/main/README.md))
2. **Build-time policy generation**: LavaMoat walks the `require()` graph of your app and generates `policy.json`, written to e.g. `./lavamoat/browserify/policy.json` (or the equivalent for `lavamoat-node`/webpack). "LavaMoat crawls your application's dependency tree and determines which resources each dependency is currently using" and snapshots that as the policy. ([lavamoat.github.io/guides/policy](https://lavamoat.github.io/guides/policy/))
3. **Runtime enforcement (the "kernel")**: a custom LavaMoat kernel is injected into the bundle/runtime and handles `require()` calls. "When required, a module is initialized, usually by evaluation inside a SES container, and enforces the app-specified LavaMoat policy." The bundle's runtime is split into a kernel/loader/prelude that initializes modules under policy, and the module sources themselves. ([LavaMoat README](https://github.com/LavaMoat/LavaMoat/blob/main/README.md), Yuri Shapkarin summary cross-checked against the README: [shapkarin.me](https://shapkarin.me/articles/LavaMoat-against-JS-supply-chain-attacks/))

`policy.json` is keyed by canonical package name (`resources`), and each entry can declare ([lavamoat.github.io/guides/policy](https://lavamoat.github.io/guides/policy/)):
- `packages` — which other packages this one may `require`/import
- `builtin` — which Node builtins it may access
- `globals` — which platform globals it may read (`true`) or read+write (`"write"`)
- `native` — whether native (compiled) modules are permitted

A `policy-override.json` lets you hand-adjust the auto-generated policy without editing the generated file directly.

The actual isolation primitive underneath all of this is **SES (Secure ECMAScript)**, an implementation of the TC39 "Hardened JavaScript" proposal maintained by the Endo project (Agoric). `lockdown()` freezes the JS realm's intrinsics (globalThis, prototypes, etc.) to remove tamper/covert-channel vectors, and **Compartments** give each package its own `globalThis`/module namespace with no default authority — no `fetch`, `Date.now()`, `Math.random()`, filesystem, etc. unless explicitly endowed. Objects passed between compartments must be `harden()`-ed (deep-frozen) first. This is real object-capability discipline: authority is only ever passed in, never reached for ambiently. ([endojs/endo SES README](https://github.com/endojs/endo/blob/master/packages/ses/README.md))

Important nuance: **SES by itself provides no dependency-level policy** — it's the sandboxing primitive (Compartments + lockdown), not a system that knows about your `node_modules` graph or auto-generates least-authority policies per package. That policy-generation-and-enforcement layer is what LavaMoat adds on top of SES. ([SES README](https://github.com/endojs/endo/blob/master/packages/ses/README.md) — confirmed no mention of module-level access control in the SES spec itself)

### Maintenance / support status (checked 2026-08-05)

- **Commit activity**: continuous through July 2026 — the most recent commits at check time were dated July 21–23, 2026, mixing renovate-bot dependency bumps with substantive changes (CI, lint, `lavapack` ES-version handling). No sign of slowdown. ([github.com/LavaMoat/LavaMoat/commits/main](https://github.com/LavaMoat/LavaMoat/commits/main))
- **Releases**: the monorepo's packages (`lavamoat`, `lavamoat-node`, `lavamoat-webpack`, `lavapack`, `@lavamoat/allow-scripts`, `@lavamoat/types`, etc.) all shipped coordinated patch releases dated **June 25, 2026** (e.g. `lavamoat@11.1.4`, `webpack@2.2.4`, `node@1.0.7`), described as bug fixes and dependency bumps. ([github.com/LavaMoat/LavaMoat/releases](https://github.com/LavaMoat/LavaMoat/releases))
- **Org backing**: "Made with love by MetaMask," funded by Consensys, built on Agoric's SES. ([LavaMoat README](https://github.com/LavaMoat/LavaMoat/blob/main/README.md))
- **Repo health**: ~1.2k stars, 82 forks, 157 open issues, 67 open PRs, MIT license, 3,285+ commits on main. `lavamoat-viz` (dependency-graph visualizer) is explicitly flagged as currently unmaintained, with only "tentative plans to resume." ([github.com/LavaMoat/LavaMoat](https://github.com/LavaMoat/LavaMoat), README unmaintained-package note)
- **Real-world adoption**: confirmed at production scale only at MetaMask itself — "LavaMoat is currently protecting tens of millions of users at MetaMask and runs at three different times during each development process." MetaMask's own security writeups (e.g. the Ledger Connect Kit supply-chain incident) cite LavaMoat as their mitigation. ([metamask.io — LavaMoat and the Ledger Software Supply Chain Attack](https://metamask.io/news/lavamoat-and-the-ledger-software-supply-chain-attack)) I could not find public, named confirmation of other companies running LavaMoat in production in 2025–2026 beyond MetaMask/Consensys-adjacent projects; it is openly licensed (MIT) and marketed as usable by any JS team, but adoption evidence outside MetaMask is essentially absent from public sources.
- **Significant practical limitation for a new app in 2026**: LavaMoat's static policy-generation analysis is **CommonJS-only** — it does not yet analyze ESM import graphs. There is an open, unresolved tracking issue for ESM support ("LavaMoat support for ECMAScript modules," endojs/endo #1079, referenced via search of the repo). ([search-confirmed via github.com/endojs/endo issue tracker discussion](https://github.com/endojs/endo)) This matters a lot for a greenfield 2026 project, which would almost certainly want to be ESM-native. `lavamoat-webpack` is out of beta but bundler support otherwise remains centered on Browserify/webpack/Node's CJS loader; no first-class Vite/esbuild/ESM story was found in the docs or README as of this check.

**Bottom line on LavaMoat itself**: it is real, working, actively maintained infrastructure with serious institutional backing (MetaMask/Consensys) and a genuine production deployment at scale — but it is essentially a single-org tool (LavaMoat the policy/kernel layer; SES/Endo the underlying primitive, also mostly Agoric+MetaMask), CommonJS-oriented, and best proven as a *build-time hardening step for a browser extension/web app*, not as the load-bearing security architecture of a long-lived, ESM-native application.

---

## 2. Alternatives within the JS/Node/npm ecosystem

### 2a. Endo/SES standalone (no LavaMoat policy tooling)

SES + Compartments is usable directly without LavaMoat's `policy.json`/kernel layer — you call `lockdown()` and manually construct `Compartment`s with only the endowments you choose. Agoric uses this directly (compartments isolate contracts within a "vat" in their blockchain runtime). ([docs.agoric.com — Hardened JavaScript](https://docs.agoric.com/guides/js-programming/hardened-js), [endojs/endo](https://github.com/endojs/endo))

The closest thing Endo has to LavaMoat's automatic-policy story is **`@endo/compartment-mapper`**: it scans a Node application's `package.json` graph (main app + transitive deps) and constructs one Compartment per package, wiring up what each package exports/imports to the compartments that need it, "trusting the package manager" for `node_modules` layout. It explicitly supports tagging packages for ESM ("import" condition) as well as CJS, which is more modern than LavaMoat's CJS-only analysis. ([npmjs.com/package/@endo/compartment-mapper description cross-checked against source doc](https://www.npmjs.com/package/@endo/compartment-mapper))

Maturity: Endo (the endojs/endo monorepo, home of `ses`, `compartment-mapper`, `eventual-send`, CapTP, the Endo daemon CLI) is larger and more active by raw numbers than LavaMoat itself — 7,300+ commits, 1,000+ stars, 320 open issues, 220 open PRs, weekly public sync calls, and active participation in TC39's Security (TG-3) working group. It's backed primarily by Agoric (a blockchain/smart-contract platform), with MetaMask as a second production adopter of the underlying SES shim. ([github.com/endojs/endo](https://github.com/endojs/endo))

Fit for a long-lived app: SES/Compartments is a **runtime primitive that stays live for the life of the process** — unlike a lint/audit step, the Compartment boundaries are enforced every time code executes, for the whole app lifetime. But without LavaMoat-style auto-policy generation, you (the app author) are on the hook for manually authoring and maintaining the endowment/policy graph for every dependency — real engineering cost that grows with the dependency tree, and there's no equivalent of LavaMoat's CJS auto-discovery for a fully hand-rolled Compartment setup. `compartment-mapper` narrows that gap for ESM/CJS Node apps but is lower-level/DIY compared to LavaMoat's opinionated `policy.json` + `allow-scripts` package.

### 2b. Socket.dev

Socket is "a developer-first supply-chain security platform" that protects against malicious dependencies, vulnerable packages, license risk across npm, PyPI, Go, Maven, Cargo, NuGet, RubyGems, etc. ([docs.socket.dev](https://docs.socket.dev/)) It works by cloning registries in real time and running static analysis + LLM-based inspection on every new package within seconds of publication, flagging ~70+ alert categories (install scripts, obfuscation, network/filesystem/shell behavior signatures, typosquats, malware) — reportedly identifying on the order of 100 supply-chain attacks/week and cataloguing 16,000+ malicious packages to date. ([docs.socket.dev/docs/threat-feed](https://docs.socket.dev/docs/threat-feed))

**Socket Firewall** (free and enterprise tiers) is the closest thing Socket has to enforcement: it's an HTTP/HTTPS proxy that intercepts package-manager requests at **install time**. The free tier only warns on AI-flagged malware (does not block); the paid Enterprise tier can be configured to actually block installation of flagged packages. ([docs.socket.dev/docs/socket-firewall-overview](https://docs.socket.dev/docs/socket-firewall-overview), [docs.socket.dev/docs/socket-firewall-free](https://docs.socket.dev/docs/socket-firewall-free), [docs.socket.dev/docs/socket-firewall-enterprise](https://docs.socket.dev/docs/socket-firewall-enterprise))

**This is categorically different from LavaMoat/SES.** Socket is detection + install-time gating based on reputation/behavior analysis of package *code* — it does not sandbox anything at runtime and gives a dependency no less ambient authority once it's allowed to install and run. It answers "should this package be allowed into the tree at all," not "what can this package touch once it's running." It's a good complementary layer (catches known-bad packages before they land) but does not provide the object-capability guarantee the project wants.

### 2c. Deno's runtime permission model

Deno sandboxes the whole process: `--allow-net[=hosts]`, `--allow-read[=paths]`, `--allow-write`, `--allow-env[=vars]`, `--allow-run[=binaries]`, `--allow-ffi`, with a default-deny posture — "unless you specifically enable it, a program run with Deno has no file, network, or environment access." Permissions can be scoped to specific hosts/paths/vars/binaries, and there are corresponding `--deny-*` flags that take precedence over allows. ([docs.deno.com/runtime/fundamentals/security](https://docs.deno.com/runtime/fundamentals/security/), [docs.deno.com/runtime/reference/permissions](https://docs.deno.com/runtime/reference/permissions/))

**Critical limitation for this use case**: Deno's permission grant is **per-process, not per-module/per-dependency**. The docs are explicit: "All code executing on the same thread shares the same privilege level. It is not possible for different modules to have different privilege levels within the same thread." The only escape hatch is Web Workers, which can be spawned with a reduced permission set — but that's a manual, coarse-grained, whole-worker partitioning, not automatic least-authority per dependency. ([docs.deno.com/runtime/fundamentals/security](https://docs.deno.com/runtime/fundamentals/security/))

So: Deno gives you a strong, mature, well-supported boundary around *your whole app vs. the OS*, which is valuable in its own right, but it does **not** give transitive per-dependency capability separation — a malicious transitive dependency gets exactly the same `--allow-net`/`--allow-read` grant as your first-party code. To get LavaMoat-style per-package containment inside Deno you'd need to layer SES/Compartments (or similar) on top of it yourself; Deno's own model doesn't solve the stated problem.

### 2d. Other JS-ecosystem options found

- **`iframe`/`vm2`/Node `vm` module** style sandboxes: `vm2` (a popular Node sandbox for running untrusted code in a V8 context) is explicitly **deprecated and known-insecure** as a security boundary — not researched in depth here since it doesn't meet a "well-maintained, safe" bar, but worth flagging so it isn't mistakenly considered; V8 `Isolate`/`vm` context boundaries are not hardened against determined escapes the way SES's frozen-primordials + Compartment model is.
- **Node's experimental permission model** (`--permission`, `--allow-fs-read`, etc., stable-ish since Node 20+): similar shape to Deno's, i.e., process-level, not per-dependency. Not independently deep-dived here, but the same "process-level, not transitive-per-package" limitation applies by construction — flagging for completeness rather than as a serious contender for the stated requirement.

---

## 3. Equivalent approaches in other language ecosystems

The user's framing is important to honor precisely: **audit/trust tooling tells you who reviewed a dependency; it does not sandbox what that dependency can do at runtime.** These are not substitutes for each other.

### 3a. Rust — audit/trust tooling (does NOT provide runtime capability enforcement)

- **`cargo-vet`** (Mozilla): ensures third-party Rust dependencies have been audited by a trusted entity; teams keep their own in-tree audit records and can import audit sets from trusted sources (e.g. Mozilla's, Google's) to avoid re-auditing everything themselves. ([mozilla.github.io/cargo-vet](https://mozilla.github.io/cargo-vet/), [github.com/mozilla/cargo-vet](https://github.com/mozilla/cargo-vet))
- **`cargo-crev`**: a cryptographically verifiable, distributed code-review/trust-graph system — you trust reviewers, reviewers vouch for crate versions. ([github.com/kpcyrd/cargo-crev via search](https://github.com/kpcyrd/cargo-crev))
- **`cargo-auditable`**: embeds a dependency manifest into the compiled binary so you can later determine what versions of what crates (and any known vulnerabilities, via `cargo audit`) went into a shipped binary. This is provenance/SBOM tooling, not a sandbox. ([rust-secure-code/cargo-auditable](https://github.com/rust-secure-code/cargo-auditable))
- None of these restrict what a dependency's code can actually *do* at build or run time — a malicious `build.rs`, malicious proc-macro, or malicious crate code runs with full ambient authority (filesystem, network, process spawn, arbitrary `unsafe`) regardless of audit status. The Rust project itself acknowledges this gap: "The Rust core team has been discussing sandboxing for years, but nothing has shipped" as a default. ([search-derived, cross-referenced against the active RFC/goal below])

### 3b. Rust — real (partial) capability-oriented mechanisms, and their limits

- **`cap-std`** (Bytecode Alliance): "a capability-based version of the Rust standard library," providing `Dir`/`cap_std::fs`, `cap_std::net`, `cap_std::time` etc. where e.g. a `Dir` handle only allows opening files underneath it and rejects `..`/symlink/absolute-path escapes; on Linux 5.6+ it uses `openat2` for this in a single syscall. ([bytecodealliance/cap-std README](https://github.com/bytecodealliance/cap-std/blob/main/cap-std/README.md), [sunfishcode's intro post](https://blog.sunfishcode.online/introducing-cap-std/))
  - **Important caveat**: `cap-std` is an *opt-in library convention*, not a compiler- or runtime-enforced boundary. Nothing stops a dependency from ignoring `cap-std` and calling `std::fs::File::open("/etc/passwd")` directly, or from writing `unsafe` code that reaches around any capability discipline entirely. It raises the bar for *cooperative* code but does not sandbox uncooperative/malicious code the way SES Compartments or WASI's host-enforced imports do. This is the precise distinction the user asked to be careful about: `cap-std` is closer to a "capability-safe API design pattern" than to an enforced object-capability boundary.
- **Sandboxed build scripts / proc-macros (in progress, not shipped)**: this is an active, official Rust Project Goal — "Explore sandboxed build scripts" — aiming to restrict `build.rs` (and eventually proc-macros) from filesystem/network/process access by default unless explicitly permitted, using a swappable sandbox runtime (cross-platform via Landlock on Linux, Seatbelt on macOS), with crates.io eventually surfacing declared permission requirements. As of the check, **no solution has shipped**; proc-macro sandboxing (e.g., via `watt`, compiling macros to Wasm) is experimental, and there's no production-ready, default-on capability boundary for arbitrary Rust dependency code as of 2026. ([rust-lang.github.io/rust-project-goals — Explore sandboxed build scripts](https://rust-lang.github.io/rust-project-goals/2024h2/sandboxed-build-script.html), [internals.rust-lang.org discussion threads on build.rs/proc-macro sandboxing](https://internals.rust-lang.org/t/sandbox-build-rs-and-proc-macros/16345))

**Conclusion for Rust**: as of 2026, Rust has *no* shipped, default-on, transitive-dependency capability enforcement equivalent to LavaMoat/SES. `cap-std` is a useful voluntary discipline; `cargo-vet`/`cargo-crev`/`cargo-auditable` are audit/provenance tools, not sandboxes; real sandboxing of build-time code (the highest-risk moment, matching npm's install-script problem) is an unshipped project goal. The realistic way to get real enforcement in the Rust ecosystem today is to go through WebAssembly (compile dependencies — or the whole app — to Wasm components and run them under a capability-enforcing host like Wasmtime; see 3c).

### 3c. WebAssembly Component Model + WASI (real runtime enforcement)

This is the strongest "real enforcement" story found in the research, and it's language-agnostic (not Rust-specific).

- **WASI Preview 2 ("WASI 0.2")** redesigned system access as capability-based interfaces (WIT — WebAssembly Interface Types) instead of ambient POSIX-style syscalls. "WASI applications run in a capability-based sandbox: a Wasm module or component starts with no ambient authority and can only do what the host explicitly grants." A component's imports are declared statically in WIT — "a component that only imports `wasi:io/streams` and `wasi:http/incoming-handler` cannot secretly open a raw socket." ([wasi.dev](https://wasi.dev/), cross-checked against [component-model.bytecodealliance.org](https://component-model.bytecodealliance.org/running-components/wasmtime.html))
- **The Component Model** is the composition layer on top of core Wasm: it lets you compose multiple sandboxed components with typed interfaces, each with its own explicit import/export surface, without exposing raw linear-memory layouts between components (unlike raw core-Wasm linking). ([component-model.bytecodealliance.org](https://component-model.bytecodealliance.org/running-components/wasmtime.html))
- **Wasmtime** (Bytecode Alliance) is the reference-speed implementation of both WASI and the Component Model, and supports fuel metering (execution-time caps) plus strict WASI compliance for portable sandbox definitions. ([docs.wasmtime.dev](https://docs.wasmtime.dev/), [wasi.dev](https://wasi.dev/))
- **Production track record**: Fastly and Fermyon run untrusted tenant code on Wasmtime in production; Shopify Functions likewise runs untrusted merchant/partner code on Wasmtime. This is a materially stronger "battle-tested at scale, for exactly this threat model (running code you don't fully trust with least authority)" story than LavaMoat has outside MetaMask. ([wasmruntime.com tutorial cross-checked against Bytecode Alliance materials](https://wasmruntime.com/en/tutorials/wasmtime))

**What this means architecturally**: this is *not* "pick a language and get this for free." It means: compile components (potentially written in Rust, JS via a JS-to-Wasm engine, Go, C, etc.) and run them inside a Wasm host (Wasmtime or similar) that enforces capability-scoped imports at the host boundary — real, OS/runtime-enforced isolation, independent of whether the guest language cooperates. This is architecturally different from LavaMoat/SES (which harden the *JS realm itself* so ordinary in-process JS objects are the capability boundary) — Wasm's boundary is the linear-memory/host-call boundary, which is a *harder* boundary (memory-safe by construction, not dependent on the guest language's cooperation with freezing/proxying), at the cost of language/tooling friction (compiling to Wasm, WASI-target support maturity per language, cross-boundary calling conventions, and it's a much bigger architectural commitment than adding a JS build step).

### 3d. Other ecosystems, briefly

- No comparably mature, shipped, default-on capability-enforcement system was found for Python, Go, or JVM-ecosystem dependency graphs in this research pass — those ecosystems' supply-chain tooling (e.g., pip-audit, Go's module checksum database, JVM SecurityManager which is deprecated/removed as of recent JDKs) is overwhelmingly audit/provenance-shaped, not runtime-sandboxing-shaped, similar to Rust's `cargo-vet`/`cargo-audit`. This wasn't exhaustively re-verified against primary sources for this report (out of the explicit scope named by the user — JS/Node, Rust, Wasm) and should be treated as a lower-confidence aside, not a citation-backed claim.

---

## 4. Bottom line: ranked options, with the ecosystem lock-in each implies

Ranked by how directly and maturely each realizes "a dependency and its transitive dependencies can only access capabilities explicitly granted to it," for a long-lived (not build-step-only) application, as of 2026:

1. **JS/Node + SES/Endo (Compartments) as the core primitive, with LavaMoat's `policy.json`/kernel layered on where its CJS-oriented tooling fits, and hand-rolled `@endo/compartment-mapper` Compartments where it doesn't (ESM).**
   Locks the project into: **JavaScript/TypeScript on Node (or a Node-compatible runtime)**.
   Why #1: this is the only option in the survey where the *enforcement primitive itself* (SES lockdown + Compartments) is mature, in-process, alive for the whole app lifetime (not just a build/install step), has a genuine object-capability model (least authority by default, capabilities passed not reached-for), and has a real, load-bearing production deployment (MetaMask, tens of millions of users) plus a second independent adopter (Agoric) driving the underlying spec through TC39. LavaMoat's own tooling (auto-policy, `allow-scripts`) is a strong accelerant for the common (CJS) case but is not itself the source of the guarantee — SES/Compartments is — so the CJS/ESM gap in LavaMoat is a tooling inconvenience to work around (manual Compartment wiring for ESM packages, or contribute/wait for ESM policy support), not a fundamental blocker to the property itself.

2. **WebAssembly Component Model + WASI Preview 2, hosted in Wasmtime (or similar), with dependencies (or isolable subsystems) compiled to Wasm components.**
   Locks the project into: **not a single language** — locks into the Wasm Component Model + a component-capable toolchain per language used (Rust has the best current support via the Bytecode Alliance's own tooling; other languages vary in WASI-target maturity), plus embedding a Wasm host runtime inside the application. This is architecturally heavier (you're not just "writing the app," you're building a host + guest-component architecture) but the enforcement is arguably *stronger* than SES's (host/linear-memory boundary vs. in-realm object freezing) and has serious production track record for exactly this threat model (Fastly, Shopify, Fermyon running untrusted code). Realistic as a first-class constraint only if the team is willing to take on Wasm-host-application architecture as a foundational decision, not an add-on.

3. **Deno's process-level permission model, alone.**
   Locks into: JavaScript/TypeScript on Deno specifically.
   Real, mature, well-documented, but explicitly **does not provide transitive per-dependency separation** — it's a whole-process boundary against the OS, and Deno's own docs are explicit that all code on a thread shares one privilege level. Only useful for this requirement if combined with SES/Compartments *inside* the Deno process (at which point you're really back in option 1's territory, just running under Deno instead of Node) or with manual Worker-per-untrusted-dependency partitioning, which doesn't scale to "every transitive dependency."

4. **Rust with `cap-std` + eventual sandboxed build scripts, once/if that project goal ships.**
   Locks into: Rust.
   Currently **not a real contender for this specific requirement** — `cap-std` is opt-in convention a malicious/careless dependency can simply ignore, and default-on build-time (let alone runtime, cross-dependency) sandboxing is an acknowledged, unshipped, in-progress Rust project goal as of 2026. Worth re-evaluating later if that goal ships, but not something to architect around today. (Rust remains attractive for other reasons — e.g., as the implementation language for Wasm components in option 2 — just not as a standalone answer to this requirement.)

5. **Audit/trust tooling alone — `cargo-vet`, `cargo-crev`, `cargo-auditable`, Socket.dev's scanning/threat-feed/firewall.**
   Explicitly **not equivalent** to the requested property and should not be treated as satisfying it, regardless of ecosystem. These tools answer "has this code been reviewed / is this package known-bad," which is valuable defense-in-depth (and worth adopting *in addition to* whichever option above is chosen — e.g., Socket.dev alongside SES/LavaMoat in the npm ecosystem is a reasonable belt-and-suspenders combination) but provides zero runtime restriction on what an *allowed* dependency can subsequently do. Conflating this category with options 1–3 would be a real architectural mistake.

**Net recommendation shape** (not a final decision, just what the evidence supports): if transitive object-capability enforcement is genuinely a first-class, load-bearing constraint, the two live options are (1) JS/TS on Node with SES/Compartments as the actual enforcement layer (LavaMoat where its CJS tooling helps, hand-rolled Compartments via `@endo/compartment-mapper` for ESM gaps) — lower architectural cost, proven at MetaMask's scale, single-ecosystem; or (2) a Wasm Component Model architecture on Wasmtime — stronger enforcement boundary and strong unrelated-industry production track record, but a materially bigger architectural commitment (a host/guest-component application shape) and no single "just write normal app code" language. Rust-without-Wasm and Deno-alone do not currently satisfy the stated requirement; audit tooling in any ecosystem never does.

---

## Sources index

- LavaMoat repo/README: https://github.com/LavaMoat/LavaMoat and https://github.com/LavaMoat/LavaMoat/blob/main/README.md
- LavaMoat policy guide: https://lavamoat.github.io/guides/policy/
- LavaMoat FAQ: https://lavamoat.github.io/about/faq/
- LavaMoat releases: https://github.com/LavaMoat/LavaMoat/releases
- LavaMoat commit history: https://github.com/LavaMoat/LavaMoat/commits/main
- MetaMask on LavaMoat: https://metamask.io/news/security/using-lavamoat-to-solve-software-supply-chain-security/ and https://metamask.io/news/lavamoat-and-the-ledger-software-supply-chain-attack
- Endo repo: https://github.com/endojs/endo
- SES README: https://github.com/endojs/endo/blob/master/packages/ses/README.md
- Hardened JavaScript (Agoric docs): https://docs.agoric.com/guides/js-programming/hardened-js
- `@endo/compartment-mapper`: https://www.npmjs.com/package/@endo/compartment-mapper
- TC39 Compartments proposal: https://github.com/tc39/proposal-compartments
- TC39 SES proposal: https://github.com/tc39/proposal-ses
- Socket.dev docs: https://docs.socket.dev/ and https://docs.socket.dev/docs/threat-feed
- Socket Firewall: https://docs.socket.dev/docs/socket-firewall-overview, https://docs.socket.dev/docs/socket-firewall-free, https://docs.socket.dev/docs/socket-firewall-enterprise
- Deno security/permissions docs: https://docs.deno.com/runtime/fundamentals/security/ and https://docs.deno.com/runtime/reference/permissions/
- `cargo-vet`: https://mozilla.github.io/cargo-vet/ and https://github.com/mozilla/cargo-vet
- `cargo-auditable`: https://github.com/rust-secure-code/cargo-auditable
- `cap-std`: https://github.com/bytecodealliance/cap-std/blob/main/cap-std/README.md, intro post https://blog.sunfishcode.online/introducing-cap-std/
- Rust sandboxed build scripts project goal: https://rust-lang.github.io/rust-project-goals/2024h2/sandboxed-build-script.html
- Rust internals discussion on build.rs/proc-macro sandboxing: https://internals.rust-lang.org/t/sandbox-build-rs-and-proc-macros/16345
- WASI: https://wasi.dev/
- WebAssembly Component Model (Bytecode Alliance): https://component-model.bytecodealliance.org/running-components/wasmtime.html
- Wasmtime docs: https://docs.wasmtime.dev/

## Confidence notes / gaps

- LavaMoat adoption beyond MetaMask: could not find named, citable confirmation of other production users as of 2026; treat "MetaMask-only confirmed" as the honest state of evidence, not proof of non-adoption elsewhere.
- The ESM-support gap in LavaMoat's policy generation is confirmed via search results referencing an open endojs/endo issue tracking "LavaMoat support for ECMAScript modules"; I was not able to open that specific issue directly to quote it verbatim — treat the existence/substance of the gap as fairly confident (converged from multiple independent search snippets) but the exact issue number/status as lower-confidence.
- Python/Go/JVM supply-chain tooling (section 3d) was intentionally not deep-dived — out of the user's named scope (JS/Node, Rust, Wasm) — and is flagged as lower-confidence/not primary-sourced.
