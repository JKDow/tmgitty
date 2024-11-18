#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

BINARY_PATH="$SCRIPT_DIR/../target/release/tmgitty"

REPO_PATH=$(tmux display-message -p "#{pane_current_path}")

FETCH_OUTPUT=$($BINARY_PATH" fetch -r "$REPO_PATH)

exit 0
