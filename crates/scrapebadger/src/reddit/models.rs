//! Typed Reddit response models.
//!
//! The Reddit OpenAPI spec does not type its 200 bodies, so these structs were
//! reverse-engineered from live API responses (see
//! `scripts/collect_reddit_samples.py`). The generated [`Reddit`](super::Reddit)
//! methods return these types instead of [`serde_json::Value`].
//!
//! Every struct is `#[serde(default)]` and uses `Option<T>` / `Vec<T>` fields
//! plus a `#[serde(flatten)] extra` catch-all, so missing, null, or newly-added
//! fields never break deserialization.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Reddit's `edited` field: `false` when never edited, otherwise the edit time
/// in unix seconds. Modeled untagged so both forms deserialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Edited {
    /// `false` (never edited) or, rarely, `true`.
    Flag(bool),
    /// Edit timestamp, unix seconds.
    Timestamp(f64),
}

impl Default for Edited {
    fn default() -> Self {
        Edited::Flag(false)
    }
}

impl Edited {
    /// The edit timestamp (unix seconds), if the item was edited.
    pub fn edited_at(&self) -> Option<f64> {
        match self {
            Edited::Timestamp(t) => Some(*t),
            Edited::Flag(_) => None,
        }
    }

    /// Whether the item has been edited.
    pub fn is_edited(&self) -> bool {
        matches!(self, Edited::Timestamp(_) | Edited::Flag(true))
    }
}

/// Cursor pagination block (`after`/`before` are Reddit fullnames).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Pagination {
    pub after: Option<String>,
    pub before: Option<String>,
    pub count: Option<i64>,
    pub limit: Option<i64>,
}

