//! The default mode: one test per fixed bug, each against what the Java did.
//!
//! Needs MySQL with the flight table: `docker compose up -d db` in java-rust/.

mod common;

use axum::http::StatusCode;
use common::*;
use flyaway::java::parse_local_date;
use flyaway::Mode;

const PASSWORD: &str = "correct horse battery";

/// One browser: it keeps the session cookie the app sets.
struct Visitor {
    app: axum::Router,
    cookie: Option<String>,
}

impl Visitor {
    fn new(app: &axum::Router) -> Visitor {
        Visitor {
            app: app.clone(),
            cookie: None,
        }
    }

    async fn send(&mut self, method: &str, path: &str, body: Option<&str>) -> Response {
        let r = send_with(&self.app, method, path, body, self.cookie.as_deref()).await;
        if let Some(set) = r.header("set-cookie") {
            self.cookie = Some(set.split(';').next().unwrap().to_string());
        }
        r
    }

    async fn get(&mut self, path: &str) -> Response {
        self.send("GET", path, None).await
    }

    async fn post(&mut self, path: &str, body: &str) -> Response {
        self.send("POST", path, Some(body)).await
    }

    async fn search(
        &mut self,
        source: &str,
        destination: &str,
        day: &str,
        persons: u32,
    ) -> Response {
        let date = future(day);
        self.get(&format!(
            "/search?date={date}&source={source}&destination={destination}&persons={persons}"
        ))
        .await
    }

    async fn log_in(&mut self) -> Response {
        self.post(
            "/login",
            &format!(
                "emailEntered=admin%40flyaway.com&passwordEntered={}",
                PASSWORD.replace(' ', "+")
            ),
        )
        .await
    }
}

fn fixed() -> axum::Router {
    app(Mode::fixed(PASSWORD))
}

/// A date in 2099 that falls on `day`.
fn future(day: &str) -> String {
    (1..=7)
        .map(|d| format!("2099-01-{d:02}"))
        .find(|s| parse_local_date(s).unwrap().day_of_week() == day)
        .unwrap()
}

fn assert_redirect(r: &Response, to: &str) {
    assert_eq!(r.status, StatusCode::SEE_OTHER, "{}", r.text());
    assert_eq!(r.header("location"), Some(to));
}

#[tokio::test]
async fn each_visitor_has_their_own_booking() {
    let app = fixed();
    let (mut ann, mut bob) = (Visitor::new(&app), Visitor::new(&app));
    assert_redirect(
        &ann.search("India", "USA", "MONDAY", 2).await,
        "/search-results.jsp",
    );
    assert_redirect(
        &bob.search("UK", "India", "FRIDAY", 1).await,
        "/search-results.jsp",
    );

    // In Java, Bob's search replaced Ann's: one static Search for everyone.
    let page = ann.get("/search-results.jsp").await.text();
    assert!(page.contains("from India to USA"), "{page}");
    assert!(page.contains("AIR India"));
    assert!(!page.contains("UKAir"));

    // A visitor who hasn't searched is sent to the search form, not shown
    // someone else's search.
    let mut cleo = Visitor::new(&app);
    assert_redirect(&cleo.get("/search-results.jsp").await, "/search-portal.jsp");
    assert_redirect(
        &cleo.post("/payment.jsp", "pname=a").await,
        "/search-portal.jsp",
    );
}

#[tokio::test]
async fn the_price_comes_from_the_database() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    v.search("India", "USA", "MONDAY", 2).await;
    // Java took ticket_price from this form.
    let r = v
        .post("/booking-details.jsp", "flight_id=1&ticket_price=1&name=x")
        .await;
    assert_eq!(r.status, StatusCode::OK);
    assert!(r.text().contains("AIR India, Rs.43000 per person"));
    let r = v
        .post(
            "/payment.jsp",
            "pname=Ada&email=ada%40example.com&phone=5550100",
        )
        .await;
    assert!(
        r.text().contains("43000 X 2 persons = Rs.86000"),
        "{}",
        r.text()
    );
}

