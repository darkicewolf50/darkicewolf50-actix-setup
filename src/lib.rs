use actix_web::{HttpResponse, Responder, get, web};
use regex::Regex;
use serde_json::json;
use std::path::{Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

#[cfg(any(all(feature = "debug", debug_assertions), feature = "full"))]
pub use swagger_docs::health_check_swagger;

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
/// If you want swagger/open api docs use the [`health_check_swagger`] version instead
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
/// use darkicewolf50_actix_setup::clean_user_file_req;
///
/// let result = clean_user_file_req("/database", "test", "txt").unwrap();
///
/// assert_eq!(result, Path::new("/database/test.txt"))
/// ```
///
/// ```rust
/// use std::path::Path;
/// use darkicewolf50_actix_setup::clean_user_file_req;
/// let result = clean_user_file_req("/database", "test", ".txt").unwrap();
///
/// assert_eq!(result, Path::new("/database/test.txt"))
/// ```
/// # Author (s)
///
/// - Brock <brock@darkicewolf50.dev>

pub fn clean_user_file_req(
    base_path: &str,
    user_file_request: &str,
    file_extension: &str,
) -> Result<PathBuf, HttpResponse> {
    let normalized_file_req = user_file_request.nfc().collect::<String>();

    if normalized_file_req.is_empty() || normalized_file_req.len() > 255 {
        return Err(HttpResponse::BadRequest().body("invalid file request"));
    }
    if normalized_file_req.chars().any(|c| c.is_control()) {
        return Err(HttpResponse::BadRequest().body("invalid file request"));
    }

    let traversal_regex = Regex::new(r"(\.\.|/|\\)").unwrap();
    if traversal_regex.is_match(&normalized_file_req) {
        return Err(HttpResponse::BadRequest().body("invalid file request"));
    }

    let allowed_char = Regex::new(r"^[A-Za-z0-9 _\-\(\)\[\]]+$").unwrap();
    if !allowed_char.is_match(&normalized_file_req) {
        return Err(HttpResponse::BadRequest().body("invalid file request"));
    }

    let final_path = Path::new(base_path)
        .join(&normalized_file_req)
        .with_extension(file_extension.trim_start_matches('.'));
    Ok(final_path)
}

// debug feature and debug build on OR ful feature on (always on)
#[cfg(any(all(feature = "debug", debug_assertions), feature = "full"))]
pub mod swagger_docs {
    use crate::log_incoming;
    use actix_web::{Responder, get, web};
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(Debug, Serialize, ToSchema)]
    struct HeathMessage {
        #[schema(example = "Hello World! I am alive, this does nothing")]
        message: String,
    }

    /// A quick method to check if the server is alive and running
    /// This also keeps out scrapers from getting useful data
    ///
    /// This version includes documentation for utopia's swagger ui and for Open Api docs
    /// But is exactly the same out as [`health_check`]
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
    /// use darkicewolf50_actix_setup::{ health_check_swagger, HeathMessage };
    /// use actix_web::{web, test, App};
    /// use serde_json::json;
    ///
    /// #[actix_web::test]
    /// async fn test_hello() {
    ///     let app = test::init_service(App::new().service(health_check_swagger)).await;
    ///     let req = test::TestRequest::get().uri("/").to_request();
    ///     let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    ///
    ///     assert_eq!(resp, HeathMessage {
    ///        message: "Hello World! I am alive, this does nothing".to_string(),
    ///     });
    /// }
    /// ```
    /// # Author (s)
    ///
    /// - Brock <brock@darkicewolf50.dev>

    #[utoipa::path(
        get,
        path = "/",
        responses(
            (status = 200, description = "Server is alive, will output the HeathMessage schema", body = [HeathMessage])
        )
    )]
    #[get("/")]
    pub async fn health_check_swagger() -> impl Responder {
        log_incoming("GET", "/");
        web::Json(HeathMessage {
            message: "Hello World! I am alive, this does nothing".to_string(),
        })
    }
}
