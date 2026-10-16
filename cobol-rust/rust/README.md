# Banco (Rust)

Rust port of [`../cobol`](../cobol). It reads and writes the same six `.dat` files and prints the same
screens.

```bash
cargo run --bin banco            # the menus, in the current folder's data files
cargo run --bin intereses        # post a month's interest
BANCO_COMPAT=1 cargo run --bin banco   # the original's behaviour, bugs included
cargo test                       # unit tests, golden sessions, parser corpus, fixes
```

The default admin login, created on first run, is `admin` / `admin123`, as in the original.

## COBOL → Rust map

| COBOL | Rust |
|---|---|
| `FILE SECTION` (`REGISTRO-CLIENTE` … `REGISTRO-EMPLEADO`) | `src/records.rs` |
| `PIC 9` storage, `DISPLAY` and `ACCEPT` rules | `src/pic.rs` |
| `OPEN` / `READ` / `WRITE` / `REWRITE` loops | `src/files.rs` |
| `DISPLAY`, `ACCEPT` | `src/term.rs` |
| `ACCEPT … FROM DATE` / `TIME` | `src/clock.rs` |
| `WORKING-STORAGE`, `MAIN`, start-up, main menu | `src/banco/mod.rs` |
| Registration, login, customer menu, "Modificar Datos" | `src/banco/clientes.rs` |
| Accounts, deposits, withdrawals, transfers, history | `src/banco/cuentas.rs` |
| Cards | `src/banco/tarjetas.rs` |
| Admin menu | `src/banco/admin.rs` |
| `intereses.cob` | `src/intereses.rs`, `src/bin/intereses.rs` |

## Tests

- `tests/golden.rs` replays every session in `tests/golden/cases` in compat mode and compares the screen
  output and the six data files with what the COBOL build produced, after every run. It also checks the
  number parser against `tests/golden/accept`.
- `tests/fixed.rs` has one test per bug the default mode fixes. Each one fails in compat mode.

The recordings are made by `../cobol/capture-golden.sh` (`make golden` in `../cobol`). To add a session,
add a numbered `NN-banco.in` or `NN-intereses.in` file to a case folder, re-record, and run `cargo test`.
A session must end at the main menu with `0`, or the COBOL program never stops.
