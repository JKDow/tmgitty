#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

BINARY_PATH="$SCRIPT_DIR/../target/release/tmgitty"

REPO_PATH=$(tmux display-message -p "#{pane_current_path}")

"$BINARY_PATH" status -r "$REPO_PATH"
