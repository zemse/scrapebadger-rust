//! Real-time Twitter Streams: WebSocket delivery and webhook verification.
//!
//! Stream Monitors and Filter Rules (created via the generated [`super::Twitter`]
//! methods) deliver detected tweets over a single WebSocket connection and/or
//! via signed HTTP webhooks.
//!
//! See <https://docs.scrapebadger.com/twitter-streams/overview>.

use futures_core::Stream;
use futures_util::StreamExt;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tokio_tungstenite::tungstenite::Message;

use crate::core::{Error, Result};

use super::Twitter;

/// A `tweet.detected` event delivered over WebSocket or webhook.
///
/// Unknown / future fields are preserved in [`TweetEvent::extra`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TweetEvent {
    /// Event type, e.g. `"tweet.detected"`.
    #[serde(default)]
    pub event: Option<String>,
    #[serde(default)]
    pub monitor_id: Option<String>,
    #[serde(default)]
    pub monitor_name: Option<String>,
    #[serde(default)]
    pub rule_id: Option<String>,
    #[serde(default)]
    pub tweet_id: Option<String>,
    #[serde(default)]
    pub author_username: Option<String>,
    #[serde(default)]
    pub tweet_text: Option<String>,
    #[serde(default)]
    pub tweet_url: Option<String>,
    #[serde(default)]
    pub tweet_published_at: Option<String>,
    #[serde(default)]
    pub detected_at: Option<String>,
    #[serde(default)]
    pub latency_ms: Option<i64>,
    /// Full tweet object (present on webhook deliveries).
    #[serde(default)]
    pub tweet_data: Option<serde_json::Value>,
    /// Any additional fields not modeled above.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Twitter {
    /// Open a WebSocket connection and stream tweet events from every active
    /// monitor and filter rule on this API key.
    ///
    /// The returned stream yields one [`TweetEvent`] per detected tweet. The
    /// server may close idle connections — implement reconnection with backoff
    /// for long-lived consumers (per ScrapeBadger's guidance).
    ///
    /// ```no_run
    /// # async fn demo(twitter: scrapebadger::Twitter) -> scrapebadger::Result<()> {
    /// use futures_util::StreamExt;
    /// let mut events = Box::pin(twitter.stream_events().await?);
    /// while let Some(event) = events.next().await {
    ///     let event = event?;
    ///     println!("@{:?}: {:?}", event.author_username, event.tweet_url);
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn stream_events(&self) -> Result<impl Stream<Item = Result<TweetEvent>>> {
        let cfg = self.client().config();
        let ws_url = websocket_url(&cfg.base_url, &cfg.api_key);

        let (ws, _resp) = tokio_tungstenite::connect_async(&ws_url)
            .await
            .map_err(|e| Error::WebSocket(e.to_string()))?;

        Ok(async_stream::try_stream! {
            let mut ws = ws;
            while let Some(msg) = ws.next().await {
                let msg = msg.map_err(|e| Error::WebSocket(e.to_string()))?;
                match msg {
                    Message::Text(text) => {
                        let event: TweetEvent = serde_json::from_str(&text)
                            .map_err(|e| Error::Decode(e.to_string()))?;
                        yield event;
                    }
                    Message::Binary(bytes) => {
                        let event: TweetEvent = serde_json::from_slice(&bytes)
                            .map_err(|e| Error::Decode(e.to_string()))?;
                        yield event;
                    }
                    Message::Close(_) => break,
                    // Ping/Pong/Frame: keep-alive, ignore.
                    _ => {}
                }
            }
        })
    }
}

/// Build the WebSocket URL from the configured base URL and API key.
fn websocket_url(base_url: &str, api_key: &str) -> String {
    let base = base_url.trim_end_matches('/');
    let base = if let Some(rest) = base.strip_prefix("https://") {
        format!("wss://{rest}")
    } else if let Some(rest) = base.strip_prefix("http://") {
        format!("ws://{rest}")
    } else {
        base.to_string()
    };
    format!("{base}/v1/twitter/stream?api_key={}", encode_query(api_key))
}

/// Minimal percent-encoding for the API key in the query string.
fn encode_query(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Verify a ScrapeBadger webhook signature.
///
/// Computes `sha256=<hex HMAC-SHA256(secret, body)>` and compares it to the
/// `X-Signature-256` header value in constant time.
///
/// ```
/// # use scrapebadger::twitter::stream::verify_webhook_signature;
/// let body = br#"{"event":"tweet.detected"}"#;
/// // Header value as received from the `X-Signature-256` request header.
/// let header = "sha256=invalid";
/// assert!(!verify_webhook_signature("my-secret", body, header));
/// ```
pub fn verify_webhook_signature(secret: &str, body: &[u8], signature_header: &str) -> bool {
    let expected = sign_webhook(secret, body);
    constant_time_eq(expected.as_bytes(), signature_header.as_bytes())
}

/// Compute the `sha256=...` signature for a webhook body (useful for testing).
pub fn sign_webhook(secret: &str, body: &[u8]) -> String {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC accepts any key length");
    mac.update(body);
    let digest = mac.finalize().into_bytes();
    format!("sha256={}", hex::encode(digest))
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webhook_signature_roundtrip() {
        let secret = "my-secret-key";
        let body = br#"{"event":"tweet.detected","tweet_id":"1234567890"}"#;
        let sig = sign_webhook(secret, body);
        assert!(sig.starts_with("sha256="));
        assert!(verify_webhook_signature(secret, body, &sig));
        assert!(!verify_webhook_signature("wrong-secret", body, &sig));
        assert!(!verify_webhook_signature(secret, b"tampered", &sig));
    }

    #[test]
    fn websocket_url_scheme_swap() {
        let url = websocket_url("https://scrapebadger.com", "sb_live_abc123");
        assert_eq!(
            url,
            "wss://scrapebadger.com/v1/twitter/stream?api_key=sb_live_abc123"
        );
        let url = websocket_url("http://localhost:8080/", "k e y");
        assert_eq!(url, "ws://localhost:8080/v1/twitter/stream?api_key=k%20e%20y");
    }
}
