#!/usr/bin/env bash
# Records the golden responses the Rust tests compare against.
#
# Each file in cases/ is one scripted session: one request per line, as
# "METHOD PATH [BODY]". Before each session the database is reset to
# db/studentdb.sql. For every request the status, the headers clients see and
# the body are saved, and after the session the students table is saved too,
# with names and addresses in hex so the stored bytes are compared exactly.
#
# Needs the PHP side running: `docker compose up -d db php` in php-rust/.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$here/.."
out="$root/rust/tests/golden"
base="${PHP_URL:-http://localhost:8180}"

sql() { (cd "$root" && docker compose exec -T db mysql -uroot "$@"); }
db_up() { (cd "$root" && docker compose up -d --wait db >/dev/null 2>&1); }

reset_db() {
  sql -e "DROP DATABASE IF EXISTS studentdb; CREATE DATABASE studentdb;"
  sql studentdb < "$here/db/studentdb.sql"
}

db_up
rm -rf "$out"
for case_file in "$here"/cases/*.txt; do
  name="$(basename "$case_file" .txt)"
  dir="$out/$name"
  mkdir -p "$dir"
  reset_db
  n=0
  while IFS= read -r line || [ -n "$line" ]; do
    n=$((n + 1))
    step="$(printf '%02d' "$n")"
    method="${line%% *}"
    rest="${line#* }"
    path="${rest%% *}"
    args=(-s -X "$method" -D "$dir/$step.raw-headers" -o "$dir/$step.body")
    if [ "$rest" != "$path" ]; then
      args+=(--data-binary "${rest#* }")
    fi
    curl "${args[@]}" "$base$path"
    # Keep the status and the headers a client relies on, not Date or Server.
    # Header names are case-insensitive, so they are saved in lower case.
    tr -d '\r' < "$dir/$step.raw-headers" | awk '
      NR == 1 { print $2; next }
      { split($0, kv, ":"); name = tolower(kv[1]) }
      name ~ /^(content-type|access-control-[a-z-]+)$/ { print name substr($0, length(kv[1]) + 1) }
    ' | { read -r status; echo "$status" > "$dir/$step.status"; sort > "$dir/$step.headers"; }
    rm "$dir/$step.raw-headers"
  done < "$case_file"
  sql -B studentdb -e \
    "SELECT id, HEX(name), HEX(address), age FROM students ORDER BY id" > "$dir/students.tsv"
  echo "$name: $n requests"
done

# With the database stopped, every script hits the bug in config/Database.php.
down="$out/database-down"
mkdir -p "$down"
(cd "$root" && docker compose stop db >/dev/null 2>&1)
for pair in get.php:GET get_one.php:GET post.php:POST put.php:PUT delete.php:DELETE; do
  script="${pair%%:*}"
  curl -s -X "${pair#*:}" --data-binary '{}' -o "$down/$script.body" "$base/api/$script"
done
db_up
echo "database-down: 5 requests"
