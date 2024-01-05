#!/usr/bin/env bash

set -e

if ! command -v cargo &>/dev/null; then
	echo "cargo not found - please install Rust!"
	exit
elif ! command -v jq &>/dev/null; then
	echo "please install jq!"
	exit
elif ! command -v samply &>/dev/null; then
	echo "samply is not found. Please install from <https://github.com/mstange/samply>"
	exit
fi

target_dir=$(jq -r '.target_directory' <<<"$(cargo metadata --format-version 1)")
bin_dir="$target_dir/profiling/stdout-vs-stderr-profiler"

export DURATION=5
cargo build --profile profiling --bin stdout-vs-stderr-profiler

sleep 1
export STREAM="stdout"
samply record -s -n -o "$STREAM-profile.json" "$bin_dir"

sleep 1
export STREAM="stderr"
samply record -s -n -o "$STREAM-profile.json" "$bin_dir"

samply load stderr-profile.json &
samply load stdout-profile.json
