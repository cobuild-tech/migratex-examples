//! FlyAway with its bugs fixed: the mode to run once Java is gone.
//!
//! Each fix is noted where it is made, with what the Java did:
//!
//! - State is kept per visitor in a session, not in static fields that
//!   every visitor shares.
//! - The admin password is an Argon2 hash, never shown, and starts from
//!   FLYAWAY_ADMIN_PASSWORD rather than "Admin".
//! - The ticket price is read from the database, not from a hidden field.
//! - SQL values are bound, not pasted into the query.
//! - Every value is HTML-escaped (askama does it by default).
//! - Bad input is a 400 page with the form again, not a 500 stack trace.
//! - The card number is never stored or shown, only its last four digits.

use crate::db::{self, Flight};
use crate::java::{parse_local_date, LocalDate};
use crate::tomcat;
use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString};
use argon2::{Argon2, PasswordVerifier};
use askama::Template;
use axum::extract::{Form, Query, State};
use axum::http::{StatusCode, Uri};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};

pub const ADMIN_EMAIL: &str = crate::compat::ADMIN_EMAIL;

/// The admin password's hash. Java kept the password itself in a static
/// field and printed it on the login page.
#[derive(Clone)]
pub struct Admin(Arc<RwLock<String>>);

fn hash(password: &str) -> String {
    Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .expect("hashing a password")
        .to_string()
}

impl Admin {
    pub fn new(password: &str) -> Admin {
        Admin(Arc::new(RwLock::new(hash(password))))
    }

    pub fn verify(&self, password: &str) -> bool {
        let phc = self.0.read().unwrap().clone();
        let parsed = PasswordHash::new(&phc).expect("stored hash");
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    }

    pub fn set(&self, password: &str) {
        *self.0.write().unwrap() = hash(password);
    }
}

#[derive(Clone)]
struct AppState {
    pool: MySqlPool,
    admin: Admin,
}

pub fn router(pool: MySqlPool, admin: Admin) -> Router {
    // Secure cookies need HTTPS; behind TLS, set `with_secure(true)`.
    let sessions = SessionManagerLayer::new(MemoryStore::default()).with_secure(false);
    Router::new()
        .route("/", get(index))
        .route("/index.jsp", get(index))
        .route("/search-portal.jsp", get(search_portal))
        .route("/search", get(search))
        .route("/search-results.jsp", get(search_results))
        .route("/booking-details.jsp", post(booking_details))
        .route("/payment.jsp", post(payment))
        .route("/thank-you.jsp", post(thank_you))
        .route("/login.jsp", get(login_page))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/admin-dashboard.jsp", get(admin_dashboard))
        .route("/change-password.jsp", get(change_password_page))
        .route("/change-password", post(change_password))
        .route("/add-flight.jsp", get(add_flight))
        .route("/show-bookings.jsp", get(show_bookings))
        .fallback(static_or_404)
        .layer(sessions)
        .with_state(AppState { pool, admin })
}

// ---------------------------------------------------------------------------
// What a visitor's session holds

