# raddio

A tiny terminal launcher: tune into a **station**, type some input, and raddio runs a
command with that input. Think of it as a minimal, configurable prompt box — small enough
to drop into a [zellij](https://zellij.dev/) floating pane and wire up to a keybinding.

> **Status:** early / work in progress. The README is an initial draft.

## What it does

A *station* is a named entry in your config that pairs a short description with a command
template. When you launch a station, raddio shows an input box; whatever you
type is substituted into the template's `{}` placeholder, and the command runs after the UI
closes.

```
raddio run "switch session"
        │        │
        │        └─ station name (from your config)
        └─ subcommand
```

Example: a station whose command is `["zellij", "action", "switch-session", "{}"]` becomes
"type a session name, press Enter, switch to it."

## Install

Build from source (requires a recent Rust toolchain):

```sh
cargo build --release
# binary at target/release/raddio

# or install onto your PATH
cargo install --path .
```

A [Nerd Font](https://www.nerdfonts.com/) is recommended for the default prompt icon (see
[`prefix`](#uiconfig-fields)).

## Quick start

```sh
# 1. Create a default config
raddio init

# 2. Edit it (see Configuration below)
$EDITOR ~/.config/raddio/config.json

# 3. Launch a station
raddio run "station name"
```

## Commands

| Command                 | Description                                                        |
| ----------------------- | ------------------------------------------------------------------ |
| `raddio init`           | Write a default config file. Will **not** overwrite an existing one. |
| `raddio run <station>`  | Open the input box for `<station>` and run its command.            |
| `raddio --help`         | Show help. `raddio <command> --help` for per-command help.         |
| `raddio --version`      | Print the version.                                                 |

## Key bindings

The input box is modeless — just start typing. It uses
[edtui](https://crates.io/crates/edtui) in single-line mode, so most Emacs-style editing
keys work (e.g. `Ctrl+A`/`Ctrl+E`, `Ctrl+W`). Text scrolls horizontally when it overflows.

| Key                    | Action                                              |
| ---------------------- | --------------------------------------------------- |
| *(type)*               | Edit the input                                      |
| `Enter`                | Submit — run the station command and exit           |
| `Esc` / `Ctrl+C`       | Quit without running anything                       |
| `Ctrl+H` / `Ctrl+L`    | Move cursor left / right                            |
| `←` / `→`, `Backspace` | Standard editing                                    |

If you press `Enter` on an empty input, the station's `default` value (if set) is used.

### Multiline mode

When a station sets `multiline: true` in its `UiConfig`, the input box accepts multiple
lines:

| Key                    | Action                                              |
| ---------------------- | --------------------------------------------------- |
| `Ctrl+Enter`           | Insert a newline (`Enter` still submits)            |
| `Ctrl+J` / `Ctrl+K`    | Move cursor down / up                               |

Lines don't wrap (vim-style `nowrap`): long lines scroll horizontally, and content taller
than the box (`max_height`) scrolls vertically.

> **Note:** `Ctrl+Enter` relies on the [kitty keyboard
> protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/). raddio enables it
> automatically where the terminal supports it (kitty, ghostty, WezTerm, foot, recent
> Alacritty, recent zellij). In terminals that don't, `Ctrl+Enter` can't be told apart from
> `Enter`, so it will submit instead of inserting a newline.

## Configuration

- **Location:** `$XDG_CONFIG_HOME/raddio/config.json` (falls back to
  `~/.config/raddio/config.json`).
- **Format:** [JSON5](https://json5.org/) — regular JSON plus comments, trailing commas,
  and unquoted keys. `raddio init` writes plain pretty-printed JSON, which you can then
  enrich with JSON5 niceties.
- Unknown fields are rejected with a clear parse error (so typos don't get silently ignored).

### Example

```json5
{
  // Global UI defaults. Every field is optional and falls back to a built-in default.
  ui: {
    max_height: 3,
    max_width: 40,
    rounded_corners: true,
    border: true,
    border_color: "#ffffff",
    prefix: " > ",          // you can use a Nerd Font icon, shown before the input (non-editable)
    prefix_color: "#ffffff",
  },

  stations: [
    {
      name: "switch session",
      description: "Switch to a zellij session",
      run: ["zellij", "action", "switch-session", "{}"],
      default: "main",
      // Per-station UI overrides layer on top of the global `ui` block.
      override_ui: { max_width: 30 },
    },
  ],
}
```

### Station fields

| Field         | Type             | Required | Description                                                                 |
| ------------- | ---------------- | -------- | --------------------------------------------------------------------------- |
| `name`        | string           | yes      | The name you pass to `raddio run`.                                          |
| `description` | string           | yes      | Human-friendly summary.                                                     |
| `run`         | array of strings | yes      | Command + args. The element `"{}"` is replaced with your input.            |
| `default`     | string           | no       | Used when you submit empty input.                                           |
| `override_ui` | UiConfig         | no       | UI settings that override the global `ui` block for this station.          |

The `run` array is executed **directly, without a shell** — so there's no quoting or
injection to worry about, but also no pipes/redirects/globs. If you need a shell, make the
command explicit, e.g. `["sh", "-c", "echo {} | some-tool"]`.

### UiConfig fields

All fields are optional. A station's `override_ui` overrides only the fields it sets; the
rest fall back to the global `ui` block, then to these defaults.

| Field             | Type    | Default     | Description                                              |
| ----------------- | ------- | ----------- | -------------------------------------------------------- |
| `max_height`      | integer | `3`         | Max box height (including borders), centered.            |
| `max_width`       | integer | `40`        | Max box width (including borders), centered.             |
| `rounded_corners` | bool    | `true`      | Rounded vs. square border corners.                       |
| `border`          | bool    | `true`      | Whether to draw a border at all.                         |
| `border_color`    | string  | `"#ffffff"` | Hex color for the border.                                |
| `prefix`          | string  | ` ` (icon) | Non-editable decoration shown before the input.          |
| `prefix_color`    | string  | `"#ffffff"` | Hex color for the prefix (falls back to `border_color`). |
| `multiline`       | bool    | `false`     | Allow multiple lines of input (see Key bindings below).  |

## zellij integration

The intended workflow: bind a key to open raddio in a floating pane. It runs your command
and the pane closes automatically on exit.

```sh
zellij run --floating --close-on-exit -- raddio run "switch session"
```

Or as a keybinding in your zellij config (KDL):

```kdl
bind "Alt s" {
    Run "raddio" "run" "switch session" {
        floating true
        close_on_exit true
    }
}
```

## Roadmap / ideas

- Fuzzy-find over a `source` command's output (e.g. fuzzy-match `zellij list-sessions`)
  rather than free-typed input.
- Multi-select / preview.
- Per-station themes beyond the current `UiConfig` knobs.

## License

TODO
