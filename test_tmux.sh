#!/bin/bash

YEAR=$1
DAY=$2

COMMANDS=("rustup run nightly cargo clippy --all-targets -- -Z macro-backtrace" "cargo test -- --nocapture")

cargo aoc input -y $YEAR -d "$DAY"

head input/$YEAR/day${DAY}.txt

[[ -f "src/day${DAY}.rs" ]] || sed -e 's/dayX/day'${DAY}'/g' src/dayX.rs > "src/day${DAY}.rs"

for COMMAND in "${COMMANDS[@]}"; do
  tmux split-window "while sleep 1; do git ls-files -cdmo --exclude-standard | entr -d $COMMAND; done"
done

while sleep 1; do git ls-files -cdmo --exclude-standard | entr cargo aoc -d $DAY; done
