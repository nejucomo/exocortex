Type: grilling
Status: open

Blocked by: 02, 03

## Question

Decide how exocortex satisfies the settled requirement (see map Notes) that multiple front-end processes — the GUI and/or non-interactive CLI subcommand invocations — can run concurrently against the same persistent data on one laptop. Candidate shapes: a daemon/service process owning the data with front-ends talking to it (and if so, over what IPC — Unix socket, local RPC, etc.), a multi-process-safe embedded store that front-ends open directly (e.g. an engine with real concurrent-writer support), or another mechanism. Decide in light of the language/runtime (ticket 02) and data model shape (ticket 03) already chosen.
