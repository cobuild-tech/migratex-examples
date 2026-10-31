#!/usr/bin/env bash
# Records the golden responses the Rust tests compare against.
#
# Each file in cases/ is one scripted session: one request per line, as
# "METHOD PATH [BODY]", where BODY is sent as a form post. FlyAway keeps its
# state in static fields, so before each session Tomcat is restarted to clear
# them. The app never writes to the database, so the database is left alone.
# For every request the status, the headers clients see and the body are saved.
#
# Needs the Java side running: `docker compose up -d db java` in java-rust/.
# JAVA_URL=http://localhost:8281 records from the Rust port instead, and
# JAVA_URL=http://localhost:8200 through nginx; SERVICE names the containers
# to restart between sessions ("rust", or "java rust" for nginx).
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$here/.."
out="${GOLDEN_DIR:-$root/rust/tests/golden}"
base="${JAVA_URL:-http://localhost:8280}"
service="${SERVICE:-java}"
[ "$base" = "http://localhost:8280" ] || service="${SERVICE:-rust}"

restart() {
  # shellcheck disable=SC2086 # SERVICE may name more than one container.
  (cd "$root" && docker compose restart $service >/dev/null 2>&1)
  # Wait for both sides without touching any state: OPTIONS on a servlet
  # only lists its methods.
  until curl -sf -o /dev/null "$base/index.jsp" &&
    curl -sf -o /dev/null -X OPTIONS "$base/login"; do sleep 1; done
}

rm -rf "$out"
for case_file in "$here"/cases/*.txt; do
  name="$(basename "$case_file" .txt)"
  dir="$out/$name"
  mkdir -p "$dir"
  restart
  n=0
  jar="$(mktemp)"
  while IFS= read -r line || [ -n "$line" ]; do
    n=$((n + 1))
    step="$(printf '%02d' "$n")"
    method="${line%% *}"
    rest="${line#* }"
    path="${rest%% *}"
    args=(-s -X "$method" -b "$jar" -c "$jar" -D "$dir/$step.raw-headers" -o "$dir/$step.body")
    if [ "$rest" != "$path" ]; then
      args+=(--data-binary "${rest#* }" -H 'Content-Type: application/x-www-form-urlencoded')
    fi
    curl "${args[@]}" "$base$path"
    # Keep the status and the headers a client relies on, not Date, the
    # session cookie or the framing. Names are saved in lower case.
    tr -d '\r' < "$dir/$step.raw-headers" | awk '
      NR == 1 { print $2; next }
      { split($0, kv, ":"); name = tolower(kv[1]) }
      name ~ /^(content-type|content-language|location|allow)$/ { print name substr($0, length(kv[1]) + 1) }
    ' | { read -r status; echo "$status" > "$dir/$step.status"; sort > "$dir/$step.headers"; }
    rm "$dir/$step.raw-headers"
  done < "$case_file"
  rm -f "$jar"
  echo "$name: $n requests"
done
