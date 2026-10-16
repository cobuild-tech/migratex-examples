# Migration: PHP → Rust

The [`rust/`](./rust) service is a rewrite of [`php/`](./php), a REST API in plain PHP with PDO and MySQL, with no
framework. It has five endpoints for a `students` table: list, get one, add, update and delete. The rewrite
serves the same five URLs from the same database. In compat mode it sends the same bytes back as PHP, notices
and stack traces included. This document describes how that migration was approached.

The original is [php_rest_api_without_framework](https://github.com/hanoak/php_rest_api_without_framework) by
Hanoak, used under the MIT licence (see [Licence](#licence)). The method is the one in our guide
[PHP to Rust: a migration guide](https://cobuildx.ai/blog/php-to-rust-migration-guide).

## Strategy

The guide moves one route at a time behind a proxy, and this example is set up that way:
[`nginx.conf`](./nginx.conf) sends each script's URL to either PHP or Rust, and both use one MariaDB. The two
read endpoints point at Rust, and the three writes at PHP. Moving a route, or moving it back, is one
`proxy_pass` line and a reload.

That only works if a client can't tell which side answered. So, as in our COBOL example, the port has **two
modes**:

- `STUDENTS_COMPAT=1` answers exactly as PHP did: always HTTP 200, the same headers, and the same body bytes,
  including PHP's HTML notices and stack traces. This is the mode to run while both sides serve traffic, and
  the mode the golden tests run.
- The default mode fixes the bugs listed below. Each fix is in `src/fixed.rs` with a comment saying what PHP
  did, and has a test in `tests/fixed.rs`.

The port matched every recording in compat mode first. The fixed mode was written after that.

## Architectural mapping

| PHP | Rust | Notes |
|---|---|---|
| `api/get.php` … `api/delete.php` | One axum route per script, at the same path | `any()` routes, because each script checks the method itself |
| `$_SERVER['REQUEST_METHOD'] === 'GET'` | `method != Method::GET` | |
| `json_decode(file_get_contents("php://input"))` | `php::json_decode` → `Option<Value>` | `null` for an empty or invalid body, as in PHP |
| `isset($data->id)` | `php::isset` | Present and not `null`. `0`, `""` and `false` count as set |
| `$data->name` | `Output::prop` | A missing key prints PHP's notice and gives `null` |
| `$stmt->bindParam(…, $value)` | `Output::bind` | PHP's string conversion: `true` → `"1"`, `15.9` → `"15.9"`, `[…]` → `"Array"` and a notice, an object → a fatal error |
| `models/Student.php` queries | `db.rs`, the same SQL with sqlx | Values bound as strings, so MariaDB converts them the way it did for PDO |
| `new PDO("mysql:host=…")` (no charset) | `MySqlConnectOptions::charset("latin1")` | See [Text in the database](#text-in-the-database) |
| `echo json_encode([...])` | `php::json_encode_object` | `\/`, `\uXXXX` for non-ASCII, nothing at all for invalid UTF-8 |
| PDO returning strings | `"id":"15"`, `"age":"23"` | Integers are formatted as strings in compat mode |
| `display_errors = On`, `html_errors = On` | `Output::notice`, `uncaught_pdo_exception`, … | Same HTML, same file paths and line numbers |
| `header('Access-Control-…')` | Set per route | `Allow-Methods: POST` on post.php, put.php *and* delete.php |

The queries use `sqlx::query` and are checked when they run, not with the `query!` macros, so the crate builds
without a database. The guide shows the macro form, which a larger project would want.

## Process followed

1. **Read all 341 lines** and listed what the API does without saying so: the always-200 replies, a GET that
   reads a JSON body, numbers as strings, the `$e` typo in `Database.php`, and the loose input handling (see
   [What the move found](#what-the-move-found)).
2. **Ran the original as it was run.** The SQL dump says phpMyAdmin with PHP 7.3.19 and MariaDB 10.4, which is
   XAMPP 7.3.19. [`docker-compose.yml`](./docker-compose.yml) runs PHP 7.3 under Apache with XAMPP's error
   settings and MariaDB 10.4, with the PHP code mounted unchanged. The PHP version matters: PHP 8.1 returns
   integers from MySQL as integers, so the JSON would differ.
3. **Recorded golden sessions.** 28 scripted sessions in `php/cases/` (79 requests) cover every endpoint, every
   method, every message, odd ids, loose types, arrays and objects, escaping, missing fields and database
   errors. `php/capture-golden.sh` resets the database before each session and saves the status, the headers
   clients see, the body, and the table, with text in hex, after each session. Five more responses are
   recorded with the database stopped.
4. **Ported each script as a function** that follows it line by line (`src/compat.rs`), with PHP's runtime
   rules in `src/php.rs`. The golden test (`tests/golden.rs`) replays every session against the router and
   compares everything byte for byte.
5. **Checked it from outside**, with the real server: `capture-golden.sh` pointed at the Rust service, and
   then at nginx with two routes on Rust, records files identical to PHP's. Recording PHP twice also gives
   identical files, so the recordings are deterministic.
6. **Added the fixes** in `src/fixed.rs`, each with a test.

## What the move found

### Fixed in the default mode

- **Every reply is HTTP 200.** A wrong method, a missing id, an unknown id and a PHP fatal error all come back
  as 200. The port uses 405 (with `Allow`), 400, 404, 422, 500 and 503.
- **Errors are HTML stack traces.** A missing field gives PHP notices, then an uncaught `PDOException` with
  file paths and the SQL error, as HTML in an `application/json` response. The port answers
  `{"message": "Error: age is missing!"}` with 422.
- **A database outage is a fatal error from a typo.** `Database.php` catches `$exp` but reads `$e`, so instead
  of an error message it prints a notice and dies calling `getMessage()` on `null`. The port answers 503.
- **Anything is accepted as a value.** `"16abc"` is id 16, so `{"id":"16abc"}` deletes student 16. `true` is
  stored as `1`, `19.5` as 20, and `["a"]` as the name `Array`. The port wants a positive whole number or a
  string of digits for an id, strings of 1 to 100 characters for name and address, and a whole number for age.
- **Update and delete never fail.** Both report success for an id that matches nothing. The port returns 404.
- **An empty list is not a list.** With no rows, `get.php` returns `{"message":"No records found!"}`, so
  clients have to handle two shapes. The port returns `[]`.
- **Numbers are strings.** PDO returns every column as a string, so PHP sent `"id":"15","age":"23"`. The port
  sends numbers.
- **The add endpoint doesn't say what it added.** The port returns 201 with the new id.
- **`Access-Control-Allow-Methods: POST` on put.php and delete.php**, copied from post.php. The port names
  each endpoint's own method.

These change what clients see, so they belong in an announced release, after the move. During the move, run
compat mode.

### Text in the database

PHP connects without a charset, so the connection is `latin1`, but the columns are `utf8`. MariaDB converts
each byte PHP sends as if it were a latin1 character, so `é` (2 bytes) is stored as `Ã©` (2 characters, 4
bytes). Reading it back reverses the conversion, so PHP never noticed. One side effect: a 4-byte emoji fits in
the 3-byte `utf8` column without error.

Rust has to connect as `latin1` too, in **both** modes. Otherwise rows written by one side read as different
text on the other while they share the database. Removing that setting makes the golden test fail on the
stored bytes, and `both_modes_store_text_the_same_way` checks the fixed mode. Fixing the stored text is a data
migration, for after PHP is gone: convert the columns, then connect as `utf8mb4`.

### Found and kept

- **`get_one.php` takes its id from a JSON body on a GET request.** Some HTTP clients and proxies drop GET
  bodies. Moving the id to the query string changes the API, so it isn't a port decision.
- **There is no authentication**, and `Access-Control-Allow-Origin: *` lets any web page call all five
  endpoints, writes included.
- **No length or range checks in PHP.** The database's `NOT NULL`, `varchar(100)` and strict mode are the
  only rules. The fixed mode adds checks only for what the column can store.

## Result

- In compat mode, all 28 sessions (79 requests) match PHP byte for byte, on status, headers, body and the
  stored rows, and so do the 5 database-down responses
- The same holds against the running service, and through nginx with routes split between PHP and Rust
- 17 tests: the golden replay, the database-down case, PHP's float and JSON string formatting, and 13 for
  the fixed mode. Clean `cargo clippy --all-targets` and `cargo fmt --check`
- Four dependencies: axum, tokio, sqlx and serde_json

## Projects

| Folder | Stack |
|---|---|
| [`php/`](./php) | PHP 7.3, PDO, MariaDB 10.4, Apache |
| [`rust/`](./rust) | Rust 2021, axum 0.7, sqlx 0.8 |

```bash
docker compose up -d --build     # MariaDB, PHP on :8180, Rust on :8181, nginx on :8100
curl -X GET localhost:8100/api/get_one.php -d '{"id":16}'

./php/capture-golden.sh          # re-records rust/tests/golden from PHP
PHP_URL=http://localhost:8181 ./php/capture-golden.sh && git diff --stat   # or from Rust: no diff
```

Rust runs in compat mode under compose. `STUDENTS_COMPAT=0 docker compose up -d rust` switches it to the fixed
mode. [`rust/README.md`](./rust/README.md) covers running the tests.

## Licence

The PHP code is © Hanoak and licensed under the MIT licence. The Rust port is released under the same licence.
See [`LICENSE`](./LICENSE).
