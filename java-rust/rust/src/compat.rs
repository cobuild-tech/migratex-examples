//! FlyAway as the Java ran it: three servlets and eleven JSP pages sharing
//! one set of static fields.
//!
//! `model.Search`, `model.Booking` and `servlets.Login` keep everything in
//! `static` fields, so there is one search, one booking and one admin login
//! for the whole server, shared by every visitor. `Statics` is those fields,
//! and each function below does what one servlet or page did to them, in the
//! same order, including the half-finished updates left by an exception.

use crate::db;
use crate::java::{parse_int, parse_local_date, show, JString, Throwable};
use crate::jsp::{lines, Charset, Page};
use crate::tomcat::{self, Params, Reply};
use axum::http::{Method, StatusCode};
use sqlx::MySqlPool;

/// `model.Search`.
#[derive(Default)]
pub struct Search {
    pub date: JString,
    pub day: JString,
    pub source: JString,
    pub destination: JString,
    pub persons: i32,
}

impl Search {
    /// `Search.getQuery()`: the values are pasted into the SQL.
    pub fn query(&self) -> String {
        format!(
            "SELECT * FROM flight WHERE source = '{}' AND destination='{}' AND days like '%{}%'",
            show(&self.source),
            show(&self.destination),
            show(&self.day)
        )
    }
}

/// `model.Booking`.
#[derive(Default)]
pub struct Booking {
    pub flight_booking_id: JString,
    pub ticket_price: i32,
    pub flight_name: JString,
    pub passenger_name: JString,
    pub passenger_email: JString,
    pub passenger_phone: JString,
    pub name_on_card: JString,
    pub card_details: JString,
}

/// `servlets.Login`.
pub struct Login {
    pub is_logged_in: bool,
    pub password: String,
}

pub const ADMIN_EMAIL: &str = "admin@flyaway.com";

impl Default for Login {
    fn default() -> Login {
        Login {
            is_logged_in: false,
            password: "Admin".into(),
        }
    }
}

/// Every static field in the app. A fresh `Statics` is a freshly started
/// Tomcat.
#[derive(Default)]
pub struct Statics {
    pub search: Search,
    pub booking: Booking,
    pub login: Login,
}

pub const JSPS: [&str; 11] = [
    "index.jsp",
    "search-portal.jsp",
    "search-results.jsp",
    "booking-details.jsp",
    "payment.jsp",
    "thank-you.jsp",
    "login.jsp",
    "admin-dashboard.jsp",
    "change-password.jsp",
    "add-flight.jsp",
    "show-bookings.jsp",
];

fn source(jsp: &str) -> &'static str {
    match jsp {
        "index.jsp" => include_str!("../../java/web/index.jsp"),
        "search-portal.jsp" => include_str!("../../java/web/search-portal.jsp"),
        "search-results.jsp" => include_str!("../../java/web/search-results.jsp"),
        "booking-details.jsp" => include_str!("../../java/web/booking-details.jsp"),
        "payment.jsp" => include_str!("../../java/web/payment.jsp"),
        "thank-you.jsp" => include_str!("../../java/web/thank-you.jsp"),
        "login.jsp" => include_str!("../../java/web/login.jsp"),
        "admin-dashboard.jsp" => include_str!("../../java/web/admin-dashboard.jsp"),
        "change-password.jsp" => include_str!("../../java/web/change-password.jsp"),
        "add-flight.jsp" => include_str!("../../java/web/add-flight.jsp"),
        "show-bookings.jsp" => include_str!("../../java/web/show-bookings.jsp"),
        _ => unreachable!(),
    }
}

// ---------------------------------------------------------------------------
// Stack frames below the application's own

const WS_FILTER: &str = "org.apache.tomcat.websocket.server.WsFilter.doFilter(WsFilter.java:53)";
const SERVICE: &str = "javax.servlet.http.HttpServlet.service(HttpServlet.java:733)";

