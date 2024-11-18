#!/usr/bin/env bash

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

$CURRENT_DIR/scripts/install.sh

set -g @tmgitty_fetch "$CURRENT_DIR/scripts/fetch.sh"
set -g @tmgitty_status "$CURRENT_DIR/scripts/status.sh"

set -g status-right "#($CURRENT_DIR/scripts/status.sh)"
