# Exocortex: Overall Architecture

## Destination

A written architecture spec at `docs/architecture.md` that synthesizes the foundational decisions for exocortex's shared substrate — language/runtime, local-first storage approach, data model shape, and process/module boundaries — each backed by an ADR in `docs/adr/`. Decision-level only (what was chosen and why; no concrete file layouts or code). Scoped to the substrate all four feature areas (journal, calendar, tasks, PKM) will stand on; each feature's own internal design is a separate future effort. The map closes when every substrate decision is a closed ticket with an ADR, `docs/architecture.md` synthesizes them, and a final review ticket confirms the synthesis holds together.

## Notes

- **Greenfield restart, GitHub state ignored.** exocortex had a prior Rust/eframe implementation on GitHub (issues, branches, a draft Copilot restart PR). All of that — and any local git history before the "rebirth" commit (`c8842fd`) — is deliberately disregarded. Don't consult it as prior art; treat this as a fresh start.
- Consult `docs/agents/domain.md` and `docs/agents/issue-tracker.md` for this repo's conventions (single-context domain docs, local-markdown issue tracker under `.scratch/`).
- Use `/grilling` and `/domain-modeling` for decision tickets unless a ticket says otherwise.
- **Platform/interfaces (settled, not open):** v1 targets a single laptop. Front-ends are a GUI and a *non-interactive* CLI — each CLI subcommand is a short-lived process performing one action with output on stdout (no REPL/TUI). The architecture must support multiple front-end processes (GUI and/or CLI invocations) running concurrently against the same persistent data — this implies a daemon/service process, a concurrency-safe multi-process-capable store, or another mechanism meeting that goal. This constraint shapes tickets 03–05 but is itself decided.
- **Multi-device sync is out of scope** for this map (see Out of scope).
- **Object-capability security is a load-bearing constraint on language/runtime choice** (ticket 01, resolved) — not a nice-to-have layered on afterward.

## Decisions so far

- [Research: transitive object-capability security across dependencies](issues/01-object-capability-security-research.md) — SES/Endo Compartments on Node (LavaMoat's tooling where it fits CJS) is the strongest real option; WASM Component Model is a heavier but stronger alternative; Rust's `cap-std` and all audit-only tools (cargo-vet, Socket.dev) don't actually satisfy the requirement. Full findings: `.scratch/architecture/research/object-capability-security.md`.

## Not yet specified

- Concrete storage mechanism, once language/runtime and data model shape are fixed (ticket 05).
- Data model shape: event-sourced append-only log vs. mutable+snapshots; one shared cross-feature primitive (an "item"/"event" type journal, calendar, tasks, and PKM all specialize) vs. independent per-feature schemas unified only at the storage layer (ticket 03).
- Process/module boundary mechanics beyond "must support concurrent multi-frontend access to shared data": daemon protocol vs. multi-process-safe store, IPC mechanism if a daemon is chosen (ticket 04).
- Packaging/distribution target and testing philosophy — deferred to a follow-on effort once the substrate is fixed; not part of this map's destination.
- Each feature's own internal architecture (journal UI, calendar logic, task semantics, PKM linking) — future effort once substrate settled.

## Out of scope

- Multi-device sync — deferred to a later effort; v1 is single-laptop, multi-process-on-one-machine only.
- The prior Rust/eframe exocortex codebase and its GitHub issues/branches/draft restart PR — intentionally disregarded; this is a greenfield restart, not a continuation.
