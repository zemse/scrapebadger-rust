//! Pagination convenience adapters for Reddit.
//!
//! Reddit listings page with an `after` fullname cursor (returned in
//! `pagination.after`). The `*_stream` adapters here wrap the generated methods
//! with [`cursor_stream`](crate::core::pagination::cursor_stream) so you can
//! iterate every item across pages as a single flat [`Stream`].
//!
//! Hand-written (not regenerated), so safe to extend alongside the generated
//! endpoint methods.

use futures_core::Stream;

use crate::core::pagination::cursor_stream;
use crate::core::Result;

use super::generated::*;
use super::models::*;
use super::Reddit;

/// `after`-cursor adapter for a listing that takes no path argument.
macro_rules! after_stream_no_path {
    ($(#[$m:meta])* $stream_fn:ident => $call_fn:ident, $params:ty, $field:ident, $item:ty) => {
        $(#[$m])*
        pub fn $stream_fn(&self, params: $params) -> impl Stream<Item = Result<$item>> {
            let this = self.clone();
            cursor_stream(None::<String>, move |after| {
                let this = this.clone();
                let mut params = params.clone();
                async move {
                    params.after = after;
                    let page = this.$call_fn(params).await?;
                    let next = page.pagination.and_then(|p| p.after);
                    Ok((page.$field, next))
                }
            })
        }
    };
}

/// `after`-cursor adapter for a listing that takes one path argument.
macro_rules! after_stream_path {
    ($(#[$m:meta])* $stream_fn:ident => $call_fn:ident, $arg:ident, $params:ty, $field:ident, $item:ty) => {
        $(#[$m])*
        pub fn $stream_fn(
            &self,
            $arg: impl AsRef<str>,
            params: $params,
        ) -> impl Stream<Item = Result<$item>> {
            let this = self.clone();
            let arg = $arg.as_ref().to_string();
            cursor_stream(None::<String>, move |after| {
                let this = this.clone();
                let arg = arg.clone();
                let mut params = params.clone();
                async move {
                    params.after = after;
                    let page = this.$call_fn(&arg, params).await?;
                    let next = page.pagination.and_then(|p| p.after);
                    Ok((page.$field, next))
                }
            })
        }
    };
}

impl Reddit {
    after_stream_path! {
        /// Stream every post in a subreddit across all pages.
        get_subreddit_posts_stream => get_subreddit_posts, subreddit, GetSubredditPostsParams, posts, RedditPost
    }
    after_stream_path! {
        /// Stream a user's posts across all pages.
        get_user_posts_stream => get_user_posts, username, GetUserPostsParams, posts, RedditPost
    }
    after_stream_path! {
        /// Stream a user's comments across all pages.
        get_user_comments_stream => get_user_comments, username, GetUserCommentsParams, comments, RedditComment
    }
    after_stream_path! {
        /// Stream posts linking to a domain across all pages.
        get_domain_posts_stream => get_domain_posts, domain, GetDomainPostsParams, posts, RedditPost
    }

    after_stream_no_path! {
        /// Stream every post matching a search across all pages.
        search_posts_stream => search_posts, SearchPostsParams, posts, RedditPost
    }
    after_stream_no_path! {
        /// Stream every subreddit matching a search across all pages.
        search_subreddits_stream => search_subreddits, SearchSubredditsParams, subreddits, RedditSubreddit
    }
    after_stream_no_path! {
        /// Stream every user matching a search across all pages.
        search_users_stream => search_users, SearchUsersParams, users, RedditUser
    }
    after_stream_no_path! {
        /// Stream newest subreddits across all pages.
        get_new_subreddits_stream => get_new_subreddits, GetNewSubredditsParams, subreddits, RedditSubreddit
    }
    after_stream_no_path! {
        /// Stream popular subreddits across all pages.
        get_popular_subreddits_stream => get_popular_subreddits, GetPopularSubredditsParams, subreddits, RedditSubreddit
    }
    after_stream_no_path! {
        /// Stream trending posts across all pages.
        get_trending_posts_stream => get_trending_posts, GetTrendingPostsParams, posts, RedditPost
    }
}
