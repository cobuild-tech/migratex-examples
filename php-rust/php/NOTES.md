# Students API (PHP)

Every file from [php_rest_api_without_framework](https://github.com/hanoak/php_rest_api_without_framework) by
Hanoak is copied here unchanged, from commit
[`0b0acaf`](https://github.com/hanoak/php_rest_api_without_framework/tree/0b0acaf). `README.md` is the
upstream README. These notes cover the files we added:

| File | What it is |
|---|---|
| `docker/` | PHP 7.3 under Apache with `pdo_mysql` and XAMPP's error settings, as the dump was made with |
| `cases/` | The scripted sessions: one request per line, `METHOD PATH [BODY]` |
| `capture-golden.sh` | Records the responses and table contents the Rust tests compare against |

`config/Database.php` connects as `root` with no password to `localhost`, which PDO reads as "use the Unix
socket", so `../docker-compose.yml` shares MariaDB's socket with the PHP container instead of changing the
file. The API is served at `/api/…`, not under `/php_rest_api_without_framework/api/…` as in the upstream
README.

```bash
cd .. && docker compose up -d --build db php
curl localhost:8180/api/get.php
./php/capture-golden.sh          # needs curl and docker
```
