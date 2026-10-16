# Banco (COBOL) — migration notes

This folder holds every file from [Banco-en-COBOL](https://github.com/Mel-Roses/Banco-en-COBOL) by Mel-Roses (the source credits XalVok), copied unchanged
from commit [`7ea14ed`](https://github.com/Mel-Roses/Banco-en-COBOL/tree/7ea14ed7512e77cae43c0b837884f7bc76beec90).
That includes `banco.cob`, the upstream `README*.md` files and `LICENSE` (AGPL-3.0), the six starting `.dat`
files, and the Windows GnuCOBOL runtime DLLs the upstream ships for running on Windows. `README.md` is the
upstream README; these notes cover the files we added.

`make run` writes to the `.dat` files, so run `git checkout -- '*.dat'` afterwards to get back to the starting data.

The other files are ours:

| File | What it is |
|---|---|
| `intereses.cob` | A monthly interest batch job in the same style, so the example covers batch work too |
| `tests/accept.cob` | Prints what `ACCEPT` stores for each input, to check the Rust number parser |
| `capture-golden.sh` | Records the golden sessions the Rust tests compare against |
| `Makefile` | Build, run and record |

```bash
brew install gnucobol            # or apt install gnucobol (GnuCOBOL 3.2 was used)
make                             # builds ./banco and ./intereses
make run                         # creates empty data files if needed, starts the menus
make golden                      # re-records ../rust/tests/golden (needs `timeout` from coreutils)
```

The source runs past column 72, so it is compiled with `-fformat=variable`. The program stops with file
status 35 if a data file is missing, so `make run` creates them first.
