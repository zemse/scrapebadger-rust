use serde_json::Value;

/// Convenient result alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Every error the SDK can produce.
///
/// API failures are mapped to typed variants by HTTP status where the meaning
/// is well-defined (`401`, `402`, `422`, `429`); anything else lands in
/// [`Error::Api`] with the raw status and decoded JSON body so callers never
/// lose information.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// No API key was supplied and `SCRAPEBADGER_API_KEY` was not set.
    #[error("missing API key: pass one to ScrapeBadger::new or set SCRAPEBADGER_API_KEY")]
    MissingApiKey,

    /// The configured base URL could not be used to build a request URL.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    /// Transport-level failure (DNS, TLS, connection, timeout).
    #[error("http transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// `401 Unauthorized` — the API key is missing or invalid.
    #[error("unauthorized (401): {message}")]
    Unauthorized { message: String },

    /// `402 Payment Required` — out of credits or no active subscription.
    #[error("payment required (402): {message}")]
    PaymentRequired { message: String },

    /// `429 Too Many Requests` — rate limit exceeded.
    #[error("rate limited (429){}", match retry_after { Some(s) => format!(", retry after {s}s"), None => String::new() })]
    RateLimited {
        /// Seconds to wait before retrying, if the server told us.
        retry_after: Option<u64>,
        message: String,
    },

    /// `422 Unprocessable Entity` — request validation failed.
    #[error("validation error (422): {message}")]
    Validation {
        message: String,
        /// The raw FastAPI `detail` array.
        detail: Value,
    },

    /// Any other non-success HTTP status.
    #[error("api error (status {status}): {message}")]
    Api {
        status: u16,
        message: String,
        /// Decoded JSON body (or `Value::Null` if the body was not JSON).
        body: Value,
    },

    /// The response body could not be decoded into the expected type.
    #[error("failed to decode response: {0}")]
    Decode(String),

    /// WebSocket streaming error (Twitter Streams).
    #[cfg(feature = "stream")]
    #[error("websocket error: {0}")]
    WebSocket(String),
}
