use std::time::Duration;

/// Default API base URL. All endpoint paths already carry their own version
/// prefix (`/v1/...` for most platforms, `/api/v1/...` for Google), so the
/// client simply concatenates `base_url + path`.
pub const DEFAULT_BASE_URL: &str = "https://scrapebadger.com";

/// Environment variable read when no API key is passed explicitly.
pub const API_KEY_ENV: &str = "SCRAPEBADGER_API_KEY";

/// Authentication header name (case-insensitive on the wire).
pub(crate) const API_KEY_HEADER: &str = "x-api-key";

/// Immutable client configuration.
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub max_retries: u32,
    pub timeout: Duration,
    pub user_agent: String,
}

impl Config {
    pub(crate) fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: DEFAULT_BASE_URL.to_string(),
            max_retries: 10,
            timeout: Duration::from_secs(300),
            user_agent: concat!("scrapebadger-rust/", env!("CARGO_PKG_VERSION")).to_string(),
        }
    }
}
