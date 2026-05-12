# Alfred Script Filter — Keyman keyboards

`keyman-keyboards.py` is a self-contained
[uv](https://docs.astral.sh/uv/) script that emits
[Alfred Script Filter JSON](https://www.alfredapp.com/help/workflows/inputs/script-filter/json/),
one item per active Keyman keyboard.

Each item:

| Field | Value |
|---|---|
| `title` | Keyboard display name (e.g. `EuroLatin (SIL)`) |
| `subtitle` | `[SELECTED]` for the currently-selected keyboard; otherwise the single language name from `kmp.json` if exactly one is listed, or empty |
| `arg` | Canonical keyboard id (e.g. `/qpolish/qpolish.kmx`) — pipe straight into `keyman select` |
| `match` | Combined `name + stem + package + language` for Alfred fuzzy match |
| `icon.path` | `<stem>.256x256.png` if present, else `<stem>.png`, else omitted |
| `uid` | Canonical id (Alfred uses this to remember the user's frequency-of-use ordering) |
| `valid` | `true` |

Icons are looked up inside each keyboard's package directory at
`~/Library/Application Support/keyman.inputmethod.Keyman/Keyman-Keyboards/<package>/`.

## Requirements

- `uv` (`brew install uv` or [the official installer](https://docs.astral.sh/uv/getting-started/installation/))
- The `keyman` CLI on `PATH`, **or** the env var `KEYMAN_CLI` pointing
  at it, **or** the script run from within the `mac/keyman-cli/`
  source tree with a `target/release/keyman` build present.

A simple way to keep the CLI fresh:

```bash
ln -sf "$(pwd)/target/release/keyman" ~/.local/bin/keyman
```

That way every `cargo build --release` is picked up without touching
the symlink.

## Wiring it into an Alfred workflow

1. Open Alfred → Workflows → `+` → Blank Workflow.
2. Right-click the canvas → **Inputs → Script Filter**.
3. Configure:
   - **Keyword**: e.g. `kb` (or whatever you want to type to trigger it).
   - **Language**: `/bin/bash`.
   - **Script**: `/absolute/path/to/keyman-keyboards.py "$1"`
     (Alfred passes the query as `$1`; we currently let Alfred do the
     fuzzy match via the `match` field, but the argument is accepted for
     future use.)
   - **With Input as**: argv (the default).
   - Tick **Run Behaviour → Always run immediately**.
4. Right-click the canvas → **Actions → Run Script** (or **Run NSAppleScript**).
   Set its language to `/bin/bash` and command to:
   ```bash
   /Users/boaz.yaniv/.local/bin/keyman select "$1"
   ```
   (Or whichever path your `keyman` binary lives at.)
5. Connect the Script Filter's output to the Run Script's input.

When you type `kb<space>...`, Alfred will show the keyboards, and
pressing Return on one will run `keyman select <canonical-id>` —
which, against a Keyman build with the matching `keyman:select` URL
patch, switches keyboards live on the next keystroke.

## Performance

Warm runs land under ~60 ms on Apple Silicon (uv venv cache hit plus
one `keyman list --json` subprocess). That's well inside Alfred's
sub-100 ms ideal.

## Customisation hooks

If you want the script to show something other than `[SELECTED]` /
language in the subtitle, the rendering logic lives in
`keyboard_to_item()` — a one-function edit.