const TRIP: &str = "trip";
const ADMIN: &str = "admin";

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Trip {
    search: Option<SearchForm>,
    flight: Option<Chosen>,
    passenger: Option<Passenger>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SearchForm {
    date: String,
    day: String,
    source: String,
    destination: String,
    persons: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Chosen {
    flight_id: String,
    name: String,
    ticket_price: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Passenger {
    name: String,
    email: String,
    phone: String,
}

async fn trip(session: &Session) -> Trip {
    session
        .get::<Trip>(TRIP)
        .await
        .ok()
        .flatten()
        .unwrap_or_default()
}

/// False if the session store failed.
async fn save(session: &Session, trip: &Trip) -> bool {
    session.insert(TRIP, trip).await.is_ok()
}

async fn is_admin(session: &Session) -> bool {
    session.get::<bool>(ADMIN).await.ok().flatten() == Some(true)
}

// ---------------------------------------------------------------------------
// Responses

fn render(status: StatusCode, t: impl Template) -> Response {
    match t.render() {
        Ok(html) => (status, Html(html)).into_response(),
        Err(_) => internal(),
    }
}

fn page(t: impl Template) -> Response {
    render(StatusCode::OK, t)
}

fn internal() -> Response {
    render(
        StatusCode::INTERNAL_SERVER_ERROR,
        MessageTemplate {
            heading: "Something went wrong",
            text: "Please try again.".into(),
            link: "/index.jsp",
            link_text: "Home",
        },
    )
}

/// After a POST, or to send a visitor back to a step they skipped.
fn see_other(path: &str) -> Response {
    Redirect::to(path).into_response()
}

// ---------------------------------------------------------------------------
// Templates

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "search_portal.html")]
struct SearchPortalTemplate {
    error: Option<String>,
}

/// A row for the pages. A NULL column shows as nothing, where Java printed
/// `null`.
struct FlightView {
    flight_id: String,
    name: String,
    source: String,
    destination: String,
    days: String,
    ticket_price: String,
}

impl From<Flight> for FlightView {
    fn from(f: Flight) -> FlightView {
        FlightView {
            flight_id: f.flight_id,
            name: f.name.unwrap_or_default(),
            source: f.source.unwrap_or_default(),
            destination: f.destination.unwrap_or_default(),
            days: f.days.unwrap_or_default(),
            ticket_price: f.ticket_price.unwrap_or_default(),
        }
    }
}

#[derive(Template)]
#[template(path = "search_results.html")]
struct SearchResultsTemplate {
    search: SearchForm,
    flights: Vec<FlightView>,
}

#[derive(Template)]
#[template(path = "booking_details.html")]
struct BookingDetailsTemplate {
    flight_name: String,
    ticket_price: u64,
    error: Option<&'static str>,
}

#[derive(Template)]
#[template(path = "payment.html")]
struct PaymentTemplate {
    ticket_price: u64,
    persons: u32,
    total: u64,
    error: Option<&'static str>,
}

struct Ticket {
    source: String,
    destination: String,
    date: String,
    day: String,
    flight_id: String,
    flight_name: String,
    persons: u32,
    passenger_name: String,
    passenger_email: String,
    passenger_phone: String,
    name_on_card: String,
    card_last4: String,
    total: u64,
}

#[derive(Template)]
#[template(path = "thank_you.html")]
struct ThankYouTemplate {
    t: Ticket,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<&'static str>,
}

#[derive(Template)]
#[template(path = "admin_dashboard.html")]
struct AdminDashboardTemplate {
    email: &'static str,
    flights: Vec<FlightView>,
}

#[derive(Template)]
#[template(path = "change_password.html")]
struct ChangePasswordTemplate {
    changed: bool,
    error: Option<&'static str>,
}

#[derive(Template)]
#[template(path = "coming_soon.html")]
struct ComingSoonTemplate {
    heading: &'static str,
}

#[derive(Template)]
#[template(path = "message.html")]
struct MessageTemplate {
    heading: &'static str,
    text: String,
    link: &'static str,
    link_text: &'static str,
}

// ---------------------------------------------------------------------------
// Booking

async fn index() -> Response {
    page(IndexTemplate)
}

async fn search_portal() -> Response {
    page(SearchPortalTemplate { error: None })
}

#[derive(Deserialize)]
struct SearchQuery {
    date: Option<String>,
    source: Option<String>,
    destination: Option<String>,
    persons: Option<String>,
}

fn today() -> i64 {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    (secs / 86_400) as i64
}

/// A date as the form's date input sends it, `YYYY-MM-DD`, not in the past.
/// Java took anything `LocalDate.parse` took, signed years included, and
/// answered anything else with a stack trace.
fn travel_date(s: &str) -> Result<LocalDate, &'static str> {
    if s.len() != 10 || !s.bytes().next().is_some_and(|b| b.is_ascii_digit()) {
        return Err("Enter the date as YYYY-MM-DD.");
    }
    let date = parse_local_date(s).map_err(|_| "That date doesn't exist.")?;
    if date.epoch_day() < today() {
        return Err("That date is in the past.");
    }
    Ok(date)
}

