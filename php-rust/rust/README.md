# students (Rust)

The Rust port of [`../php`](../php). See [`../README.md`](../README.md) for how and why.

```bash
DATABASE_URL=mysql://root@127.0.0.1:3317/studentdb cargo run        # fixed mode, on :8080
STUDENTS_COMPAT=1 cargo run                                         # answers exactly as PHP did
```

| File | What it is |
|---|---|
| `src/lib.rs` | The router: one route per PHP script, at the same path |
| `src/compat.rs` | The five scripts, line by line |
| `src/php.rs` | PHP's runtime rules: `json_decode`, `isset`, notices, string conversion, `json_encode` |
| `src/db.rs` | `models/Student.php`: the same SQL, and the `latin1` connection |
| `src/fixed.rs` | The default mode, with the bugs fixed |

## Tests

The tests need the MariaDB from `../docker-compose.yml`, and drop and reload `studentdb` as they go.

```bash
docker compose -f ../docker-compose.yml up -d --wait db
cargo test
```

Without a local Rust toolchain:

```bash
docker run --rm --network php-rust_default -v "$PWD/..":/work -w /work/rust \
  -e DATABASE_URL=mysql://root@db:3306/studentdb rust:1.90 cargo test
```

`tests/golden.rs` replays every session in `../php/cases` and compares with `tests/golden`, which
`../php/capture-golden.sh` records from PHP. `tests/fixed.rs` has one test per fix.
