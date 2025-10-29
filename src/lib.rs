use actix_web::{HttpResponse, Responder, get, web};
use regex::Regex;
use serde_json::json;
use std::path::{Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

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
/// use std::path::Path;
/// use darkicewolf50_actix_setup::clean_file_user_path;
///
/// let result = clean_file_user_path("/database", "test".to_string(), "txt").unwrap();
///
/// assert_eq!(result, Path::new("/database/test.txt"))
/// ```
///
/// ```rust
/// use std::path::Path;
/// use darkicewolf50_actix_setup::clean_file_user_path;
/// let result = clean_file_user_path("/database", "test".to_string(), ".txt").unwrap();
///
/// assert_eq!(result, Path::new("/database/test.txt"))
/// ```
/// # Author (s)
///
/// - Brock <brock@darkicewolf50.dev>

pub fn clean_user_file_req(
    base_path: &str,
    user_file_request: String,
    file_extension: &str,
) -> Result<PathBuf, HttpResponse> {
    let normalized_file_req = user_file_request.nfc().collect::<String>();

    if normalized_file_req.is_empty() || normalized_file_req.len() > 255 {
        return Err(HttpResponse::BadRequest().body("invalid blog name"));
    }
    if normalized_file_req.chars().any(|c| c.is_control()) {
        return Err(HttpResponse::BadRequest().body("invalid blog name"));
    }

    let traversal_regex = Regex::new(r"(\.\.|/|\\)").unwrap();
    if traversal_regex.is_match(&normalized_file_req) {
        return Err(HttpResponse::BadRequest().body("invalid blog name"));
    }

    let allowed_char = Regex::new(r"^[A-Za-z0-9 _\-\(\)\[\]]+$").unwrap();
    if !allowed_char.is_match(&normalized_file_req) {
        return Err(HttpResponse::BadRequest().body("invalid blog name"));
    }

    let final_path = Path::new(base_path)
        .join(&normalized_file_req)
        .with_extension(file_extension.trim_start_matches('.'));

    Ok(final_path)
}
