use actix_web::{Responder, get, web};

use serde_json::json;
/// Logs out the request to the application, with method, and path it took to get there
///
/// # Params
///
/// - method - One of the HTTP request mothds.
/// - path_source - the path you need to get to the function
///
/// # Returns
///
/// - Nothing, prints to terminal the method used and path it is going to
///
/// # Example
/// ```rust
/// // this is how a public but internal module would be used by an outside user (ex_crate needs to be changed)
/// use darkicewolf50_actix_setup::log_incoming;
/// let result = log_incoming("GET", "/");
/// // unit value and should only be printed to the terminal
/// assert_eq!(result, ())
/// ```
/// # Author (s)
///
/// - Brock <brock@darkicewolf50.dev>

pub fn log_incoming(method: &'static str, path_source: &str) {
    println!("{} request, path: {}", method, path_source);
}

/// A quick method to check if the server is alive and running
/// This also keeps out scrapers from getting useful data
///
/// # Params
///
/// - nothing - needs nothing to check health of server
///
/// # Returns
///
/// - Json response with a alive message
///
/// # Example
/// ```rust
/// use darkicewolf50_actix_setup::health_check;
/// use actix_web::{web, test, App};
/// use serde_json::json;
///
/// #[actix_web::test]
/// async fn test_hello() {
///     let app = test::init_service(App::new().service(health_check)).await;
///     let req = test::TestRequest::get().uri("/").to_request();
///     let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;
///
///     assert_eq!(resp, json!({
///         "body": {
///             "message": "Hello I am alive, this does nothing"
///         }
///     }));
/// }
/// ```
/// # Author (s)
///
/// - Brock <brock@darkicewolf50.dev>

#[get("/")]
pub async fn health_check() -> impl Responder {
    log_incoming("GET", "/");
    web::Json(json!({
    "body": {
            "message": "Hello World! I am alive, this does nothing"
        }
    }))
}