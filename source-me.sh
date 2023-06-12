#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

alias new="$SCRIPT_DIR/new"
alias g++="g++ -fsanitize=address,undefined -Wall"
