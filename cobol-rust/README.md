# Migration: COBOL → Rust

The [`rust/`](./rust) program is a complete rewrite of [`cobol/`](./cobol), a menu-driven banking system
written in GnuCOBOL: customer registration and login, current, savings and payroll accounts, deposits,
withdrawals, transfers by IBAN, debit cards, a movement history, and an administrator menu for customers,
employees, settings and statistics. All its data lives in six fixed-length sequential files. The rewrite
reads and writes those files unchanged and prints the same Spanish screens, character for character. This
document describes how that migration was approached.

The original is [Banco-en-COBOL](https://github.com/Mel-Roses/Banco-en-COBOL) by Mel-Roses, used under the
AGPL-3.0 (see [Licence](#licence)). The method is the one in our guide
[COBOL to Rust: a migration guide](https://cobuildx.ai/blog/cobol-to-rust-migration-guide).

## Strategy

The program is one 1,582-line file with 81 paragraphs, so a **full rewrite** was chosen over
running the two side by side step by step. What made it safe is that the old program can still be run:
GnuCOBOL builds it unchanged, so every session was recorded on the original first and the Rust port was
held to those recordings byte for byte, its screens and its data files both.

That raised the main question of the port: the original has bugs, some serious (every account gets the
same IBAN, and logging out closes the program). A port that fixes them can't be compared byte for byte
with the original, and a port that keeps them is no use. So the port has **two modes**:

- `BANCO_COMPAT=1` reproduces the original exactly, bugs included. This is what the golden tests run.
- The default mode fixes the bugs listed below. Each fix is one `if self.fixed()` in the code, with a
  comment that says what the original does, and one test that fails in compat mode.

The port first matched every recording in compat mode. Only then were the fixes added, one at a time.

## Architectural mapping

| COBOL | Rust | Notes |
|---|---|---|
| `FILE SECTION` record layouts | Structs in `src/records.rs` with `decode` / `encode` | Byte for byte: 378, 177, 226, 284, 202 and 309 bytes. Tested by round-trip |
| `PIC X(n)` | `[u8; N]` (`Text<N>`) | Raw bytes, padded with spaces. No `String`, so accented input passes through as typed |
| `PIC 9(n)V9(m)` (unsigned `DISPLAY`) | `u64` count of the smallest unit + a `Pic` describing the field | Cents for `V99`, thousandths for `V999`. Never floating point |
| `MOVE` / `ADD` / `SUBTRACT` / `COMPUTE`, no `ROUNDED`, no `ON SIZE ERROR` | `Pic::store` | Drops the sign, cuts high-order digits, truncates decimals: `ADD 1` to a `PIC 99` holding 99 gives 0 |
| `DISPLAY` of a number | `Pic::show` | `000000000825.50` |
| `ACCEPT` into a number | `Pic::accept` | Copied from libcob 3.2's `cob_move_alphanum_to_display`, see [Numbers](#numbers) |
| `WORKING-STORAGE SECTION` | Fields on the `Banco` struct | Kept as shared state: paragraphs pass results to each other through these fields, and some behaviour depends on it |
| Paragraphs + `PERFORM` | Methods with the paragraph's name | `REGISTRAR-MOVIMIENTO` is `registrar_movimiento` |
| `EVALUATE WS-OPCION WHEN "1"` | `match &self.opcion { b"1 " => … }` | The whole `PIC X(2)` is compared, so `01` is not `1`, as in the original |
| `OPEN EXTEND` + `WRITE` | `Files::append` | |
| `OPEN OUTPUT` + `WRITE` | `Files::replace` | |
| `OPEN I-O` + read until match + `REWRITE` | `Files::rewrite_first` | First match only, as in the original |
| `DISPLAY … NO ADVANCING` / `ACCEPT` | `Term::prompt` / `Term::accept_x`, `accept_n` | |
| `ACCEPT … FROM DATE` / `FROM TIME` | `clock::now()` | Honours GnuCOBOL's `COB_CURRENT_DATE`, so one setting pins both clocks |
| `EXIT PARAGRAPH` | `return Ok(())` | |
| File status 35 on a missing file | Missing files are created | The original needs the six `.dat` files to exist |

## Process followed

1. **Read the whole program** and listed every menu path, record layout, picture clause and odd
   behaviour, including the broken ones (see [What the move found](#what-the-move-found)).
2. **Built the original** with GnuCOBOL 3.2. It needs `-fformat=variable` because its lines run past
   column 72, and empty `.dat` files to start. Its source is unchanged.
3. **Pinned down the runtime.** Three things the program relies on are not in its source: how `ACCEPT`
   turns text into a number, what `DISPLAY` prints, and how arithmetic overflows. They were read from the
   libcob 3.2 source and checked with small probe programs.
4. **Recorded golden sessions.** Ten scripted sessions, some across several runs of the program, cover every
   menu option and every bug found. `cobol/capture-golden.sh` feeds each to the COBOL build with the clock
   pinned and saves the screen output and all six data files after every run.
5. **Ported the record layouts first**, each with a round-trip test, then the number handling, then the
   paragraphs in order, until every recorded session matched in compat mode.
6. **Checked the number parser on its own**: 1,550 inputs (the awkward cases plus random strings of digits,
   spaces, points, commas and signs) go through `ACCEPT` in COBOL and through `Pic::accept` in Rust, for all
   four numeric field types the program reads.
7. **Added the fixes** behind the mode switch, each with a test.
8. **Added a batch job.** The original has no batch work, which is where most COBOL lives, so a small
   `intereses.cob` was written in the same style to post monthly interest from the rates the admin menu
   already stores. It was ported and recorded the same way.

## What the move found

### Fixed in the default mode

- **Every account gets the same IBAN.** `STRING "ES91" WS-IBAN-NUM-EDIT DELIMITED BY SPACE …` uses a
  `PIC Z(10)` field, whose leading zeros print as spaces, so `DELIMITED BY SPACE` stops before the number.
  Every account is `ES910000000000`, and a transfer "by IBAN" always lands in the oldest account in the bank.
  The port builds `ES91` + a 10-digit sequence + `0000000000`.
- **Logging out closes the program.** The customer and admin menus share one `WS-MOSTRAR-MENU` flag with the
  main menu, so "0. Cerrar Sesion" ends all three loops. The port goes back to the main menu.
- **Ids start again on every run.** The counters only ever start from their `VALUE` clause, so after a
  restart the next customer is 1001 again, next to the first 1001. Logging in as 1001 then finds the first
  one, and a deposit into a duplicate account id updates whichever account came first in the file. The port
  carries on from the highest id on file.
- **Received transfers are filed under the sender.** The receiving movement uses `WS-CLIENTE-ACTUAL`, so it
  never appears in the receiver's history and shows twice in the sender's. The port files it under the
  receiving account's owner.
- **Any customer can see, block and re-PIN any card.** "Ver Tarjetas" lists every card in the bank, and
  block, unblock and change PIN take any card id. The port limits all four to the customer's own cards, and
  says "Tarjeta no encontrada" when there is no match (the original says nothing).
- **A PIN can be anything.** The port wants four digits.
- **Deleted customers can still log in.** The login looks up the id without checking the record's state.
- **Every movement is stamped with the time the program started.** `WS-FECHA-ACTUAL` and `WS-HORA-ACTUAL` are
  set once at start-up. The port reads the clock for each movement.
- **End of input loops forever.** GnuCOBOL gives `ACCEPT` a space at end of input, so a piped session that
  doesn't end with `0` prints "Opcion invalida" forever. The port closes the program, in both modes.

### Found and kept, so the screens stay the same

- **`7,25` is read as 725.00.** `ACCEPT` skips commas, which is a thousands separator to COBOL. For a
  Spanish bank, where the comma is the decimal separator, that is a real risk, but changing what amounts mean
  is a business decision, not a port decision.
- **Limits and commission are stored, never applied.** Every account carries a daily limit, a monthly limit
  and a commission, and the admin menu can change the defaults, but no paragraph reads them.
- **The customer login is the customer id**, with no password or PIN. Admin passwords are stored in plain
  text. The admin can delete the last admin account.
- **Card numbers, PINs and CVVs come from the clock.** Every card issued on the same day has the same number.
- **"Copia de Seguridad" and "Restaurar Datos" do nothing** but print that they worked.
- **Deleting a customer leaves their accounts and cards active**, and money in them still counts in the
  statistics.
- **Arithmetic overflow is silent.** A deposit that takes a balance past 999,999,999,999.99 loses the
  leading digits, because no `ON SIZE ERROR` is coded.
- "Datos actualizados" and "Configuracion guardada" print after "0. Volver" too, and the closing message
  still says "Cerrando SantiBanco…".

## Numbers

Every amount is an unsigned fixed-point decimal, and the port never uses floating point. The rules it copies:

- **Stored results drop the sign and cut, never round.** `SUBTRACT 5 FROM 3` into an unsigned field gives 2.
  Interest of 2.5720… is posted as 2.57, and so is 2.5799….
- **`ACCEPT` is generous.** It skips leading spaces, a sign and commas, keeps as many decimals as the field
  has, and turns anything else into 0, unless the field is already full when the bad character comes. So
  `12a` is 12 in a `PIC 9(8)` id but 0 in a `PIC 9(12)V99` amount. The line is cut to the field's size first,
  so typing 13 digits into a 12-digit amount drops the first digit.

`Pic::accept` matches GnuCOBOL on all 1,550 inputs in `rust/tests/golden/accept`, for every field type.

## Result

- Every recorded session matches the COBOL build byte for byte in compat mode, both the screen output and
  all six data files after each run, across 10 sessions and 19 runs of the two programs
- The two programs can share a data folder: running COBOL, then Rust, then COBOL, then Rust on one folder
  gives the same files and screens as the all-COBOL recording
- 16 tests: record round-trips, number handling, the golden sessions, the parser corpus, and one test per
  fix. Clean `cargo clippy -- -D warnings` and `cargo fmt --check`
- One dependency (`chrono`, for the local clock)

## Projects

| Folder | Stack |
|---|---|
| [`cobol/`](./cobol) | GnuCOBOL 3.2, sequential files |
| [`rust/`](./rust) | Rust 2021, `cargo` |

Both run in the folder that holds the data files. `rust/` is a standalone crate with its own README and
tests.

### The COBOL side

`cobol/` is the upstream repository merged in with its history, so `git log cobol-rust/cobol` shows the
original commits and authors (from [`7ea14ed`](https://github.com/Mel-Roses/Banco-en-COBOL/tree/7ea14ed7512e77cae43c0b837884f7bc76beec90)).
Its README is the upstream one. The commit after the import removes the bundled Windows GnuCOBOL runtime
(`*.dll`) and the sample `.dat` files. `banco.cob` itself is unchanged. These files are ours:

| File | What it is |
|---|---|
| `intereses.cob` | A monthly interest batch job in the same style, so the example covers batch work too |
| `tests/accept.cob` | Prints what `ACCEPT` stores for each input, to check the Rust number parser |
| `capture-golden.sh` | Records the golden sessions the Rust tests compare against |
| `Makefile` | Build, run and record |

```bash
cd cobol
brew install gnucobol            # or apt install gnucobol (GnuCOBOL 3.2 was used)
make                             # builds ./banco and ./intereses
make run                         # creates empty data files if needed, starts the menus
make golden                      # re-records ../rust/tests/golden (needs `timeout` from coreutils)
```

The source runs past column 72, so it is compiled with `-fformat=variable`. The program stops with file
status 35 if a data file is missing, so `make run` creates them first.

## Licence

`banco.cob` is © its author and licensed under the GNU Affero General Public License v3.0. The Rust port is a
derivative work and is released under the same licence. See [`LICENSE`](./LICENSE). The rest of this
repository is not affected.
