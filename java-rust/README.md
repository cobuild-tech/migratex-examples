# Migration: Java → Rust

The [`rust/`](./rust) service is a rewrite of [`java/`](./java), a flight-booking site built with JSP pages and
Servlets on Tomcat, reading a MySQL `flight` table over JDBC. Visitors search for flights, pick one, enter
passenger and card details and get a ticket summary. An admin logs in to see the flight table and change the
password. The rewrite serves the same URLs from the same database. In compat mode it sends back the same bytes
as Tomcat, Java stack traces included. This document describes how that migration was approached.

The original is [FlyAway](https://github.com/abhinov-gogoi/FlyAway) by Abhinov Gogoi, used under the GNU
GPL v3 (see [Licence](#licence)). The method is the one in our guide
[Java to Rust: a migration guide](https://cobuildx.ai/blog/java-to-rust-migration-guide).

## Strategy

The guide moves one route at a time behind a proxy, and this example is set up that way:
[`nginx.conf`](./nginx.conf) sends each URL to either Java or Rust, and both use one MySQL.

Here the routes can't move one at a time, because FlyAway keeps its state in memory. `model.Search`,
`model.Booking` and `servlets.Login` hold everything in `static` fields, which live inside one JVM. A search made
on the Java side is invisible to Rust. So the URLs move in **groups that share state**:

| Group | State | URLs |
|---|---|---|
| pages | none | `/`, `index.jsp`, `search-portal.jsp`, `add-flight.jsp`, `show-bookings.jsp`, `/img/`, `/style/` |
| booking | `Search`, `Booking` | `/search`, `search-results.jsp`, `booking-details.jsp`, `payment.jsp`, `thank-you.jsp` |
| admin | `Login` | `login.jsp`, `/login`, `admin-dashboard.jsp`, `change-password.jsp`, `/change-password` |

In `nginx.conf` the pages and booking groups are on Rust and admin is still on Java. Moving a group, or moving
it back, is a few `proxy_pass` lines and a reload.

That only works if a client can't tell which side answered. So, as in our COBOL and PHP examples, the port has
**two modes**:

- `FLYAWAY_COMPAT=1` answers exactly as Tomcat did: the same status, the same headers, and the same body bytes.
  That includes Tomcat's error pages with their Java stack traces, `null` printed for a missing value, and
  `?` for a character ISO-8859-1 doesn't have. This is the mode to run while both sides serve traffic, and the
  mode the golden tests run.
- The default mode fixes the bugs listed below. It is in `src/fixed.rs`, and each fix has a test in
  `tests/fixed.rs`.

The port matched every recording in compat mode first. The fixed mode was written after that.

## Architectural mapping

| Java | Rust | Notes |
|---|---|---|
| `web.xml` servlet mappings, Jasper, the DefaultServlet | `compat::handle` | One fallback handler that routes as Tomcat did, odd methods and paths included |
| `static` fields in `Search`, `Booking`, `Login` | `compat::Statics` behind a `tokio::sync::Mutex` | One copy for the whole server, as in Java |
| JSP text between `<% %>` tags | `jsp::Page`, which reads the original `.jsp` files | Jasper copies that text unchanged, CRLF line endings included, so the port writes it from the same files |
| Scriptlets and `<%= %>` | One Rust function per page, walking the page's tags in order | Walking past a tag without handling it panics |
| `String` that may be `null` | `Option<String>`, printed as `null` | `java::show` |
| `Integer.parseInt` | `java::parse_int` | Line for line: `+3`, `007` and Arabic-Indic `٣` are all accepted, and each failure throws from Java's line |
| `LocalDate.parse`, `getDayOfWeek()` | `java::parse_local_date`, `LocalDate::day_of_week` | ISO_LOCAL_DATE in strict mode: signed years, error indexes, `Invalid date 'FEBRUARY 30'` |
| Exceptions and Tomcat's error page | `java::Throwable`, `tomcat::exception` | The frames are OpenJDK 1.8.0_282's and Tomcat 9.0.44's, so the page matches byte for byte |
| `request.getParameter` | `tomcat::Params` | Query string decoded as UTF-8, form body as ISO-8859-1, as Tomcat does with no charset set |
| `DriverManager.getConnection` on every request | A sqlx pool | Java opened a connection per page and never closed it |
| `Search.getQuery()` string concatenation | `db::flights_by_query` in compat, `db::search` with bound values in fixed | |
| `response.sendRedirect("/x.jsp")` | 302 with `Location: /x.jsp` | Tomcat 9 sends relative redirects |

## Changes to the original

The [`java/`](./java) folder is FlyAway at commit
[`55c8bc7`](https://github.com/abhinov-gogoi/FlyAway/tree/55c8bc770869ddfc08f457ee90dd9c12c9901325), without the
IDE files, the prebuilt 10 MB `FlyAway.war` and the screenshots. Two things were changed so it runs here:

- **The database credentials are redacted.** The upstream JSPs and `WEB-INF/common-bugs-fix.txt` contain the
  host, user and password of three hosted databases. They are replaced with `<redacted>`, and the live ones
  point at the `db` service.
- **The database in the dump is renamed** from its hosted name to `flyaway`.

Everything else is byte for byte as upstream, and [`.gitattributes`](./.gitattributes) keeps git from
changing the CRLF line endings.

## Process followed

1. **Read all 1,014 lines** (five Java classes and eleven JSPs) and listed what the app does without saying so:
   the shared static state, the password on the login page, the price taken from a hidden field, and the
   rest under [What the move found](#what-the-move-found).
2. **Ran the original as it was run.** The upstream run configuration is Tomcat 9.0.44, and the last commit
   moved the code to Java 8. [`java/docker/Dockerfile`](./java/docker/Dockerfile) compiles the servlets with
   that image's JDK (1.8.0_282) and deploys the JSPs unchanged as the ROOT app, with Connector/J 8.0.22 from the
   `pom.xml`. The dump is from MySQL 8.0, so the database is `mysql:8.0`.
3. **Recorded golden sessions.** 13 scripted sessions in `java/cases/` (174 requests) cover every page and
   servlet, the booking flow, the admin flow, wrong methods, missing and malformed parameters, odd numbers
   and dates, injected SQL and HTML, and non-ASCII text. `java/capture-golden.sh` restarts Tomcat before each
   session, because the static fields would otherwise carry over, and saves the status, the headers clients
   see, and the body. Recording twice gives identical files.
4. **Ported each servlet and page** as a function that follows it line by line (`src/compat.rs`), with the
   JDK's behaviour in `src/java.rs` and Tomcat's in `src/tomcat.rs`. The golden test (`tests/golden.rs`)
   replays every session against the router and compares everything byte for byte.
5. **Checked it from outside**, with the real servers: `capture-golden.sh` pointed at the Rust service
   records files identical to Tomcat's. Through nginx, with the groups split between the two sides, every
   file matches except one, a `TRACE` request that nginx answers itself.
6. **Added the fixes** in `src/fixed.rs`, each with a test.

## What the move found

### Fixed in the default mode

- **Every visitor shares one booking.** The search, the chosen flight, the passenger and the card are static
  fields, so a second visitor's search replaces the first's, and the thank-you page shows whoever paid last.
  The port keeps each visitor's booking in their own session.
- **Every visitor shares one login.** `Login.isLoggedIn` is a static boolean. Once the admin logs in, anyone
  can open the dashboard. Anyone who fails to log in, or merely opens `login.jsp`, logs the admin out. The port
  logs in a session, and gives it a new id when it does.
- **The admin password is on the login page.** `login.jsp` prints "Current Password : Admin". The dashboard
  and the change-password page print it too, and changing it echoes the new one back. The port keeps an Argon2
  hash, starts from `FLYAWAY_ADMIN_PASSWORD` instead of `Admin`, and never shows it.
- **The visitor sets the price.** The ticket price comes from a hidden field on the results page.
  `ticket_price=1` pays Rs.1, `-100` pays Rs.-500, and `2147483647` for five people overflows the `int` to
  Rs.2147483643. The port reads the price from the table, and checks that the flight is on the searched route
  and day.
- **The search is SQL injection.** `Search.getQuery()` pastes the values into the SQL, so
  `source=India' OR '1'='1` changes the query. Connector/J doesn't allow a second statement, so it can read
  but not write. The port binds the values.
- **Nothing is HTML-escaped.** Every `<%= %>` writes the value as is, so search parameters and the hidden fields
  can inject markup and scripts. askama escapes by default.
- **Bad input is a stack trace.** A missing field, a word for a number or an impossible date is a 500 page
  showing the exception, the class and line it came from, and the Tomcat version. The port answers 400 with the
  form again.
- **An invalid card is booked anyway.** `thank-you.jsp` prints "Please enter valid card details" and then
  shows the booked ticket. The full card number is on the page and stays in a static field. The port checks
  the number (length and Luhn), shows only the last four digits, and keeps none of it.
- **The checks are loose.** Persons can be 0, negative, or digits from other scripts; dates can be in the past
  or have five-digit signed years. The port takes 1 to 5 travellers and a `YYYY-MM-DD` date from today on.
- **Admin pages are open.** `show-bookings.jsp` and `add-flight.jsp` don't check the login. The port does.
- **Characters outside ISO-8859-1 become `?`.** The pages are written as ISO-8859-1, so a `€` in a search
  shows as `?`. The port writes UTF-8.

These change what clients see, so they belong in an announced release, after the move. During the move, run
compat mode.

### Found and kept

- **Bookings aren't saved anywhere.** Nothing is written to the database, and "Previous bookings" says
  "Coming Soon". The thank-you page is the only record. Storing bookings is a new feature, not part of a port.
- **The admin password resets on restart.** Java kept it in a static field. The port keeps the hash in
  memory too, starting from the environment variable.
- **The schema is loose.** Ticket prices are `varchar`, and the days a flight runs are one comma-separated
  string matched with `LIKE`. Changing them is a data migration, for after Java is gone.
- **There are no CSRF tokens.** The port's session cookie is `SameSite=Strict`, which covers the forms here.

### Not reproduced in compat mode

- **Concurrent requests.** Java let two requests write the static fields at once. The port holds a lock for
  each request, so requests take turns. The result is one of the outcomes Java could have produced.
- **Tomcat's `JSESSIONID` cookie.** Every JSP creates a session that nothing reads. The port doesn't set it.

## Result

- In compat mode, all 13 sessions (174 requests) match Tomcat byte for byte, on status, headers and body
- The same holds against the running service. Through nginx, with routes split between Java and Rust, every
  response matches except `TRACE`, which nginx answers itself
- 17 tests: the golden replay, 2 for the JDK's number and date rules, and 14 for the fixed mode. Clean
  `cargo clippy --all-targets` and `cargo fmt --check`
- Seven dependencies: axum, tokio, sqlx, askama, tower-sessions, argon2 and serde

## Projects

| Folder | Stack |
|---|---|
| [`java/`](./java) | Java 8, Servlets 3.0, JSP, Tomcat 9.0.44, Connector/J 8.0.22, MySQL 8.0 |
| [`rust/`](./rust) | Rust 2021, axum 0.7, sqlx 0.8, askama 0.12, tower-sessions 0.13 |

```bash
docker compose up -d --build     # MySQL, Java on :8280, Rust on :8281, nginx on :8200
open http://localhost:8200/

./java/capture-golden.sh         # re-records rust/tests/golden from Java
JAVA_URL=http://localhost:8281 ./java/capture-golden.sh && git diff --stat   # or from Rust: no diff
```

The Tomcat image is amd64 only. On Apple silicon Docker runs it emulated, which is slower but gives the same
pages.

Rust runs in compat mode under compose. `FLYAWAY_COMPAT=0 FLYAWAY_ADMIN_PASSWORD=… docker compose up -d rust`
switches it to the fixed mode. [`rust/README.md`](./rust/README.md) covers running the tests.

## Licence

FlyAway is © Abhinov Gogoi and licensed under the GNU General Public License v3.0. The Rust port is a
derivative work and is released under the same licence. See [`LICENSE`](./LICENSE).
