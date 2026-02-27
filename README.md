# tuiznos

A toy TUI (Terminal User Interface) program built with [ratatui](https://ratatui.rs/) and
[tui-textarea](https://github.com/rhysd/tui-textarea). This is a learning project — most of the
source code is adapted from the ratatui and tui-textarea examples. The goal is to understand how
these libraries work by reading, modifying, breaking, and testing the code.

## What it does

The app renders a split terminal window:

- **Top half** — a static output box (a `Paragraph` widget), placeholder for future AI output
- **Bottom half** — an editable text area with vim-like keybindings

The text area supports Normal, Insert, Visual, and Operator modes, and the block title/cursor color
change depending on which mode is active.

If a file path is passed as a command-line argument, the textarea loads that file's contents on
startup.

## Running

```
cargo run
```

Or with a file:

```
cargo run -- path/to/file.txt
```

## Keybindings (Normal mode)

| Key         | Action                                  |
|-------------|-----------------------------------------|
| `q`         | Quit                                    |
| `i`         | Enter Insert mode                       |
| `a`         | Move forward one char, enter Insert     |
| `A`         | Move to end of line, enter Insert       |
| `I`         | Move to start of line, enter Insert     |
| `o`         | New line below, enter Insert            |
| `O`         | New line above, enter Insert            |
| `h j k l`  | Move cursor left/down/up/right          |
| `w`         | Move forward by word                    |
| `b`         | Move backward by word                   |
| `e`         | Move to end of word                     |
| `^`         | Move to start of line                   |
| `$`         | Move to end of line                     |
| `gg`        | Move to top of buffer                   |
| `G`         | Move to bottom of buffer                |
| `x`         | Delete character under cursor           |
| `D`         | Delete to end of line                   |
| `C`         | Delete to end of line, enter Insert     |
| `u`         | Undo                                    |
| `Ctrl+r`    | Redo                                    |
| `p`         | Paste                                   |
| `v`         | Start Visual mode (character)           |
| `V`         | Select current line (Visual mode)       |
| `y/d/c`     | Enter Operator mode (yank/delete/change)|
| `Ctrl+d`    | Half page down                          |
| `Ctrl+u`    | Half page up                            |
| `Ctrl+f`    | Page down                               |
| `Ctrl+b`    | Page up                                 |
| `Ctrl+e`    | Scroll down one line                    |
| `Ctrl+y`    | Scroll up one line                      |

**Insert mode:** `Esc` or `Ctrl+c` returns to Normal mode. All other keys use the default
tui-textarea mappings (regular text input, backspace, arrow keys, etc.).

**Visual mode:** `y` yanks, `d` deletes, `c` cuts and enters Insert, `Esc` or `v` returns to
Normal.

### Key types

- **`App`** — owns the terminal handle and drives the draw/event loop
- **`Vim`** — a state machine that takes a raw `Input` event and returns a `Transition`
- **`Transition`** — what the app should do next: change mode, stay put, queue a pending key, or quit
- **`Mode`** — the current editing mode; also owns the block title and cursor colour for that mode
- **`TxtArea`** — thin wrapper around `TextArea<'static>` that applies mode-specific styling
- **`OutputBox`** — a `Paragraph` widget with rounded borders, intended to eventually display AI responses

## Dependencies

| Crate         | Purpose                                      |
|---------------|----------------------------------------------|
| `ratatui`     | Terminal rendering framework                 |
| `tui-textarea`| Editable text widget with vim key support    |
| `crossterm`   | Cross-platform terminal backend              |
| `color-eyre`  | Error reporting (pulled in, not yet wired up)|

## TODOs left in the code

- Wire up `Key::Enter` in Normal mode to pass textarea content to the output box
- Make the output box dynamic (display actual responses)
- Test and clean up the `cleanup()` function
- Remove the debug `println!` that dumps textarea lines on exit

## Notes on learning

The vim key-handling logic (`src/input/vim.rs`) is taken almost verbatim from the tui-textarea
vim example. Breaking it down:

- `Vim::transition()` receives a single `Input` (key + modifier flags) and a mutable reference to
  the `TextArea`, then pattern-matches on both the current mode and the key to decide what to do.
- Two-key sequences like `gg` are handled via a `pending` field — the first `g` returns
  `Transition::Pending(input)` and is stored; the next event checks `self.pending` before matching.
- Operator mode (`y`, `d`, `c`) works by entering `Mode::Operator(char)`, starting a selection,
  letting the next motion key extend it, and then performing the operation on the selection.