#[tokio::test]
async fn a_flight_must_be_one_the_search_found() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    v.search("India", "USA", "MONDAY", 1).await;
    // Flight 4 is UK to India.
    let r = v.post("/booking-details.jsp", "flight_id=4").await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    let r = v.post("/booking-details.jsp", "flight_id=99").await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    let r = v.post("/booking-details.jsp", "").await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn search_values_are_bound_not_pasted() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    // In Java this listed AIR India: the quote closed the SQL string.
    let r = v
        .get(&format!(
            "/search?date={}&source=India'+OR+'1'%3D'1&destination=USA&persons=1",
            future("MONDAY")
        ))
        .await;
    assert_redirect(&r, "/search-results.jsp");
    let page = v.get("/search-results.jsp").await.text();
    assert!(page.contains("No flights"), "{page}");
    assert!(!page.contains("AIR India"));
}

#[tokio::test]
async fn text_is_escaped() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    let r = v
        .get(&format!(
            "/search?date={}&source=%3Cscript%3Ex%3C%2Fscript%3E&destination=USA&persons=1",
            future("MONDAY")
        ))
        .await;
    assert_redirect(&r, "/search-results.jsp");
    let page = v.get("/search-results.jsp").await.text();
    assert!(
        page.contains("from &lt;script&gt;x&lt;/script&gt; to"),
        "{page}"
    );
    assert!(!page.contains("<script>"));
}

#[tokio::test]
async fn bad_input_is_a_400_with_the_form() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    let monday = future("MONDAY");
    for query in [
        // Each was a 500 page with a Java stack trace.
        format!("date={monday}&source=India&destination=USA"),
        format!("date={monday}&source=India&destination=USA&persons=abc"),
        "date=bad&source=India&destination=USA&persons=1".to_string(),
        "date=2099-02-30&source=India&destination=USA&persons=1".to_string(),
        "source=India&destination=USA&persons=1".to_string(),
        // Each was accepted.
        format!("date={monday}&source=India&destination=USA&persons=-2"),
        format!("date={monday}&source=India&destination=USA&persons=%D9%A3"),
        "date=%2B12099-01-05&source=India&destination=USA&persons=1".to_string(),
        "date=2020-01-06&source=India&destination=USA&persons=1".to_string(),
        format!("date={monday}&source=India&destination=India&persons=1"),
        format!("date={monday}&destination=USA&persons=1"),
    ] {
        let r = v.get(&format!("/search?{query}")).await;
        assert_eq!(r.status, StatusCode::BAD_REQUEST, "{query}");
        assert!(r.text().contains("Search Flights"), "{query}");
    }
}

#[tokio::test]
async fn passenger_and_card_details_are_checked() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    v.search("India", "USA", "MONDAY", 1).await;
    v.post("/booking-details.jsp", "flight_id=1").await;
    for body in [
        "",
        "pname=&email=a%40b.c&phone=5550100",
        "pname=A&email=nope&phone=5550100",
        "pname=A&email=a%40b.c&phone=12",
        "pname=A&email=a%40b.c&phone=call+me",
    ] {
        let r = v.post("/payment.jsp", body).await;
        assert_eq!(r.status, StatusCode::BAD_REQUEST, "{body}");
    }
    v.post("/payment.jsp", "pname=Ada&email=a%40b.c&phone=5550100")
        .await;
    // Java printed "Please enter valid card details", then booked it anyway.
    for body in [
        "name_on_card=&card_details=4111111111111111",
        "name_on_card=Ada&card_details=",
        "name_on_card=Ada&card_details=4111111111111112",
        "name_on_card=Ada&card_details=1",
    ] {
        let r = v.post("/thank-you.jsp", body).await;
        assert_eq!(r.status, StatusCode::BAD_REQUEST, "{body}");
        assert!(r.text().contains("Confirm Payment"));
    }
}

#[tokio::test]
async fn the_card_number_is_not_shown_or_kept() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    v.search("UK", "USA", "FRIDAY", 3).await;
    v.post("/booking-details.jsp", "flight_id=5").await;
    v.post("/payment.jsp", "pname=Ada&email=a%40b.c&phone=5550100")
        .await;
    let r = v
        .post(
            "/thank-you.jsp",
            "name_on_card=ADA&card_details=4111+1111+1111+1111",
        )
        .await;
    assert_eq!(r.status, StatusCode::OK);
    let page = r.text();
    assert!(page.contains("**** **** **** 1111"));
    assert!(!page.contains("4111"));
    assert!(page.contains("Total Paid- Rs.102975"));
    // The booking is finished: submitting again starts over.
    let r = v
        .post(
            "/thank-you.jsp",
            "name_on_card=ADA&card_details=4111111111111111",
        )
        .await;
    assert_redirect(&r, "/search-portal.jsp");
}

