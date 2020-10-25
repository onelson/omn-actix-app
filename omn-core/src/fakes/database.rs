use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// Fake databases have problems too.
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Unable to connect to database")]
    ConnectionFailure,
    #[error("Invalid db url: `{0}`")]
    BadUrl(String),
    #[error("Failed to execute query.")]
    QueryFailure,
}

/// Pretend DB lib Result type
pub type Result<T> = std::result::Result<T, DbError>;

/// Pretend this is a real db connection.
pub struct DbConnection;
/// Fake url type, used to pretend to validate raw strings.
pub struct DbUrl;

// An implementation for this trait for a type is what allows us to call
// `.parse()` on a string and get whatever the type is back.
impl FromStr for DbUrl {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with("db://") {
            Ok(DbUrl)
        } else {
            Err(DbError::BadUrl(s.to_string()))
        }
    }
}

fn check_availablility() -> bool {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        % 3
        == 0
}

pub fn get_connection(db_url: &str) -> Result<DbConnection> {
    // Leverage the `FromStr` impl here to "validate" our connection string.
    // The `?` means this function "bails" with an early return of
    // `Err(DbError::BadUrl(...))` in the event the incoming string is invalid.
    //
    // I like to think of `?` as doing the same thing for error handling as
    // async/await did for promises in js. This is to say, it focuses the flow
    // of the function body on the *happy path*. With `?` the errors
    // *propagate up* to the call site, thus diminishing the noise in here.
    //
    // The main thing to know about `?` is it's going to inject some code that
    // checks to see if the `Result` it's used on is Ok or Err, and when it's
    // Err it will `return`. This means a couple things that can be confusing
    // for newcomers to the lang.
    //
    // First, the function signature needs to be a `Result` of some kind.
    //
    // Second, the Error types for the thing the `?` is used on and the function
    // signature need to be "compatible."
    //
    // What do I mean by "compatible?" Well, the boilerplate code injected by
    // using `?` does a little more than just branch based on `Ok`/`Err`.
    // When there's an `Err`, it'll (short story version) call `.into()` on it,
    // which means if there's are `From` or `Into` trait implementations defined
    // to convert between the Error type being returned and the one named in the
    // function signature, that conversion will happen automatically.
    //
    // Part the big *value add* of crates like `thiserror` is they provide
    // low-friction ways to codegen those `From` implementations to facilitate
    // this idiom.
    //
    // > For a *very long, deep dive* into all this, check out Andrew Gallant's
    // > blog post on rust error handling:
    // > <https://blog.burntsushi.net/rust-error-handling/>
    //
    // In this specific case, no conversion is actually needed since this
    // `.parse()` call *also returns* a `DbError` (same as the function
    // signature), but if it returned a different Error type, this code would
    // still work as written so long as we ensure there's a `From` for that other
    // type to convert it into a `DbError`.

    let _parsed_db_url: DbUrl = db_url.parse()?;

    // Maybe the database is unreliable and flakes out regularly
    if check_availablility() {
        Err(DbError::ConnectionFailure)
    } else {
        Ok(DbConnection)
    }
}

/// Using a generic type in the signature here so the call site can hint to the
/// db lib how to unpack the results.
///
/// In practice, this `T` would probably have some sort of trait bounds using a
/// trait supplied by the lib so it can know how to convert from SQL types
/// into whatever `T` is, but we're just pretending here.
///
/// For this fake, we're going to demand that T has an implementation for
/// `Default` and we'll just return whatever the default is, I guess.
pub fn run_query<T: Default>(_conn: &DbConnection, _sql: &str) -> Result<T> {
    Ok(T::default())
}
