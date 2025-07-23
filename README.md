# docgen-rs

A modern, TUI-based documentation assistant for Rust projects, powered by [Ratatui](https://github.com/ratatui-org/ratatui). docgen-rs helps developers visualize, analyze, and document code changes efficiently, with a professional and intuitive terminal interface.

## Features

- **📊 Module Impact Heatmap:**
  - Visualize changed files/modules from Git diffs.
  - See lines added/removed, functions modified, and impact bars color-coded by change weight.

- **⚙️ Code Change Flow Diagram:**
  - Step-by-step horizontal diagram: [Git Diff] → [Metadata Form] → [LLM Analysis] → [Markdown Gen] → [PDF Export].
  - Highlights the current step and animates transitions.

- **💸 LLM Token Cost Estimator:**
  - Estimates token usage for diffs and form answers.
  - Color-coded gauge (green/yellow/red) for cost awareness.

- **📝 Metadata Form:**
  - Guided form for describing code changes, DB changes, extensibility, and performance notes.

- **Professional TUI Design:**
  - Clean, bordered panels with headings and keyboard navigation.
  - Tabs for switching between Form, Git Diff Viewer, and LLM Stats.

## Screenshots

> _Add screenshots here after running the app!_

## Usage

### Run the TUI
```sh
cargo run
```

### Keyboard Navigation
- **←/→**: Switch between Form, Git Diff Viewer, and LLM Stats
- **↑/↓**: Navigate form steps (when in Form view)
- **Esc**: Quit

## Development

### Prerequisites
- Rust (edition 2021 or later)
- [Ratatui](https://github.com/ratatui-org/ratatui) crate (see `Cargo.toml`)
- [crossterm](https://github.com/crossterm-rs/crossterm) for terminal handling

### Project Structure
- `src/tui/`
  - `app.rs`: Application state and enums
  - `layout.rs`: Layout logic for the TUI
  - `widgets.rs`: All custom widgets and panels
  - `events.rs`: Keyboard and input handling
- `src/main.rs`: Entry point and terminal setup

### Customization
- The TUI is modular and ready for expansion:
  - Connect real Git diffs and LLM APIs
  - Add more panels or export options
  - Refine the UI with more Ratatui widgets

## License

MIT
