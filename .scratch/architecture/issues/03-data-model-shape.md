Type: grilling
Status: open

Blocked by: 02

## Question

Decide the shape of exocortex's core data model, now that language/runtime is fixed ([ticket 02](02-choose-language-runtime.md)). Two entangled questions:

1. Event-sourced append-only log of facts/events vs. mutable state with snapshots.
2. One shared cross-feature primitive (e.g. a generic "item" or "event" type that journal, calendar, tasks, and PKM all specialize) vs. independent per-feature schemas unified only at the storage layer.
