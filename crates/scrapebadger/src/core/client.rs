use std::sync::Arc;
use std::time::Duration;

use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::config::{Config, API_KEY_ENV, API_KEY_HEADER, DEFAULT_BASE_URL};
use super::error::{Error, Result};

/// Cheap-to-clone HTTP client shared by every platform namespace.
///
/// Internally reference-counted; cloning only bumps an `Arc`.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

struct Inner {
    http: reqwest::Client,
    config: Config,
}

impl Client {
    pub(crate) fn from_config(config: Config) -> Result<Self> {
        let http = reqwest::Client::builder()
            .user_agent(config.user_agent.clone())
            .timeout(config.timeout)
            .build()?;
        Ok(Self {
            inner: Arc::new(Inner { http, config }),
        })
    }

    /// The configuration this client was built with.
    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    /// Execute a request with auth, retry, and typed error handling, decoding
    /// the success body into `T`. Used by all generated endpoint code.
    ///
    /// `path` must already include the version prefix (`/v1/...` or
    /// `/api/v1/...`); it is concatenated onto the configured base URL.
    pub async fn send<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<T> {
        let url = format!(
            "{}{}",
            self.inner.config.base_url.trim_end_matches('/'),
            path
        );
        let max_retries = self.inner.config.max_retries;

        let mut attempt: u32 = 0;
        loop {
            let mut req = self
                .inner
                .http
                .request(method.clone(), &url)
                .header(API_KEY_HEADER, &self.inner.config.api_key);
            if !query.is_empty() {
                req = req.query(query);
            }
            if let Some(ref b) = body {
                req = req.json(b);
            }

            let result = req.send().await;

            let response = match result {
                Ok(resp) => resp,
                Err(err) => {
                    // Retry transient transport failures (timeouts, connection resets).
                    if attempt < max_retries && (err.is_timeout() || err.is_connect()) {
                        backoff(attempt).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(Error::Transport(err));
                }
            };

            let status = response.status();

            // Retry 502/503/504 with exponential backoff.
            if matches!(
                status,
                StatusCode::BAD_GATEWAY
                    | StatusCode::SERVICE_UNAVAILABLE
                    | StatusCode::GATEWAY_TIMEOUT
            ) && attempt < max_retries
            {
                backoff(attempt).await;
                attempt += 1;
                continue;
            }

            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.trim().parse::<u64>().ok());

            // Retry 429 (rate limited): wait the server-provided `Retry-After`
            // when present (capped), otherwise fall back to exponential backoff.
            if status == StatusCode::TOO_MANY_REQUESTS && attempt < max_retries {
                let secs = retry_after
                    .map(|s| s.min(MAX_RETRY_AFTER_SECS))
                    .unwrap_or_else(|| backoff_secs(attempt));
                tokio::time::sleep(Duration::from_secs(secs)).await;
                attempt += 1;
                continue;
            }

            let bytes = response.bytes().await?;

            if status.is_success() {
                // Tolerate empty bodies (e.g. 204) by treating them as JSON null.
                let slice: &[u8] = if bytes.is_empty() { b"null" } else { &bytes };
                return serde_json::from_slice::<T>(slice).map_err(|e| {
                    Error::Decode(format!(
                        "{e}; body: {}",
                        String::from_utf8_lossy(&bytes)
                            .chars()
                            .take(500)
                            .collect::<String>()
                    ))
                });
            }

            return Err(map_error(status, &bytes, retry_after));
        }
    }
}

/// Map a non-success response into a typed [`Error`].
fn map_error(status: StatusCode, bytes: &[u8], retry_after: Option<u64>) -> Error {
    let body: Value = serde_json::from_slice(bytes).unwrap_or(Value::Null);
    let message = extract_message(&body)
        .unwrap_or_else(|| String::from_utf8_lossy(bytes).chars().take(300).collect());

    match status {
        StatusCode::UNAUTHORIZED => Error::Unauthorized { message },
        StatusCode::PAYMENT_REQUIRED => Error::PaymentRequired { message },
        StatusCode::TOO_MANY_REQUESTS => Error::RateLimited {
            retry_after,
            message,
        },
        StatusCode::UNPROCESSABLE_ENTITY => Error::Validation {
            message,
            detail: body.get("detail").cloned().unwrap_or(Value::Null),
        },
        _ => Error::Api {
            status: status.as_u16(),
            message,
            body,
        },
    }
}

/// Pull a human-readable message out of the common error body shapes.
fn extract_message(body: &Value) -> Option<String> {
    for key in ["message", "error", "detail"] {
        match body.get(key) {
            Some(Value::String(s)) => return Some(s.clone()),
            Some(other @ Value::Array(_)) | Some(other @ Value::Object(_)) => {
                return Some(other.to_string())
            }
            _ => {}
        }
    }
    None
}

/// Upper bound on how long we'll honor a `Retry-After` before a 429 retry, so a
/// hostile/huge header value can't stall a request indefinitely.
const MAX_RETRY_AFTER_SECS: u64 = 60;

/// Exponential backoff schedule: 1s, 2s, 4s, 8s, … capped at 30s.
fn backoff_secs(attempt: u32) -> u64 {
    1u64.checked_shl(attempt).unwrap_or(u64::MAX).min(30)
}

/// Sleep for the exponential backoff delay for `attempt`.
async fn backoff(attempt: u32) {
    tokio::time::sleep(Duration::from_secs(backoff_secs(attempt))).await;
}

/// Resolve an API key from an explicit value or the environment.
pub(crate) fn resolve_api_key(explicit: Option<String>) -> Result<String> {
    explicit
        .filter(|k| !k.is_empty())
        .or_else(|| std::env::var(API_KEY_ENV).ok().filter(|k| !k.is_empty()))
        .ok_or(Error::MissingApiKey)
}

#[allow(dead_code)]
pub(crate) const _DEFAULT_BASE_URL: &str = DEFAULT_BASE_URL;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_schedule_is_capped() {
        assert_eq!(backoff_secs(0), 1);
        assert_eq!(backoff_secs(3), 8);
        assert_eq!(backoff_secs(5), 30); // 32 -> capped
        assert_eq!(backoff_secs(100), 30); // shift overflow -> capped
    }

    #[test]
    fn resolve_api_key_prefers_explicit() {
        assert_eq!(resolve_api_key(Some("k".into())).unwrap(), "k");
    }
}
