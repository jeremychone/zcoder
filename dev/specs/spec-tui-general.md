# TUI General Specification

## Intent

Define the terminal UI lifecycle, module boundaries, and integration shape for the `zcoder` interactive CLI.

The TUI provides:

- terminal initialization and restoration
- a central app event channel
- terminal input forwarding
- executor status forwarding
- a UI loop that renders state and dispatches user actions
- a full module structure that can grow toward the AIPack TUI organization

The scope covers the top-level TUI module, the core runtime modules, shared support modules, and view module registry. It does not define executor internals, AI provider behavior, or individual file-change application logic.

Visible entry point:

```rust
pub async fn start_tui(
	executor_tx: ExecutorTx,
	status_rx: Receiver<ExecStatusEvent>,
	initial_prompt: Option<String>,
) -> Result<()>;
```

## Code Design

The TUI is organized around three top-level areas:

```text
src/tui/
  mod.rs
  core/
  support/
  view/
```

`src/tui/mod.rs` is the top-level module registry and public entry surface. It should keep the public API small:

```rust
mod core;
mod support;
mod view;

pub use core::start_tui;
use core::*;
use view::*;
```

The full target structure mirrors the AIPack TUI shape while staying scoped to `zcoder` behavior:

```text
src/tui/
  mod.rs
  core/
    mod.rs
    app_event_handlers.rs
    ping_timer.rs
    term_reader.rs
    tui_impl.rs
    tui_loop.rs
    app_state/
      mod.rs
      app_state_base.rs
      state_processor.rs
    event/
      mod.rs
      app_event.rs
    types/
      mod.rs
  support/
    mod.rs
    formatters.rs
    number_utils.rs
    ui_ext.rs
  view/
    mod.rs
    action_view.rs
    config_view.rs
    install_view.rs
    main_view.rs
    popup_view.rs
    run_main_view.rs
    run_overview.rs
    run_tasks_view.rs
    runs_nav_view.rs
    runs_view.rs
    sum_view.rs
    task_view.rs
    comp/
      mod.rs
    facade/
      mod.rs
    style/
      mod.rs
    support/
      mod.rs
```

Main responsibilities:

- `src/tui/mod.rs`
  - registers the `core`, `support`, and `view` modules
  - exports only the TUI entry point and intentional public types
- `src/tui/core/mod.rs`
  - registers the runtime modules
  - flattens selected core exports such as `start_tui`, `AppState`, `AppEvent`, and channel wrapper types
- `src/tui/core/tui_impl.rs`
  - owns terminal setup and teardown
  - creates internal app channels
  - starts terminal reader and status forwarder tasks
  - starts the UI loop
- `src/tui/core/tui_loop.rs`
  - draws before handling each event
  - receives all app, terminal, tick, and executor events from one stream
  - delegates event interpretation to app event handlers or state processors
  - sends executor actions through `ExecutorTx`
  - controls quit behavior and loop lifecycle
- `src/tui/core/term_reader.rs`
  - reads terminal input and forwards it as `AppEvent` values
  - uses a short fused delay and fused terminal event future inside a `tokio::select!` loop, matching the AIPack terminal reader pattern
    - safe fused select behavior, non-busy polling, graceful shutdown, and tolerance for late terminal events during quit.
  - exits cleanly when the app event sender is closed, and avoids treating a late terminal resize after quit as a hard failure
- `src/tui/core/event/`
  - defines the central `AppEvent` boundary and semantic `AppActionEvent` values
- `src/tui/core/app_state/`
  - owns renderable state and state transitions
  - keeps executor internals out of UI state
- `src/tui/core/types/`
  - stores small shared TUI enums and aliases
- `src/tui/support/`
  - stores formatting and utility helpers shared across views and core
- `src/tui/view/`
  - renders `AppState`
  - owns layout composition, reusable components, style constants, and view facades

Event flow:

```text
Terminal input -> AppEvent::Term -> tui_loop
User intent -> AppEvent::Action -> app_event_handlers/state_processor -> tui_loop
Executor status -> AppEvent::Exec -> tui_loop
Timer tick -> AppEvent::Tick -> tui_loop
Redraw request -> AppEvent::DoRedraw -> tui_loop
```

Terminal lifecycle:

- `ratatui::init()` is called before the UI loop starts.
- The terminal is cleared before the first render.
- Mouse capture is paired with TUI setup and teardown when mouse handling is enabled.
- `ratatui::restore()` is called after the UI loop exits.
- The result from the UI loop is returned after terminal restoration.

## Design Considerations

The design uses one app event stream so the UI loop can process terminal input, internal actions, ticks, and executor updates in a consistent order.

The terminal reader follows the AIPack delay and fuse pattern. Each loop creates a short `Delay`, fuses it, fuses the next terminal event future, and selects between them. The delay branch intentionally does nothing. This gives the reader a periodic cooperative wake point instead of waiting indefinitely on one terminal event future, keeps shutdown behavior predictable as the TUI evolves, and avoids busy polling. Fusing the futures makes the select branches safe after completion and prevents accidental repolling of a completed future.

The module structure separates runtime coordination, state mutation, support utilities, and rendering. This keeps the current simple prompt UI easy to understand while giving the codebase clear expansion points for run views, task views, popups, config, installation workflows, and reusable components.

Terminal setup is isolated in `tui_impl.rs` so the UI loop remains focused on event coordination. This also keeps terminal cleanup localized and easier to preserve on errors.

The executor boundary is preserved from the first TUI shape. The TUI loop does not perform long-running work, model requests, file loading, extraction, application, installs, or checks. It only sends typed executor actions and reacts to executor lifecycle events.

The AIPack-like structure is intentionally modular rather than flat. It avoids turning `tui_loop.rs` or `view.rs` into catch-all files as TUI behavior grows.
