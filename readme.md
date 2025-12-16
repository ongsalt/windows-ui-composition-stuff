gui library (again)

# Architecture

we basically have 2 tree (or 3 if using react-like api): widget tree + compositor tree (and optionally vdom)

- User facing API: gonna look like swiftui or svelte, react is also possible but no
  - signal
  - This must send ui tree patching command to layer below
  - this might be pain in the ass, at first im thinking of doing the entire thing in swift so its not that hard. but now...
  - or ffi???
- Backing view tree - look like dom
  - we do layout here
  - TODO: flexbox layouting
  - backed by `Windows.UI.Composition`

## Animation api

- we should offload task as much as possible to the compositor
- think off user-facing api...

# Todos

- stop relying on `windows_numerics`
- node arena
- Event handling
  - parse it (might just use winit at this point)
  - dispatch it
- Hit testing and layouting
- background brush
- shadow
- DPI handling...
- Expose animation api