/// One of the form's choices, 1 to 5. Java took any int, including 0,
/// negative numbers, and digits from other scripts.
fn persons(s: &str) -> Result<u32, &'static str> {
    match s {
        "1" | "2" | "3" | "4" | "5" => Ok(s.parse().unwrap()),
        _ => Err("Choose between 1 and 5 travellers."),
    }
}

fn place(s: Option<&str>) -> Result<String, &'static str> {
    match s.map(str::trim) {
        Some(p) if !p.is_empty() && p.chars().count() <= 128 => Ok(p.to_string()),
        _ => Err("Choose where you are flying from and to."),
    }
}

/// `/search`. Java answered every method here; GET is what the form sends.
async fn search(session: Session, Query(q): Query<SearchQuery>) -> Response {
    let form = (|| {
        let date = travel_date(q.date.as_deref().unwrap_or(""))?;
        let source = place(q.source.as_deref())?;
        let destination = place(q.destination.as_deref())?;
        if source == destination {
            return Err("Choose two different places.");
        }
        let persons = persons(q.persons.as_deref().unwrap_or(""))?;
        Ok(SearchForm {
            date: q.date.clone().unwrap_or_default(),
            day: date.day_of_week().to_string(),
            source,
            destination,
            persons,
        })
    })();
    match form {
        Err(error) => render(
            StatusCode::BAD_REQUEST,
            SearchPortalTemplate {
                error: Some(error.into()),
            },
        ),
        Ok(form) => {
            // A new search starts a new booking.
            let trip = Trip {
                search: Some(form),
                ..Trip::default()
            };
            if !save(&session, &trip).await {
                return internal();
            }
            see_other("/search-results.jsp")
        }
    }
}

async fn search_results(State(app): State<AppState>, session: Session) -> Response {
    let Some(search) = trip(&session).await.search else {
        return see_other("/search-portal.jsp");
    };
    match db::search(&app.pool, &search.source, &search.destination, &search.day).await {
        Ok(flights) => page(SearchResultsTemplate {
            search,
            flights: flights.into_iter().map(FlightView::from).collect(),
        }),
        // Java printed the stack trace to the log and showed an empty table.
        Err(e) => {
            eprintln!("search: {e}");
            internal()
        }
    }
}

#[derive(Deserialize)]
struct BookingForm {
    flight_id: Option<String>,
}

/// The flight is looked up by id. Java took the name and the price from
/// hidden fields, so a visitor could set their own price.
async fn booking_details(
    State(app): State<AppState>,
    session: Session,
    Form(form): Form<BookingForm>,
) -> Response {
    let mut trip = trip(&session).await;
    let Some(search) = &trip.search else {
        return see_other("/search-portal.jsp");
    };
    let id = form.flight_id.unwrap_or_default();
    let flight = match db::flight_by_id(&app.pool, &id).await {
        Ok(Some(f)) => f,
        Ok(None) => {
            return render(
                StatusCode::BAD_REQUEST,
                MessageTemplate {
                    heading: "Unknown flight",
                    text: "That flight doesn't exist.".into(),
                    link: "/search-results.jsp",
                    link_text: "Back to the results",
                },
            )
        }
        Err(e) => {
            eprintln!("booking: {e}");
            return internal();
        }
    };
    // It must also be one the search found.
    let on_route = flight.source.as_deref() == Some(&search.source)
        && flight.destination.as_deref() == Some(&search.destination)
        && flight
            .days
            .as_deref()
            .is_some_and(|d| d.to_ascii_uppercase().contains(&search.day));
    let price = flight
        .ticket_price
        .as_deref()
        .and_then(|p| p.trim().parse::<u64>().ok());
    let (true, Some(ticket_price)) = (on_route, price) else {
        return render(
            StatusCode::BAD_REQUEST,
            MessageTemplate {
                heading: "Unknown flight",
                text: "That flight doesn't fly on the day you searched for.".into(),
                link: "/search-results.jsp",
                link_text: "Back to the results",
            },
        );
    };
    let chosen = Chosen {
        flight_id: flight.flight_id,
        name: flight.name.unwrap_or_default(),
        ticket_price,
    };
    let view = BookingDetailsTemplate {
        flight_name: chosen.name.clone(),
        ticket_price,
        error: None,
    };
    trip.flight = Some(chosen);
    trip.passenger = None;
    if !save(&session, &trip).await {
        return internal();
    }
    page(view)
}