/// Under a servlet that overrides `service()`.
const SERVLET: [&str; 2] = [SERVICE, WS_FILTER];
/// Under a servlet's `doPost()`.
const DO_POST: [&str; 3] = [
    "javax.servlet.http.HttpServlet.service(HttpServlet.java:652)",
    SERVICE,
    WS_FILTER,
];
/// Under a JSP's generated `_jspService()`.
const JSP: [&str; 7] = [
    "org.apache.jasper.runtime.HttpJspBase.service(HttpJspBase.java:71)",
    SERVICE,
    "org.apache.jasper.servlet.JspServletWrapper.service(JspServletWrapper.java:467)",
    "org.apache.jasper.servlet.JspServlet.serviceJspFile(JspServlet.java:378)",
    "org.apache.jasper.servlet.JspServlet.service(JspServlet.java:326)",
    SERVICE,
    WS_FILTER,
];

/// An exception from a JSP, as Jasper wraps it: the JSP line it came from,
/// with three lines either side, and the original as the root cause.
/// `generated` is the frame in the servlet Jasper made from the page.
fn jasper(jsp: &str, line: usize, generated: &str, cause: Throwable) -> Throwable {
    let cause = cause.at(&[generated]).at(&JSP);
    let src = lines(source(jsp));
    let mut message = format!("An exception occurred processing [{jsp}] at line [{line}]\n\n");
    for n in line.saturating_sub(3).max(1)..=(line + 3).min(src.len()) {
        message.push_str(&format!("{n}: {}\n", src[n - 1]));
    }
    message.push_str("\n\nStacktrace:");
    Throwable {
        class: "org.apache.jasper.JasperException",
        message: Some(message),
        frames: [
            "org.apache.jasper.servlet.JspServletWrapper.handleJspException(JspServletWrapper.java:611)",
            "org.apache.jasper.servlet.JspServletWrapper.service(JspServletWrapper.java:500)",
            "org.apache.jasper.servlet.JspServlet.serviceJspFile(JspServlet.java:378)",
            "org.apache.jasper.servlet.JspServlet.service(JspServlet.java:326)",
            SERVICE,
            WS_FILTER,
        ]
        .iter()
        .map(|f| f.to_string())
        .collect(),
        cause: Some(Box::new(cause)),
    }
}

/// A NullPointerException from calling `.equals` on a null String.
fn npe() -> Throwable {
    Throwable::null_pointer(&[])
}

// ---------------------------------------------------------------------------
// Dispatch

/// Routes one request the way Tomcat would with FlyAway's web.xml.
pub async fn handle(
    statics: &mut Statics,
    pool: &MySqlPool,
    method: &Method,
    path: &str,
    params: &Params,
) -> Reply {
    match path {
        "/search" => search_servlet(statics, params),
        "/login" => do_post_only(method, || login_servlet(statics, params)),
        "/change-password" => do_post_only(method, || change_password_servlet(statics, params)),
        "/" => jsp(statics, pool, method, "index.jsp", params).await,
        _ => {
            if let Some(name) = path.strip_prefix('/').filter(|n| JSPS.contains(n)) {
                jsp(statics, pool, method, name, params).await
            } else if let Some(file) = tomcat::static_file(path) {
                file
            } else {
                tomcat::not_found(path)
            }
        }
    }
}

/// `HttpServlet.service()` for a servlet that only overrides `doPost`.
fn do_post_only(method: &Method, post: impl FnOnce() -> Reply) -> Reply {
    const ALLOW: &str = "POST, OPTIONS";
    match method.as_str() {
        "POST" => post(),
        "OPTIONS" => Reply {
            status: StatusCode::OK,
            headers: vec![("allow", ALLOW.into())],
            body: Vec::new(),
        },
        "TRACE" => tomcat::method_not_allowed("TRACE method is not allowed", Some(ALLOW)),
        "HEAD" => tomcat::method_not_allowed("HTTP method GET is not supported by this URL", None),
        "GET" | "PUT" | "DELETE" => tomcat::method_not_allowed(
            &format!("HTTP method {method} is not supported by this URL"),
            None,
        ),
        other => tomcat::status_page(
            StatusCode::NOT_IMPLEMENTED,
            Some(&format!(
                "Method [{other}] is not implemented by this Servlet for this URI"
            )),
        ),
    }
}