#[tokio::test]
async fn the_password_is_never_shown() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    // Java printed "Current Password : Admin" on the login page.
    let page = v.get("/login.jsp").await.text();
    assert!(!page.contains(PASSWORD) && !page.contains("Admin<"));
    v.log_in().await;
    for path in ["/admin-dashboard.jsp", "/change-password.jsp"] {
        let page = v.get(path).await.text();
        assert!(!page.contains(PASSWORD), "{path}");
    }
    let r = v
        .post("/change-password", "passwordEntered=quiet+harbour+lights")
        .await;
    assert_eq!(r.status, StatusCode::OK);
    assert!(r.text().contains("Password changed."));
    assert!(!r.text().contains("quiet harbour"));
}

#[tokio::test]
async fn the_default_password_does_not_work() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    let r = v
        .post(
            "/login",
            "emailEntered=admin%40flyaway.com&passwordEntered=Admin",
        )
        .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    assert_redirect(&v.get("/admin-dashboard.jsp").await, "/login.jsp");
}

#[tokio::test]
async fn logging_in_is_per_visitor() {
    let app = fixed();
    let (mut admin, mut other) = (Visitor::new(&app), Visitor::new(&app));
    assert_redirect(&admin.log_in().await, "/admin-dashboard.jsp");
    assert_eq!(
        admin.get("/admin-dashboard.jsp").await.status,
        StatusCode::OK
    );

    // In Java, Login.isLoggedIn was one static boolean: once the admin had
    // logged in, anyone could open the dashboard...
    assert_redirect(&other.get("/admin-dashboard.jsp").await, "/login.jsp");
    // ...and anyone's failed login, or a look at the login page, logged
    // the admin out.
    let r = other
        .post(
            "/login",
            "emailEntered=admin%40flyaway.com&passwordEntered=wrong",
        )
        .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    other.get("/login.jsp").await;
    assert_eq!(
        admin.get("/admin-dashboard.jsp").await.status,
        StatusCode::OK
    );

    assert_redirect(&admin.post("/logout", "").await, "/login.jsp");
    assert_redirect(&admin.get("/admin-dashboard.jsp").await, "/login.jsp");
}

#[tokio::test]
async fn admin_pages_need_a_login() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    // show-bookings.jsp and add-flight.jsp were open to anyone.
    for path in [
        "/admin-dashboard.jsp",
        "/change-password.jsp",
        "/show-bookings.jsp",
        "/add-flight.jsp",
    ] {
        assert_redirect(&v.get(path).await, "/login.jsp");
    }
    assert_redirect(
        &v.post("/change-password", "passwordEntered=whatever123")
            .await,
        "/login.jsp",
    );
}

#[tokio::test]
async fn a_changed_password_replaces_the_old_one() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    v.log_in().await;
    // Java threw a NullPointerException with no field, and took " ".
    let r = v.post("/change-password", "").await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    let r = v.post("/change-password", "passwordEntered=+").await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    v.post("/change-password", "passwordEntered=s3cret-enough")
        .await;

    let mut w = Visitor::new(&app);
    assert_eq!(w.log_in().await.status, StatusCode::UNAUTHORIZED);
    let r = w
        .post(
            "/login",
            "emailEntered=admin%40flyaway.com&passwordEntered=s3cret-enough",
        )
        .await;
    assert_redirect(&r, "/admin-dashboard.jsp");
}

#[tokio::test]
async fn wrong_methods_and_paths() {
    let app = fixed();
    let mut v = Visitor::new(&app);
    let r = v.send("PUT", "/index.jsp", None).await;
    assert_eq!(r.status, StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(r.header("allow"), Some("GET,HEAD"));
    let r = v.get("/login").await;
    assert_eq!(r.status, StatusCode::METHOD_NOT_ALLOWED);
    let r = v.get("/WEB-INF/web.xml").await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
    let r = v.get("/img/favicon.png").await;
    assert_eq!(r.header("content-type"), Some("image/png"));
}
