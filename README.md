# `exocortex`

say more here

## Design Aspirations

I am not a design expert but have a few principles I'd like to aim for:

- All UI is designed to be immediately responsive regardless of latency, as much as possible. The easy/obvious architectural joint here is the interface between `egui` and `..._memory::Provider`. This principle should mean that even if disk I/O has heavy latency but RAM/gpu memory/processing power is relatively un-bottlenecked, the UI will remain responsive.
- All UI is designed to make latency legible. Every widget has indications as to whether or not the state is synced or just in-memory.
- No widget is ever allowed to interrupt the user's attention by moving it's location on screen, appearing suddenly, disappearing suddenly, or changing or replacing text or images suddenly, or using excessive motion or "visually loud" attention grabbing such as flashing _unless_ this is in direct immediate response to a positive user-initiated action.
  - Example: the user presses the "show me the key bindings help" button, so it's ok for the key bindings help widget to pop up _over_ the previously active widgets (or more nicely, it is allowed to move them over a bit to display itself, while preserving as much of the visual context of other widgets as possible).
  - Example 2: a network message comes in signifying an important change to multiple items in view of the user. Those widgets are allowed to _gently_ alter their appearance to signify some new state has changed due to a non-user-initiated event, but this cannot include resizing the widget, flashing, turning a bright color, etc...
  - Putting this principle and those examples together, when a user sees the gentle indication that some external event has updated something about the state on screen, the user can then explicitly investigate what's new by prompting a widget to expand / change text / disappear / etc... via key shortcuts or touches or mousing or speech or eye gestures.
- For platforms with keyboards, everything is keyboard first.
  - Desktop vs mobile UI will be largely different based on the ergonomics of that device format. I haven't thought much about this yet and only vaguely have aspirations for a mobile variant.

## Inspirations and Related Apps

- RoamResearch introduced me to the "Personal Knowledge Management" genre.
- LogSeq, along with a rich tradition of open source local first decentralized tools and toolmakers and hackers from the dawn of computing.
- AnyType and/or LogSeq are probably better than `exocortex` for now (and I'm exploring the use of both). If `exocortex` fulfills my vision, it will be more "hardwired" and less "flexible" than those apps, in exchange for smoother flow.
  - The philosophy is a bit different. I see the philosophy of AnyType / LogSeq as "empower many users to do their own PKM and collaborative PKM". Meanwhile `exocortex` is "help me do very specific flows to improve my cognitive agency."
