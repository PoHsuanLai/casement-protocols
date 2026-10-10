#!/usr/bin/env bash
# The crate's boundary, mechanically enforced (ARCHITECTURE.md section 1 is the table; this is
# its mechanical form).
#
# `cargo tree -i <dep>` exits 101 when the dependency is absent, which is precisely the state
# we want. Checking the exit status would therefore fail whenever the boundary holds, so we
# check for OUTPUT instead: any line naming the dependency is a leak.
set -uo pipefail
cd "$(dirname "$0")/.."
crate=casement-protocols
fail=0

# A crate that cargo cannot find would make every check below pass vacuously.
if ! cargo tree -p "$crate" --depth 0 >/dev/null 2>&1; then
  echo "ERROR: cargo tree cannot resolve $crate; the boundary was not checked"
  exit 1
fi

# 1. Default features: the vocab is pure. It reaches no Wayland crate, event loop, compositor
# toolkit, bus or runtime, through any path.
FORBIDDEN="wayland-client wayland-server wayland-backend wayland-scanner wayland-protocols wayland-sys calloop smithay zbus tokio"
leaked=0
for dep in $FORBIDDEN; do
  if cargo tree -p "$crate" -i "$dep" -e normal,build 2>/dev/null | grep -q .; then
    echo "LEAK: $crate (default features) reaches $dep"
    cargo tree -p "$crate" -i "$dep" -e normal,build 2>/dev/null | head -20
    leaked=1
    fail=1
  fi
done
[ "$leaked" -eq 0 ] && echo "boundary holds: $crate (default features) reaches none of $FORBIDDEN"

# 2. Direct dependencies, exactly: serde and thiserror by default; with every feature, the
# wayland crates the bindings need and bitflags (named by wayland-scanner's bitfield code).
# A dependency not listed is a leak; so is one the crate no longer has.
check_direct() {
  local label="$1" flags="$2" want="$3" found
  # shellcheck disable=SC2086
  found=$(cargo tree -p "$crate" --depth 1 -e normal,build --prefix none $flags 2>/dev/null \
    | awk 'NR > 1 {print $1}' | sort -u | tr '\n' ' ')
  want=$(printf '%s\n' $want | sort -u | tr '\n' ' ')
  if [ "$found" != "$want" ]; then
    echo "EDGE: $crate ($label) depends on [${found% }], the table allows [${want% }]"
    fail=1
  else
    echo "edges hold: $crate ($label) depends on [${found% }]"
  fi
}
check_direct "default features" "" "serde thiserror"
check_direct "all features" "--all-features" \
  "serde thiserror wayland-scanner wayland-backend wayland-client wayland-server wayland-protocols bitflags"

# 3. The vocab names no Wayland crate, and nothing outside the generated bindings names one.
if grep -rnE '\bwayland_(client|server|backend|scanner|protocols)\b' src/vocab src/lib.rs; then
  echo "LEAK: the vocab or lib.rs names a wayland crate"
  fail=1
else
  echo "boundary holds: src/vocab and src/lib.rs name no wayland crate"
fi

# 4. No unsafe in our own source (the generated bindings' unsafe comes from wayland-scanner).
if grep -rnE '\bunsafe\b' src; then
  echo "LEAK: unsafe in src/"
  fail=1
else
  echo "boundary holds: no unsafe in src/"
fi

# 5. Clean room: every interface is ours (quire_*), and no XML defines or names a COSMIC
# interface.
if grep -nE '<interface name="' protocols/*.xml | grep -vE 'name="quire_[a-z0-9_]+_v1"'; then
  echo "LEAK: an interface outside quire_*_v1"
  fail=1
elif grep -nE 'zcosmic|cosmic_' protocols/*.xml; then
  echo "LEAK: a protocol names a COSMIC interface"
  fail=1
else
  echo "boundary holds: every interface is quire_*_v1 and none is COSMIC's"
fi

# 6. desktop-client: its core reaches no Wayland crate and no I/O, and names none in source;
# the wire layer is the only place wayland_* is named in code (comments may mention it).
dc=desktop-client
leaked=0
for dep in $FORBIDDEN rustix; do
  if cargo tree -p "$dc" -i "$dep" -e normal,build 2>/dev/null | grep -q .; then
    echo "LEAK: $dc (default features) reaches $dep"
    leaked=1
    fail=1
  fi
done
[ "$leaked" -eq 0 ] && echo "boundary holds: $dc (default features) reaches no wayland crate"
if grep -rnE '\bwayland_(client|server|backend|scanner|protocols|protocols_wlr)\b' desktop-client/src \
  | grep -vE '^desktop-client/src/client/' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//'; then
  echo "LEAK: desktop-client names a wayland crate outside src/client/"
  fail=1
else
  echo "boundary holds: desktop-client names a wayland crate only in src/client/"
fi
if grep -rnE '\bunsafe\b' desktop-client/src; then
  echo "LEAK: unsafe in desktop-client/src"
  fail=1
fi

exit "$fail"
