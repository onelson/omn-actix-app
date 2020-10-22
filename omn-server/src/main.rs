use actix_web::dev::HttpResponseBuilder;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Serialize;

/// This type is shared with handlers by way of the `Data` extractor.
/// <https://docs.rs/actix-web/3.1.0/actix_web/web/struct.Data.html>
///
/// It's common to need to have some shared config info available, and this is
/// not the only way to do it but this is *a way* to go about it.
///
/// Often methods get hung off this type to help hand off REST clients or
/// db connections configured based on the fields in this struct.
#[derive(Clone, Debug, Serialize)]
struct Settings {
    host: String,
    port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            host: String::from("0.0.0.0"),
            port: 7878,
        }
    }
}

impl Settings {
    /// Typically, we'll try to source the service config data exclusively from
    /// environment vars. Relying on env vars is made nice for local dev thanks
    /// to `dotenv` (activated in `main()`), but also dovetails nicely with
    /// config maps in k8s since they can be injected into pods as a bunch of env
    /// vars.
    ///
    /// There's probably a crate to help do this but we usually just do it by
    /// hand.
    pub fn from_env() -> Settings {
        let mut settings = Settings::default();
        if let Ok(host) = std::env::var("HOST") {
            settings.host = host;
        }
        if let Ok(port) = std::env::var("PORT").map(|s| s.parse().expect("PORT")) {
            settings.port = port;
        }
        settings
    }
}

/// A custom error type for our service. We use this to gather up all the
/// potential failure modes our code might encounter so they can all be unified
/// under a single concrete type.
#[derive(Debug, thiserror::Error)]
enum OmnError {
    // This error attribute is used to generate an implementation of the
    // `Display` trait, which is required to be compatible with actix-web's
    // `ResponseError` trait.
    #[error("Some kind of DB problem.")]
    Database,
    #[error("Very Unlucky!")]
    Unlucky,
}

/// This trait can be customized for the app's error type.
///
/// For example, you might match on `self` to set a special status code or body
/// for particular variants.
///
/// - <https://actix.rs/docs/errors/>
/// - <https://mattgathu.github.io/2020/04/16/actix-web-error-handling.html>
impl ResponseError for OmnError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unlucky => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
        }
    }

    fn error_response(&self) -> HttpResponse {
        // Build a response from the status code, but just dump the `Display`
        // for the variant in the body.
        HttpResponseBuilder::new(self.status_code()).body(format!("{}", self))
    }
}

/// Convenience type alias to encourage the use of *our error type*.
type Result<T> = std::result::Result<T, OmnError>;

async fn info(settings: web::Data<Settings>) -> Result<HttpResponse> {
    // Since `Settings` derives `serde::Serialize` it can be converted to json
    // automatically.
    //
    // To write it to the response body, we do have to do a funky little
    // dance to reach inside the `Data`/`Arc` wrappers that allow this data to
    // be shared around the app.
    // The `into_inner()` gets the `Arc` out of the `Data`. We then use `*` to
    // *deref* the `Arc` so we can get at the underlying `Settings`, but we still
    // need to `&` it so we don't try to *move* the data.
    // The `Arc` permits us to have *many readers*, but we still need to be sure
    // we don't *move*.
    Ok(HttpResponse::Ok().json(&*settings.into_inner()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let settings = Settings::from_env();

    // Wrapping our settings in a `Data` makes it easy to share with the workers
    // in the `HttpServer`. Each invocation of the factory can clone a copy.
    //
    // <https://actix.rs/docs/application/>
    let data = web::Data::new(settings.clone());

    HttpServer::new(move || {
        // This closure (app factory) may run several times as the server spawns
        // workers.
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(info))
    })
    .bind(format!("{}:{}", &settings.host, &settings.port))?
    .run()
    .await
}
