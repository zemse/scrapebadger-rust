//! Pagination convenience adapters for Vinted (page-number based).
//!
//! Vinted listings page with a 1-based `page` query parameter. The `*_stream`
//! adapters wrap the generated methods with
//! [`page_stream`](crate::core::pagination::page_stream), which stops at the
//! first empty page.
//!
//! Hand-written (not regenerated), so safe to extend.

use futures_core::Stream;

use crate::core::pagination::page_stream;
use crate::core::Result;

use super::generated::*;
use super::Vinted;

impl Vinted {
    /// Stream every item matching a search across all pages.
    pub fn search_items_stream(
        &self,
        params: SearchItemsParams,
    ) -> impl Stream<Item = Result<VintedItemSummary>> {
        let this = self.clone();
        page_stream(1, move |page| {
            let this = this.clone();
            let mut params = params.clone();
            async move {
                params.page = Some(page as i64);
                Ok(this.search_items(params).await?.items)
            }
        })
    }

    /// Stream a user's items across all pages.
    pub fn get_user_items_stream(
        &self,
        user_id: impl AsRef<str>,
        params: GetUserItemsParams,
    ) -> impl Stream<Item = Result<VintedItemSummary>> {
        let this = self.clone();
        let user_id = user_id.as_ref().to_string();
        page_stream(1, move |page| {
            let this = this.clone();
            let user_id = user_id.clone();
            let mut params = params.clone();
            async move {
                params.page = Some(page as i64);
                Ok(this.get_user_items(&user_id, params).await?.items)
            }
        })
    }
}
