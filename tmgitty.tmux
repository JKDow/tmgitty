#!/usr/bin/env bash

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

$CURRENT_DIR/scripts/install.sh

tmux set-option -g @tmgitty_fetch "$CURRENT_DIR/scripts/fetch.sh"
tmux set-option -g @tmgitty_status "$CURRENT_DIR/scripts/status.sh"

tmux set-option -g status-right "#($CURRENT_DIR/scripts/status.sh) | %H:%M %d-%b-%y "

# tmux set-hook -g pane-focus-in 'run-shell "$CURRENT_DIR/scripts/fetch.sh"'