async fn jsp(
    statics: &mut Statics,
    pool: &MySqlPool,
    method: &Method,
    name: &str,
    params: &Params,
) -> Reply {
    const ALLOW: &str = "GET, HEAD, POST, OPTIONS";
    match method.as_str() {
        "GET" | "POST" | "HEAD" => {}
        "OPTIONS" => {
            return Reply {
                status: StatusCode::OK,
                headers: vec![("allow", ALLOW.into())],
                body: Vec::new(),
            }
        }
        _ => {
            return tomcat::method_not_allowed(
                "JSPs only permit GET, POST or HEAD. Jasper also permits OPTIONS",
                Some(ALLOW),
            )
        }
    }
    let charset = if name == "search-portal.jsp" {
        Charset::Utf8
    } else {
        Charset::Latin1
    };
    let page = Page::new(source(name), charset);
    let s = statics;
    let result = match name {
        "search-results.jsp" => Ok(search_results(page, s, pool).await),
        "booking-details.jsp" => booking_details(page, s, params),
        "payment.jsp" => payment(page, s, params),
        "thank-you.jsp" => thank_you(page, s, params),
        "login.jsp" => Ok(login_page(page, s)),
        "admin-dashboard.jsp" => Ok(admin_dashboard(page, s, pool).await),
        "change-password.jsp" => Ok(change_password_page(page, s)),
        // index, search-portal, add-flight and show-bookings have no Java.
        _ => Ok(page),
    };
    match result {
        Ok(page) => Reply::ok(Some(charset.content_type()), page.finish()),
        Err(e) => tomcat::exception(&e, false),
    }
}

// ---------------------------------------------------------------------------
// Servlets

/// The body a servlet wrote with `resp.getWriter()`. The servlets never set
/// a content type, so Tomcat sends none and encodes as ISO-8859-1.
fn written(text: String) -> Reply {
    Reply::ok(None, Charset::Latin1.encode(&text))
}

/// `servlets.SearchServlet`, which overrides `service()` and so answers
/// every method.
fn search_servlet(st: &mut Statics, params: &Params) -> Reply {
    let line = |n: u32| format!("servlets.SearchServlet.service(SearchServlet.java:{n})");
    st.search.date = params.get("date");
    st.search.source = params.get("source");
    st.search.destination = params.get("destination");
    match parse_int(params.get("persons").as_deref()) {
        Ok(persons) => st.search.persons = persons,
        Err(e) => return tomcat::exception(&e.at(&[&line(19)]).at(&SERVLET), false),
    }
    match st.search.date.as_deref() {
        None => tomcat::exception(&npe().at(&[&line(21)]).at(&SERVLET), false),
        Some("") => written("Please enter a valid date\n".into()),
        Some(date) => match parse_local_date(date) {
            Ok(d) => {
                st.search.day = Some(d.day_of_week().into());
                Reply::redirect("/search-results.jsp")
            }
            Err(e) => {
                let e = e
                    .at(&[
                        "servlets.SearchServlet.getDay(SearchServlet.java:32)",
                        &line(26),
                    ])
                    .at(&SERVLET);
                tomcat::exception(&e, false)
            }
        },
    }
}

/// `servlets.Login.doPost`. It calls `getWriter()` first, so its error page
/// comes out in ISO-8859-1.
fn login_servlet(st: &mut Statics, params: &Params) -> Reply {
    let failed = || {
        tomcat::exception(
            &npe()
                .at(&["servlets.Login.doPost(Login.java:22)"])
                .at(&DO_POST),
            true,
        )
    };
    let email = params.get("emailEntered");
    let pass = params.get("passwordEntered");
    let Some(email) = email else { return failed() };
    if email == ADMIN_EMAIL {
        let Some(pass) = pass else { return failed() };
        if pass == st.login.password {
            st.login.is_logged_in = true;
            // "You have LoggedIn" is printed, then dropped by the redirect.
            return Reply::redirect("/admin-dashboard.jsp");
        }
    }
    st.login.is_logged_in = false;
    written("Login Failed : Incorrect email or Password\n".into())
}

