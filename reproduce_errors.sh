#!/bin/sh

print_header() {
	sep="$(for _ in $(seq 1 $((${#1} + 4))); do printf '#'; done)"
	printf "\n${sep}\n# %s #\n${sep}\n\n" "$1"
}

print_header "Bad error"
cargo check --features bad_error

print_header "Better error"
cargo check --features better_error

print_header "Good(-ish) error"
cargo check --features good_error
