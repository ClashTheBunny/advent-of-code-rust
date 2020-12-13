#!/bin/bash

DAY=$1

COMMANDS=("rustup run nightly cargo clippy --all-targets" "cargo test -- --nocapture" "cargo aoc -d $DAY")

cargo aoc input -y 2020 -d "$DAY"

head input/2020/day${DAY}.txt

for COMMAND in "${COMMANDS[@]}"; do
  echo tmux split-window "git ls-files -cdmo --exclude-standard | entr $COMMAND"
  tmux split-window "git ls-files -cdmo --exclude-standard | entr $COMMAND"
done