#[derive(Deserialize)]
struct PassengerForm {
    pname: Option<String>,
    email: Option<String>,
    phone: Option<String>,
}

fn passenger(form: PassengerForm) -> Result<Passenger, &'static str> {
    let field = |v: Option<String>| v.map(|s| s.trim().to_string()).unwrap_or_default();
    let (name, email, phone) = (field(form.pname), field(form.email), field(form.phone));
    if name.is_empty() || name.chars().count() > 100 {
        return Err("Enter the passenger's name.");
    }
    let at = email.find('@');
    if email.chars().count() > 254 || !at.is_some_and(|i| i > 0 && i < email.len() - 1) {
        return Err("Enter a valid email address.");
    }
    let digits = phone.chars().filter(char::is_ascii_digit).count();
    if !(6..=15).contains(&digits)
        || !phone
            .chars()
            .all(|c| c.is_ascii_digit() || " +-()".contains(c))
    {
        return Err("Enter a valid phone number.");
    }
    Ok(Passenger { name, email, phone })
}

async fn payment(session: Session, Form(form): Form<PassengerForm>) -> Response {
    let mut trip = trip(&session).await;
    let (Some(search), Some(flight)) = (&trip.search, &trip.flight) else {
        return see_other("/search-portal.jsp");
    };
    let total = flight.ticket_price * u64::from(search.persons);
    match passenger(form) {
        Err(error) => render(
            StatusCode::BAD_REQUEST,
            BookingDetailsTemplate {
                flight_name: flight.name.clone(),
                ticket_price: flight.ticket_price,
                error: Some(error),
            },
        ),
        Ok(p) => {
            let view = PaymentTemplate {
                ticket_price: flight.ticket_price,
                persons: search.persons,
                total,
                error: None,
            };
            trip.passenger = Some(p);
            if !save(&session, &trip).await {
                return internal();
            }
            page(view)
        }
    }
}

#[derive(Deserialize)]
struct CardForm {
    name_on_card: Option<String>,
    card_details: Option<String>,
}

/// A card number: 12 to 19 digits, spaces allowed, passing the Luhn check.
fn card_number(s: &str) -> Option<String> {
    let digits: Vec<u32> = s
        .chars()
        .filter(|c| *c != ' ')
        .map(|c| c.to_digit(10))
        .collect::<Option<_>>()?;
    if !(12..=19).contains(&digits.len()) {
        return None;
    }
    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| match (i % 2, d * 2) {
            (0, _) => d,
            (_, doubled) if doubled > 9 => doubled - 9,
            (_, doubled) => doubled,
        })
        .sum();
    sum.is_multiple_of(10)
        .then(|| digits.iter().map(|d| d.to_string()).collect())
}

/// Java printed "Please enter valid card details" and then booked the
/// ticket anyway, with the full card number on the page.
async fn thank_you(session: Session, Form(form): Form<CardForm>) -> Response {
    let trip = trip(&session).await;
    let (Some(search), Some(flight), Some(passenger)) =
        (&trip.search, &trip.flight, &trip.passenger)
    else {
        return see_other("/search-portal.jsp");
    };
    let total = flight.ticket_price * u64::from(search.persons);
    let name = form.name_on_card.unwrap_or_default().trim().to_string();
    let card = card_number(form.card_details.as_deref().unwrap_or(""));
    let (false, Some(card)) = (name.is_empty() || name.chars().count() > 100, card) else {
        return render(
            StatusCode::BAD_REQUEST,
            PaymentTemplate {
                ticket_price: flight.ticket_price,
                persons: search.persons,
                total,
                error: Some("Enter the name on the card and a valid card number."),
            },
        );
    };
    let ticket = Ticket {
        source: search.source.clone(),
        destination: search.destination.clone(),
        date: search.date.clone(),
        day: search.day.clone(),
        flight_id: flight.flight_id.clone(),
        flight_name: flight.name.clone(),
        persons: search.persons,
        passenger_name: passenger.name.clone(),
        passenger_email: passenger.email.clone(),
        passenger_phone: passenger.phone.clone(),
        name_on_card: name,
        card_last4: card[card.len() - 4..].to_string(),
        total,
    };
    // The booking is done: the same form can't be submitted twice.
    if session.remove::<Trip>(TRIP).await.is_err() {
        return internal();
    }
    page(ThankYouTemplate { t: ticket })
}

