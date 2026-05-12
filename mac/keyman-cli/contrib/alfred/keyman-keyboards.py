#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///

"""Alfred script filter that lists Keyman keyboards for live selection.

Drives `keyman list --json` and emits Alfred's script-filter JSON. The
selected item's `arg` is the canonical keyboard id (`/<pkg>/<file>.kmx`),
suitable for piping straight into a follow-up `keyman select` action.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

KEYBOARDS_ROOT = (
    Path.home()
    / "Library/Application Support/keyman.inputmethod.Keyman/Keyman-Keyboards"
)

ICON_SUFFIXES = (".256x256.png", ".png")


def find_keyman_cli() -> str:
    """Resolve the `keyman` binary path.

    Alfred runs script filters with a deliberately minimal PATH, so we
    fall back to common install locations rather than relying on the
    user's interactive shell environment.
    """
    env_override = os.environ.get("KEYMAN_CLI")
    if env_override and Path(env_override).is_file():
        return env_override

    if (which := shutil.which("keyman")):
        return which

    script_dir = Path(__file__).resolve().parent
    candidates = [
        script_dir.parent.parent / "target" / "release" / "keyman",
        Path.home() / ".cargo" / "bin" / "keyman",
        Path.home() / "bin" / "keyman",
        Path("/usr/local/bin/keyman"),
        Path("/opt/homebrew/bin/keyman"),
    ]
    for candidate in candidates:
        if candidate.is_file():
            return str(candidate)
    raise FileNotFoundError(
        "keyman CLI not found. Set $KEYMAN_CLI or put `keyman` on PATH."
    )


def fetch_keyboards(cli: str) -> list[dict]:
    proc = subprocess.run(
        [cli, "list", "--json"],
        check=True,
        capture_output=True,
        text=True,
    )
    payload = json.loads(proc.stdout)
    return payload.get("keyboards", [])


def stem_for(canonical_id: str) -> str:
    name = canonical_id.rsplit("/", 1)[-1]
    return name[:-4] if name.endswith(".kmx") else name


def find_icon(stem: str, package: str) -> str | None:
    pkg_dir = KEYBOARDS_ROOT / package
    for suffix in ICON_SUFFIXES:
        candidate = pkg_dir / f"{stem}{suffix}"
        if candidate.is_file():
            return str(candidate)
    return None


def keyboard_to_item(kb: dict) -> dict:
    canonical_id = kb["id"]
    name = kb["name"]
    package = kb["package"]
    selected = bool(kb.get("selected", False))
    language = kb.get("language")

    stem = stem_for(canonical_id)

    if selected:
        subtitle = "[SELECTED]"
    elif language:
        subtitle = language
    else:
        subtitle = ""

    item: dict = {
        "uid": canonical_id,
        "title": name,
        "subtitle": subtitle,
        "arg": canonical_id,
        "match": " ".join(filter(None, [name, stem, package, language])),
        "valid": True,
    }

    if (icon_path := find_icon(stem, package)) is not None:
        item["icon"] = {"path": icon_path}

    return item


def error_response(message: str) -> dict:
    return {
        "items": [
            {
                "uid": "keyman-cli-error",
                "title": "Keyman CLI error",
                "subtitle": message,
                "valid": False,
            }
        ]
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "query",
        nargs="?",
        default="",
        help="Alfred query (forwarded for Alfred's own fuzzy match)",
    )
    parser.parse_args()

    try:
        cli = find_keyman_cli()
        keyboards = fetch_keyboards(cli)
    except FileNotFoundError as exc:
        json.dump(error_response(str(exc)), sys.stdout, ensure_ascii=False)
        return
    except subprocess.CalledProcessError as exc:
        stderr = (exc.stderr or "").strip() or "non-zero exit from `keyman list --json`"
        json.dump(error_response(stderr), sys.stdout, ensure_ascii=False)
        return
    except json.JSONDecodeError as exc:
        json.dump(error_response(f"unparseable JSON from keyman CLI: {exc}"), sys.stdout, ensure_ascii=False)
        return

    items = [keyboard_to_item(kb) for kb in keyboards]
    json.dump({"items": items}, sys.stdout, ensure_ascii=False)


if __name__ == "__main__":
    main()
