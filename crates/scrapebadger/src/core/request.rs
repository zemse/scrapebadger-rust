//! Small helper used by generated endpoint code to assemble query strings.

/// Accumulates query parameters, skipping any that are `None`.
///
/// Array values are serialized as a single comma-separated value, which is the
/// convention ScrapeBadger uses for its list-valued query parameters
/// (e.g. Twitter `ids`, `usernames`).
#[derive(Default)]
pub struct QueryParams(Vec<(String, String)>);

impl QueryParams {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Push an optional scalar value; no-op when `None`.
    pub fn opt<T: ToString>(&mut self, key: &str, value: Option<&T>) -> &mut Self {
        if let Some(v) = value {
            self.0.push((key.to_string(), v.to_string()));
        }
        self
    }

    /// Push an optional list value as a comma-separated string; no-op when
    /// `None` or empty.
    pub fn list<T: ToString>(&mut self, key: &str, value: Option<&Vec<T>>) -> &mut Self {
        if let Some(items) = value {
            if !items.is_empty() {
                let joined = items
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                self.0.push((key.to_string(), joined));
            }
        }
        self
    }

    pub fn into_pairs(self) -> Vec<(String, String)> {
        self.0
    }
}