/// `servlets.ChangePassword.doPost`.
fn change_password_servlet(st: &mut Statics, params: &Params) -> Reply {
    let pass = params.get("passwordEntered");
    if !st.login.is_logged_in {
        return written("You must login first\n".into());
    }
    match pass {
        None => tomcat::exception(
            &npe()
                .at(&["servlets.ChangePassword.doPost(ChangePassword.java:19)"])
                .at(&DO_POST),
            true,
        ),
        Some(p) if p.is_empty() => written("Password can't be empty\n".into()),
        Some(p) => {
            st.login.password = p;
            written(format!(
                "Password changed. New Password is {}\n",
                st.login.password
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// JSP pages

async fn search_results(mut p: Page, st: &mut Statics, pool: &MySqlPool) -> Page {
    p.code("String query = Search.getQuery();");
    let query = st.search.query();
    p.expr("Search.source", show(&st.search.source));
    p.expr("Search.destination", show(&st.search.destination));
    p.expr("Search.date", show(&st.search.date));
    p.expr("Search.day", show(&st.search.day));
    p.expr("Search.persons", st.search.persons);
    p.code("statement.executeQuery(query)");
    // An SQL error is printed to the log and the table stays empty.
    let flights = db::flights_by_query(pool, &query).await.unwrap_or_default();
    let body = p.mark();
    for f in &flights {
        p.rewind(body);
        p.expr("getString(\"name\")", show(&f.name));
        p.expr("getString(\"source\")", show(&f.source));
        p.expr("getString(\"destination\")", show(&f.destination));
        p.expr("Search.date", show(&st.search.date));
        p.expr("getString(\"ticket_price\")", show(&f.ticket_price));
        p.expr("getString(\"name\")", show(&f.name));
        p.expr("getString(\"ticket_price\")", show(&f.ticket_price));
        p.expr("getString(\"flight_id\")", &f.flight_id);
        p.flush();
    }
    if flights.is_empty() {
        p.skip("printStackTrace");
    } else {
        p.code("printStackTrace");
    }
    p
}

fn booking_details(mut p: Page, st: &mut Statics, params: &Params) -> Result<Page, Throwable> {
    p.code("Booking.flight_booking_id");
    st.booking.flight_booking_id = params.get("flight_id");
    st.booking.ticket_price = parse_int(params.get("ticket_price").as_deref()).map_err(|e| {
        jasper(
            "booking-details.jsp",
            4,
            "org.apache.jsp.booking_002ddetails_jsp._jspService(booking_002ddetails_jsp.java:122)",
            e,
        )
    })?;
    st.booking.flight_name = params.get("name");
    Ok(p)
}

fn payment(mut p: Page, st: &mut Statics, params: &Params) -> Result<Page, Throwable> {
    p.code("Booking.passenger_name");
    let b = &mut st.booking;
    b.passenger_name = params.get("pname");
    b.passenger_email = params.get("email");
    b.passenger_phone = params.get("phone");
    // `a.equals("") || b.equals("") || c.equals("")`, one line each.
    let at = |line: usize| {
        jasper(
            "payment.jsp",
            line,
            &format!(
                "org.apache.jsp.payment_jsp._jspService(payment_jsp.java:{})",
                line + 119
            ),
            npe(),
        )
    };
    let mut invalid = false;
    for (line, field) in [
        (8, &b.passenger_name),
        (9, &b.passenger_email),
        (10, &b.passenger_phone),
    ] {
        match field.as_deref() {
            None => return Err(at(line)),
            Some("") => {
                invalid = true;
                break;
            }
            Some(_) => {}
        }
    }
    if invalid {
        p.println("Please enter valid passenger details");
        p.skip("}");
        return Ok(p);
    }
    p.expr("Booking.ticket_price", st.booking.ticket_price);
    p.expr("Search.persons", st.search.persons);
    p.expr(
        "Booking.ticket_price*Search.persons",
        st.booking.ticket_price.wrapping_mul(st.search.persons),
    );
    p.code("}");
    Ok(p)
}

fn thank_you(mut p: Page, st: &mut Statics, params: &Params) -> Result<Page, Throwable> {
    p.code("Booking.name_on_card");
    st.booking.name_on_card = params.get("name_on_card");
    st.booking.card_details = params.get("card_details");
    let npe = || {
        jasper(
            "thank-you.jsp",
            7,
            "org.apache.jsp.thank_002dyou_jsp._jspService(thank_002dyou_jsp.java:126)",
            npe(),
        )
    };
    let name = st.booking.name_on_card.as_deref().ok_or_else(npe)?;
    let empty = name.is_empty()
        || st
            .booking
            .card_details
            .as_deref()
            .ok_or_else(npe)?
            .is_empty();
    if empty {
        // The message is printed and the page carries on regardless.
        p.println("Please enter valid card details");
    }
    let (s, b) = (&st.search, &st.booking);
    p.expr("Search.source", show(&s.source));
    p.expr("Search.destination", show(&s.destination));
    p.expr("Search.date", show(&s.date));
    p.expr("Search.day", show(&s.day));
    p.expr("Booking.flight_booking_id", show(&b.flight_booking_id));
    p.expr("Booking.flight_name", show(&b.flight_name));
    p.expr("Search.persons", s.persons);
    p.expr("Booking.passenger_name", show(&b.passenger_name));
    p.expr("Booking.passenger_email", show(&b.passenger_email));
    p.expr("Booking.passenger_phone", show(&b.passenger_phone));
    p.expr("Booking.name_on_card", show(&b.name_on_card));
    p.expr("Booking.card_details", show(&b.card_details));
    p.expr(
        "Search.persons*Booking.ticket_price",
        s.persons.wrapping_mul(b.ticket_price),
    );
    Ok(p)
}

fn login_page(mut p: Page, st: &mut Statics) -> Page {
    // Opening the login page logs the admin out, for everyone.
    p.code("Login.isLoggedIn = false;");
    st.login.is_logged_in = false;
    p.expr("Login.password", &st.login.password);
    p
}

fn change_password_page(mut p: Page, st: &mut Statics) -> Page {
    p.code("if (Login.isLoggedIn)");
    if st.login.is_logged_in {
        p.expr("Login.password", &st.login.password);
        p.code("You must Login first");
    } else {
        p.skip("You must Login first");
        p.print("You must Login first");
    }
    p
}

async fn admin_dashboard(mut p: Page, st: &mut Statics, pool: &MySqlPool) -> Page {
    p.code("driverName");
    p.code("if (Login.isLoggedIn)");
    if !st.login.is_logged_in {
        p.skip("You must Login first");
        p.print("You must Login first");
        return p;
    }
    p.expr("Login.email", ADMIN_EMAIL);
    p.expr("Login.password", &st.login.password);
    p.code("SELECT * FROM flight");
    let flights = db::all_flights(pool).await.unwrap_or_default();
    let body = p.mark();
    for f in &flights {
        p.rewind(body);
        p.expr("getString(\"flight_id\")", &f.flight_id);
        p.expr("getString(\"name\")", show(&f.name));
        p.expr("getString(\"source\")", show(&f.source));
        p.expr("getString(\"destination\")", show(&f.destination));
        p.expr("getString(\"days\")", show(&f.days));
        p.expr("getString(\"ticket_price\")", show(&f.ticket_price));
        p.flush();
    }
    if flights.is_empty() {
        p.skip("You must Login first");
    } else {
        p.code("You must Login first");
    }
    p
}
