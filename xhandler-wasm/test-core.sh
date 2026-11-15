#!/usr/bin/env bash
# Tests core.sh against a release made from dist/ the way .github/workflows/release.yml makes it
# (xhandler-dist-<version>.tar.gz, xhandler.wasm, SHA256SUMS), served on localhost.
#
#   test-core.sh <dist dir>
set -euo pipefail
dist=$(realpath "$1")
core=$(realpath "$(dirname "$0")/core.sh")
version=9.9.9-rc.1

work=$(mktemp -d)
cleanup() {
  [ -n "${server:-}" ] && kill "$server" 2>/dev/null
  rm -rf "$work"
}
trap cleanup EXIT

release="$work/releases/v$version"
mkdir -p "$release"
tar -czf "$release/xhandler-dist-$version.tar.gz" -C "$dist" .
cp "$dist/xhandler.wasm" "$release/"
(cd "$release" && sha256sum * >SHA256SUMS)

port=$(python3 -c 'import socket; s = socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1])')
python3 -m http.server "$port" --bind 127.0.0.1 -d "$work/releases" >/dev/null 2>&1 &
server=$!
export XHANDLER_RUST_RELEASES="http://127.0.0.1:$port"
for _ in $(seq 50); do curl -sf "$XHANDLER_RUST_RELEASES/v$version/SHA256SUMS" >/dev/null && break; sleep 0.1; done

failures=0
check() {
  local name=$1 expected=$2
  shift 2
  if "$@" >"$work/out" 2>&1; then actual=pass; else actual=fail; fi
  if [ "$actual" = "$expected" ]; then
    echo "ok   $name"
  else
    echo "FAIL $name: expected $expected, got $actual"
    sed 's/^/     /' "$work/out"
    failures=$((failures + 1))
  fi
}

check "download a release" pass "$core" download "$version" "$work/synced"
check "the download is the dist" pass diff -r "$dist" "$work/synced"
check "download a release with a leading v" pass "$core" download "v$version" "$work/synced"
echo "$version" >"$work/VERSION"
check "verify the released wasm" pass "$core" verify "$work/synced/xhandler.wasm" "$work/VERSION"
check "verify-release the released wasm" pass "$core" verify-release "v$version" "$work/synced/xhandler.wasm" "$work/VERSION"
printf '%s\r\n' "$version" >"$work/VERSION.crlf"
check "verify-release with a CRLF VERSION" pass "$core" verify-release "v$version" "$work/synced/xhandler.wasm" "$work/VERSION.crlf"
check "verify-release another tag" fail "$core" verify-release v9.9.8 "$work/synced/xhandler.wasm" "$work/VERSION"

echo 9.9.8 >"$work/VERSION.unreleased"
check "verify an unreleased core (warns)" pass "$core" verify "$work/synced/xhandler.wasm" "$work/VERSION.unreleased"
check "download an unreleased core" fail "$core" download 9.9.8 "$work/other"
check "download a version outside the scheme" fail "$core" download 9.9.9-next.1 "$work/other"

cp "$work/synced/xhandler.wasm" "$work/tampered.wasm"
echo x >>"$work/tampered.wasm"
check "verify a tampered wasm" fail "$core" verify "$work/tampered.wasm" "$work/VERSION"

echo x >>"$release/xhandler-dist-$version.tar.gz"
check "download a tampered dist" fail "$core" download "$version" "$work/other"

check "verify with the releases unreachable" fail env XHANDLER_RUST_RELEASES=http://127.0.0.1:1 "$core" verify "$work/synced/xhandler.wasm" "$work/VERSION"
check "usage without arguments" fail "$core" verify-release

[ "$failures" -eq 0 ] || {
  echo "$failures core.sh checks failed"
  exit 1
}