// ---------------------------------------------------------------------------
// Admin

/// Opening the login page no longer logs anyone out.
async fn login_page() -> Response {
    page(LoginTemplate { error: None })
}

#[derive(Deserialize)]
struct LoginForm {
    #[serde(rename = "emailEntered")]
    email: Option<String>,
    #[serde(rename = "passwordEntered")]
    password: Option<String>,
}

/// A failed login only fails this visitor's attempt. In Java it logged the
/// admin out for everyone.
async fn login(
    State(app): State<AppState>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> Response {
    let email_ok = form
        .email
        .as_deref()
        .is_some_and(|e| e.trim().eq_ignore_ascii_case(ADMIN_EMAIL));
    // Verify even for a wrong email, so both take the same time.
    let password_ok = app.admin.verify(form.password.as_deref().unwrap_or(""));
    if !(email_ok && password_ok) {
        return render(
            StatusCode::UNAUTHORIZED,
            LoginTemplate {
                error: Some("Incorrect email or password."),
            },
        );
    }
    // A new session id at login, so an id set before it is worthless.
    if session.cycle_id().await.is_err() || session.insert(ADMIN, true).await.is_err() {
        return internal();
    }
    see_other("/admin-dashboard.jsp")
}

async fn logout(session: Session) -> Response {
    let _ = session.flush().await;
    see_other("/login.jsp")
}

async fn admin_dashboard(State(app): State<AppState>, session: Session) -> Response {
    if !is_admin(&session).await {
        return see_other("/login.jsp");
    }
    match db::all_flights(&app.pool).await {
        Ok(flights) => page(AdminDashboardTemplate {
            email: ADMIN_EMAIL,
            flights: flights.into_iter().map(FlightView::from).collect(),
        }),
        Err(e) => {
            eprintln!("dashboard: {e}");
            internal()
        }
    }
}

async fn change_password_page(session: Session) -> Response {
    if !is_admin(&session).await {
        return see_other("/login.jsp");
    }
    page(ChangePasswordTemplate {
        changed: false,
        error: None,
    })
}

#[derive(Deserialize)]
struct NewPassword {
    #[serde(rename = "passwordEntered")]
    password: Option<String>,
}

/// Java echoed the new password back, and threw a NullPointerException
/// when the field was missing.
async fn change_password(
    State(app): State<AppState>,
    session: Session,
    Form(form): Form<NewPassword>,
) -> Response {
    if !is_admin(&session).await {
        return see_other("/login.jsp");
    }
    let password = form.password.unwrap_or_default();
    if password.chars().count() < 8 {
        return render(
            StatusCode::BAD_REQUEST,
            ChangePasswordTemplate {
                changed: false,
                error: Some("Use at least 8 characters."),
            },
        );
    }
    app.admin.set(&password);
    page(ChangePasswordTemplate {
        changed: true,
        error: None,
    })
}

/// Both were open to anyone in Java.
async fn add_flight(session: Session) -> Response {
    if !is_admin(&session).await {
        return see_other("/login.jsp");
    }
    page(ComingSoonTemplate {
        heading: "Add a new Flight",
    })
}

async fn show_bookings(session: Session) -> Response {
    if !is_admin(&session).await {
        return see_other("/login.jsp");
    }
    page(ComingSoonTemplate {
        heading: "Past Bookings",
    })
}

async fn static_or_404(uri: Uri) -> Response {
    match tomcat::static_file(uri.path()) {
        Some(file) => crate::into_response(file),
        None => render(
            StatusCode::NOT_FOUND,
            MessageTemplate {
                heading: "Not found",
                text: format!("There is no page at {}.", uri.path()),
                link: "/index.jsp",
                link_text: "Home",
            },
        ),
    }
}
