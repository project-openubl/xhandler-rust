#!/usr/bin/env bash
# Sets the version of the workspace crates (main says 0.0.0-dev; releases set the tag's).
# perl rather than sed: the same command works on the Linux, macOS and Windows runners.
set -euo pipefail
version="$1"
cd "$(dirname "$0")/.."
# [workspace.package]'s version, and the version of the xhandler dependency of xhandler-cli
perl -0pi -e "s/(\[workspace\.package\]\r?\nversion = )\"[^\"]*\"/\$1\"$version\"/; s/(xhandler = \{ path = \"xhandler\", version = )\"[^\"]*\"/\$1\"$version\"/" Cargo.toml
grep -q "^version = \"$version\"" Cargo.toml
grep -q "xhandler = { path = \"xhandler\", version = \"$version\"" Cargo.toml
# Cargo.lock records the workspace versions too
cargo update --workspace --quiet
