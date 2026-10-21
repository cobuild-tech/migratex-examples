# flyaway (Rust)

The Rust port of [`../java`](../java). See [`../README.md`](../README.md) for how and why.

```bash
FLYAWAY_ADMIN_PASSWORD=… cargo run     # fixed mode, on :8080
FLYAWAY_COMPAT=1 cargo run             # answers exactly as Tomcat did
```

`DATABASE_URL` defaults to `mysql://flyaway:flyaway@127.0.0.1:3327/flyaway`, the MySQL from
`../docker-compose.yml`.

| File | What it is |
|---|---|
| `src/lib.rs` | The two modes and their routers |
| `src/compat.rs` | The three servlets and the JSP pages, line by line, over one set of static fields |
| `src/jsp.rs` | Writes a page from its original `.jsp` file, the way Jasper does |
| `src/java.rs` | The JDK's rules: `null`, `Integer.parseInt`, `LocalDate.parse`, exceptions and their frames |
| `src/tomcat.rs` | Tomcat's: request parameters, static files, error pages |
| `src/db.rs` | The `flight` table |
| `src/fixed.rs` and `templates/` | The default mode, with the bugs fixed |

## Tests

The tests need the MySQL from `../docker-compose.yml`. They only read from it.

```bash
docker compose -f ../docker-compose.yml up -d --wait db
cargo test
```

Without a local Rust toolchain:

```bash
docker run --rm --network java-rust_default -v "$PWD/..":/work -w /work/rust \
  -e DATABASE_URL=mysql://flyaway:flyaway@db:3306/flyaway rust:1.90 cargo test
```

`tests/golden.rs` replays every session in `../java/cases` and compares with `tests/golden`, which
`../java/capture-golden.sh` records from Tomcat. `tests/fixed.rs` has one test per fix.
