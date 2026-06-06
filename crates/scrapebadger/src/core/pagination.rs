//! Generic pagination helpers.
//!
//! ScrapeBadger paginates differently per platform (Twitter/Google use opaque
//! cursors, Amazon/Vinted use page numbers, Reddit uses `after` fullnames), so
//! there is no single universal paginator. These building blocks let callers
//! turn any "fetch one page" closure into a flat [`Stream`] of items.

use std::future::Future;

use futures_core::Stream;

use super::error::Result;

/// Turn a cursor-based fetch closure into a stream of individual items.
///
/// `fetch` receives the current cursor (`None` on the first call) and returns a
/// page of items plus the next cursor; iteration stops when it yields `None`.
///
/// ```no_run
/// # use scrapebadger::core::pagination::cursor_stream;
/// # async fn demo(client: scrapebadger::Twitter) -> scrapebadger::Result<()> {
/// use futures_util::StreamExt;
/// let stream = cursor_stream(None::<String>, |cursor| {
///     let client = client.clone();
///     async move {
///         let page = client
///             .advanced_search_tweets(scrapebadger::twitter::AdvancedSearchTweetsParams {
///                 query: Some("rust".into()),
///                 cursor,
///                 ..Default::default()
///             })
///             .await?;
///         Ok((page.data.unwrap_or_default(), page.next_cursor))
///     }
/// });
/// futures_util::pin_mut!(stream);
/// while let Some(tweet) = stream.next().await {
///     let _tweet = tweet?;
/// }
/// # Ok(()) }
/// ```
pub fn cursor_stream<T, C, F, Fut>(initial: Option<C>, fetch: F) -> impl Stream<Item = Result<T>>
where
    F: Fn(Option<C>) -> Fut,
    Fut: Future<Output = Result<(Vec<T>, Option<C>)>>,
{
    async_stream::try_stream! {
        let mut cursor = initial;
        loop {
            let (items, next) = fetch(cursor).await?;
            for item in items {
                yield item;
            }
            match next {
                Some(c) => cursor = Some(c),
                None => break,
            }
        }
    }
}

/// Turn a page-number fetch closure into a stream of individual items.
///
/// `fetch` receives the 1-based page number and returns that page's items;
/// iteration stops on the first empty page.
pub fn page_stream<T, F, Fut>(start_page: u32, fetch: F) -> impl Stream<Item = Result<T>>
where
    F: Fn(u32) -> Fut,
    Fut: Future<Output = Result<Vec<T>>>,
{
    async_stream::try_stream! {
        let mut page = start_page.max(1);
        loop {
            let items = fetch(page).await?;
            if items.is_empty() {
                break;
            }
            for item in items {
                yield item;
            }
            page += 1;
        }
    }
}
