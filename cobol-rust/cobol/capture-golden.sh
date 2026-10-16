#!/usr/bin/env bash
# Runs every golden session against the COBOL programs and saves what they
# print and the data files they leave behind. The Rust tests compare their own
# runs against these files byte for byte.
#
#   make golden          (builds the programs first)
#
# Each case is a folder under ../rust/tests/golden/cases/ with numbered steps:
#   01-banco.in        typed into ./banco, one answer per line
#   02-intereses.in    runs ./intereses (input ignored)
# Steps in a case share one data folder, so a later step sees what an earlier
# one wrote, exactly like restarting the program.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
golden="$here/../rust/tests/golden"
data_files=(clientes.dat cuentas.dat tarjetas.dat movimientos.dat config.dat empleados.dat)

# A fixed clock, so dates, times, PINs and CVVs come out the same on every run.
export COB_CURRENT_DATE="2026/10/16 09:30:15.420000000"

for case_dir in "$golden"/cases/*/; do
  case_name="$(basename "$case_dir")"
  work="$(mktemp -d)"
  for f in "${data_files[@]}"; do : > "$work/$f"; done

  rm -rf "$case_dir/expected"
  for step in "$case_dir"/*.in; do
    step_name="$(basename "$step" .in)"
    program="${step_name#*-}"
    out="$case_dir/expected/$step_name"
    mkdir -p "$out"
    (cd "$work" && timeout 20 "$here/$program" < "$step" > "$out/stdout.txt" 2>&1) || { echo "$case_name/$step_name did not finish: check the session ends with 0"; exit 1; }
    for f in "${data_files[@]}"; do cp "$work/$f" "$out/$f"; done
  done
  rm -rf "$work"
  echo "captured $case_name"
done

# Number parsing: every input line goes through ACCEPT for each numeric
# field type banco.cob reads.
corpus="$golden/accept/input.txt"
{
  wc -l < "$corpus" | tr -d ' '
  while IFS= read -r line; do
    for _ in 1 2 3 4; do printf '%s\n' "$line"; done
  done < "$corpus"
} | "$here/tests/accept" > "$golden/accept/expected.txt"
echo "captured accept corpus ($(wc -l < "$corpus" | tr -d ' ') inputs)"