/// A Reddit post (link or self-post).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditPost {
    pub id: Option<String>,
    pub fullname: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub author_fullname: Option<String>,
    pub selftext: Option<String>,
    pub selftext_html: Option<String>,
    pub url: Option<String>,
    pub permalink: Option<String>,
    pub domain: Option<String>,
    pub subreddit: Option<String>,
    pub subreddit_id: Option<String>,
    pub subreddit_name_prefixed: Option<String>,
    pub subreddit_type: Option<String>,
    pub subreddit_subscribers: Option<i64>,
    pub score: Option<i64>,
    pub ups: Option<i64>,
    pub downs: Option<i64>,
    pub upvote_ratio: Option<f64>,
    pub num_comments: Option<i64>,
    pub num_crossposts: Option<i64>,
    pub total_awards_received: Option<i64>,
    pub gilded: Option<i64>,
    pub created_utc: Option<f64>,
    pub created_at: Option<String>,
    pub edited: Option<Edited>,
    pub is_self: Option<bool>,
    pub is_video: Option<bool>,
    pub is_gallery: Option<bool>,
    pub is_original_content: Option<bool>,
    pub is_nsfw: Option<bool>,
    pub is_spoiler: Option<bool>,
    pub is_stickied: Option<bool>,
    pub locked: Option<bool>,
    pub archived: Option<bool>,
    pub pinned: Option<bool>,
    pub distinguished: Option<String>,
    pub suggested_sort: Option<String>,
    pub link_flair_text: Option<String>,
    pub author_flair_text: Option<String>,
    /// Any fields not modeled above (media, awards, flair detail, …).
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A Reddit comment. `replies` nests the same type for threaded replies.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditComment {
    pub id: Option<String>,
    pub fullname: Option<String>,
    pub body: Option<String>,
    pub body_html: Option<String>,
    pub author: Option<String>,
    pub author_fullname: Option<String>,
    pub subreddit: Option<String>,
    pub subreddit_id: Option<String>,
    pub subreddit_name_prefixed: Option<String>,
    pub subreddit_type: Option<String>,
    pub post_id: Option<String>,
    pub parent_id: Option<String>,
    pub permalink: Option<String>,
    pub score: Option<i64>,
    pub ups: Option<i64>,
    pub downs: Option<i64>,
    pub controversiality: Option<i64>,
    pub total_awards_received: Option<i64>,
    pub gilded: Option<i64>,
    pub score_hidden: Option<bool>,
    pub depth: Option<i64>,
    pub created_utc: Option<f64>,
    pub created_at: Option<String>,
    pub edited: Option<Edited>,
    pub is_submitter: Option<bool>,
    pub is_stickied: Option<bool>,
    pub locked: Option<bool>,
    pub archived: Option<bool>,
    pub collapsed: Option<bool>,
    pub distinguished: Option<String>,
    pub author_flair_text: Option<String>,
    /// Nested replies (Reddit returns these inline).
    pub replies: Vec<RedditComment>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A subreddit / community.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditSubreddit {
    pub id: Option<String>,
    pub fullname: Option<String>,
    pub name: Option<String>,
    pub display_name_prefixed: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub public_description: Option<String>,
    pub public_description_html: Option<String>,
    pub url: Option<String>,
    pub subscribers: Option<i64>,
    pub created_utc: Option<f64>,
    pub created_at: Option<String>,
    pub subreddit_type: Option<String>,
    pub lang: Option<String>,
    pub is_nsfw: Option<bool>,
    pub over18: Option<bool>,
    pub quarantine: Option<bool>,
    pub wiki_enabled: Option<bool>,
    pub submission_type: Option<String>,
    pub comment_score_hide_mins: Option<i64>,
    pub community_icon: Option<String>,
    pub icon_img: Option<String>,
    pub banner_img: Option<String>,
    pub primary_color: Option<String>,
    pub key_color: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The small "user profile subreddit" embedded on a [`RedditUser`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditUserSubreddit {
    pub display_name: Option<String>,
    pub display_name_prefixed: Option<String>,
    pub title: Option<String>,
    pub public_description: Option<String>,
    pub subscribers: Option<i64>,
    pub url: Option<String>,
    pub subreddit_type: Option<String>,
    pub over_18: Option<bool>,
    pub icon_img: Option<String>,
    pub banner_img: Option<String>,
    pub community_icon: Option<String>,
    pub primary_color: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A Reddit user / account.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditUser {
    pub id: Option<String>,
    pub name: Option<String>,
    pub display_name_prefixed: Option<String>,
    pub link_karma: Option<i64>,
    pub comment_karma: Option<i64>,
    pub awardee_karma: Option<i64>,
    pub awarder_karma: Option<i64>,
    pub total_karma: Option<i64>,
    pub created_utc: Option<f64>,
    pub created_at: Option<String>,
    pub is_gold: Option<bool>,
    pub is_employee: Option<bool>,
    pub is_mod: Option<bool>,
    pub is_friend: Option<bool>,
    pub verified: Option<bool>,
    pub has_verified_email: Option<bool>,
    pub hide_from_robots: Option<bool>,
    pub accept_followers: Option<bool>,
    pub icon_img: Option<String>,
    pub snoovatar_img: Option<String>,
    pub subreddit: Option<RedditUserSubreddit>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A subreddit a user moderates (distinct, slimmer shape from [`RedditSubreddit`]).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ModeratedSubreddit {
    pub fullname: Option<String>,
    pub sr: Option<String>,
    pub name: Option<String>,
    pub display_name_prefixed: Option<String>,
    pub sr_display_name_prefixed: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub subscribers: Option<i64>,
    pub subreddit_type: Option<String>,
    pub over_18: Option<bool>,
    pub community_icon: Option<String>,
    pub icon_img: Option<String>,
    pub banner_img: Option<String>,
    pub primary_color: Option<String>,
    pub key_color: Option<String>,
    pub created_utc: Option<f64>,
    pub created_at: Option<String>,
    pub mod_permissions: Vec<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A subreddit rule.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditRule {
    pub priority: Option<i64>,
    pub short_name: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub violation_reason: Option<String>,
    pub created_utc: Option<f64>,
    pub kind: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A trophy on a user's profile.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RedditTrophy {
    pub id: Option<String>,
    pub award_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub icon_40: Option<String>,
    pub icon_70: Option<String>,
    pub url: Option<String>,
    pub granted_at: Option<f64>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A single wiki page's content and revision metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WikiPage {
    pub title: Option<String>,
    pub content_md: Option<String>,
    pub content_html: Option<String>,
    pub revision_by: Option<String>,
    pub revision_date: Option<f64>,
    pub revision_id: Option<String>,
    pub may_revise: Option<bool>,
    pub reason: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ===== Response envelopes (one per endpoint return type) =====

/// A page of posts. Reused across listing, subreddit, user, domain, and search
/// post endpoints; the optional context fields are populated where relevant.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PostsResponse {
    pub posts: Vec<RedditPost>,
    pub pagination: Option<Pagination>,
    pub sort: Option<String>,
    pub subreddit: Option<String>,
    pub username: Option<String>,
    pub domain: Option<String>,
    pub query: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A single post.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PostResponse {
    pub post: Option<RedditPost>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A post plus its comment thread.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PostCommentsResponse {
    pub post: Option<RedditPost>,
    pub comments: Vec<RedditComment>,
    pub pagination: Option<Pagination>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The original post plus any duplicate crossposts.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PostDuplicatesResponse {
    pub original: Option<RedditPost>,
    pub duplicates: Vec<RedditPost>,
    pub pagination: Option<Pagination>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A single subreddit.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SubredditResponse {
    pub subreddit: Option<RedditSubreddit>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A page of subreddits (new / popular / search).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SubredditsResponse {
    pub subreddits: Vec<RedditSubreddit>,
    pub pagination: Option<Pagination>,
    pub query: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A subreddit's rules plus the site-wide rules.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SubredditRulesResponse {
    pub rules: Vec<RedditRule>,
    pub site_rules: Vec<String>,
    pub subreddit: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A single user profile.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UserResponse {
    pub user: Option<RedditUser>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A page of users (search).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UsersResponse {
    pub users: Vec<RedditUser>,
    pub pagination: Option<Pagination>,
    pub query: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A user's comments.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UserCommentsResponse {
    pub comments: Vec<RedditComment>,
    pub pagination: Option<Pagination>,
    pub username: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The subreddits a user moderates.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UserModeratedResponse {
    pub subreddits: Vec<ModeratedSubreddit>,
    pub username: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A user's trophies.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UserTrophiesResponse {
    pub trophies: Vec<RedditTrophy>,
    pub username: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// The list of wiki page names for a subreddit.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WikiPagesResponse {
    pub pages: Vec<String>,
    pub subreddit: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A single wiki page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WikiPageResponse {
    pub page: Option<WikiPage>,
    pub subreddit: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edited_handles_both_forms() {
        // `false` -> Flag, not edited.
        let e: Edited = serde_json::from_str("false").unwrap();
        assert!(!e.is_edited());
        assert_eq!(e.edited_at(), None);
        // A timestamp -> edited.
        let e: Edited = serde_json::from_str("1780898223.0").unwrap();
        assert!(e.is_edited());
        assert_eq!(e.edited_at(), Some(1780898223.0));
    }

    #[test]
    fn post_tolerates_edited_timestamp_and_unknown_fields() {
        // `edited` as a timestamp (Reddit's mixed type) and a field we don't
        // model must both round-trip without error.
        let json = r#"{
            "id": "abc",
            "title": "hi",
            "edited": 1780898223.0,
            "some_future_field": {"nested": true}
        }"#;
        let p: RedditPost = serde_json::from_str(json).unwrap();
        assert_eq!(p.id.as_deref(), Some("abc"));
        assert!(p.edited.unwrap().is_edited());
        assert!(p.extra.contains_key("some_future_field"));
    }

    #[test]
    fn comment_replies_nest() {
        let json = r#"{
            "id": "c1",
            "body": "parent",
            "replies": [{"id": "c2", "body": "child", "edited": false}]
        }"#;
        let c: RedditComment = serde_json::from_str(json).unwrap();
        assert_eq!(c.replies.len(), 1);
        assert_eq!(c.replies[0].body.as_deref(), Some("child"));
    }

    #[test]
    fn missing_and_null_fields_default() {
        // Empty object: every field absent -> defaults, no error.
        let p: PostsResponse = serde_json::from_str("{}").unwrap();
        assert!(p.posts.is_empty());
        // Explicit nulls tolerated.
        let s: SubredditResponse = serde_json::from_str(r#"{"subreddit": null}"#).unwrap();
        assert!(s.subreddit.is_none());
    }
}
