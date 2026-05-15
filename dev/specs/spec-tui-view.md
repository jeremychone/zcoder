# TUI View Specification

## Intent

Define the visual structure, render behavior, and view module organization for the terminal UI.

The view renders `AppState` into a terminal frame using `ratatui`. It provides:

- a header
- an answer or error content area
- a status line
- a prompt input area
- a footer with key hints
- a module structure for run views, task views, config, install states, popups, components, styles, and view facades

The scope covers `src/tui/view/` and its submodules. It does not handle user input, state mutation, executor events, or terminal lifecycle.

Primary interface:

```rust
pub fn render(f: &mut Frame, state: &AppState);
```

## Code Design

The view module is organized as a top-level registry plus focused render modules:

```text
src/tui/view/
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

`src/tui/view/mod.rs` is the view module registry and public re-export surface:

```rust
mod facade;

mod action_view;
mod config_view;
mod install_view;
mod main_view;
mod popup_view;
mod run_main_view;
mod run_overview;
mod run_tasks_view;
mod runs_nav_view;
mod runs_view;
mod sum_view;
mod support;
mod task_view;

pub use action_view::*;
pub use config_view::*;
pub use install_view::*;
pub use main_view::*;
pub use popup_view::*;
pub use run_main_view::*;
pub use run_overview::*;
pub use run_tasks_view::*;
pub use runs_nav_view::*;
pub use runs_view::*;
pub use sum_view::*;
pub use task_view::*;

pub mod comp;
pub mod style;
```

Module responsibilities:

- `main_view.rs`
  - owns the top-level layout
  - renders background and core content
  - delegates sections to focused view modules
  - renders overlays last
- `action_view.rs`
  - renders action bars, key hints, and available commands
- `runs_view.rs`
  - renders the list of known runs when run history exists
- `runs_nav_view.rs`
  - renders run navigation and selected run summary
- `run_main_view.rs`
  - renders the selected run detail screen
- `run_overview.rs`
  - renders run metadata and high-level status
- `run_tasks_view.rs`
  - renders task rows for the selected run
- `task_view.rs`
  - renders selected task details
- `sum_view.rs`
  - renders compact summary content
- `config_view.rs`
  - renders configuration UI and config popups
- `install_view.rs`
  - renders install or first-run setup workflows
- `popup_view.rs`
  - renders transient and user-dismissed popup overlays
- `comp/`
  - stores reusable widgets and small UI components
- `style/`
  - stores color, style, and theme helpers
- `facade/`
  - stores view-facing data shaping helpers when raw state needs conversion before rendering
- `support/`
  - stores view-local rendering utilities

The initial prompt UI uses a vertical layout with five sections:

- header, fixed height of 3
- content, flexible height
- status, fixed height of 1
- input, fixed height of 3
- footer, fixed height of 1

Layout constraints:

```rust
[
	Constraint::Length(3),
	Constraint::Min(0),
	Constraint::Length(1),
	Constraint::Length(3),
	Constraint::Length(1),
]
```

Header behavior:

- renders the application name `zcoder`
- uses a bordered block
- uses cyan foreground styling

Content behavior:

- shows `Error: {err}` when `state.last_error()` exists
- otherwise shows `state.last_answer()` when available
- otherwise shows `No answer yet. Type a prompt and press Enter.`
- uses a bordered block titled `AI Answer`
- wraps text with trimming enabled

Status behavior:

- renders `Status: {state.status()}`
- uses red when an error exists
- uses yellow while waiting
- uses green when idle and no error exists

Input behavior:

- renders the current prompt buffer from `state.input()`
- uses a bordered block titled `Prompt (/q to quit)`
- uses dark gray while waiting
- uses default style when editable

Footer behavior:

- renders key hints for sending and quitting
- shows `[Enter] Send`, `[/q] Quit`, and `[Ctrl-c] Quit`

Layering order for the full TUI:

- render the base background
- render catastrophic system error fallback when present
- render header and main content
- render action bar
- render config or install popup when active
- render final popup overlay last

## Design Considerations

The view is a pure render layer over `AppState`. This separation keeps visual decisions independent from input handling and executor communication.

Fixed heights are used for header, status, input, and footer so the content area can absorb terminal resizing and answer length variation.

The content area prioritizes errors over answers because errors require immediate attention and explain failed prompt runs.

The input area is dimmed while waiting to communicate that prompt submission is temporarily disabled by the core event logic.

The footer keeps available key actions visible without requiring a separate help screen.

The AIPack-like view structure keeps `main_view.rs` as a layout coordinator instead of a catch-all renderer. Focused modules make it easier to add runs, tasks, config, install flows, and popups without mixing unrelated rendering logic.

Reusable components, style helpers, facades, and support utilities are kept in separate folders so view modules can share behavior without depending on core event handling or executor internals.
