# tmgitty
My personal git plugin for TMUX

Adds git status information to the right hand side of the status bar

To install ensure rust is available on the system as the install script will run `cargo build`

Install rust with the command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Then install using TPM by adding the below line to your tmux.conf
`set -g @plugin 'JKDow/tmgitty'`

