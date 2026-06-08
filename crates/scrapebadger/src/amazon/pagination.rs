//! Pagination convenience adapters for Amazon (page-number based).
//!
//! Amazon listings page with a 1-based `page` query parameter. The `*_stream`
//! adapters wrap the generated methods with
//! [`page_stream`](crate::core::pagination::page_stream), which stops at the
//! first empty page.
//!
//! Hand-written (not regenerated), so safe to extend.

use futures_core::Stream;

use crate::core::pagination::page_stream;
use crate::core::Result;

use super::generated::*;
use super::Amazon;

macro_rules! page_stream_no_path {
    ($(#[$m:meta])* $stream_fn:ident => $call_fn:ident, $params:ty, $field:ident, $item:ty) => {
        $(#[$m])*
        pub fn $stream_fn(&self, params: $params) -> impl Stream<Item = Result<$item>> {
            let this = self.clone();
            page_stream(1, move |page| {
                let this = this.clone();
                let mut params = params.clone();
                async move {
                    params.page = Some(page as i64);
                    Ok(this.$call_fn(params).await?.$field)
                }
            })
        }
    };
}

macro_rules! page_stream_path {
    ($(#[$m:meta])* $stream_fn:ident => $call_fn:ident, $arg:ident, $params:ty, $field:ident, $item:ty) => {
        $(#[$m])*
        pub fn $stream_fn(
            &self,
            $arg: impl AsRef<str>,
            params: $params,
        ) -> impl Stream<Item = Result<$item>> {
            let this = self.clone();
            let arg = $arg.as_ref().to_string();
            page_stream(1, move |page| {
                let this = this.clone();
                let arg = arg.clone();
                let mut params = params.clone();
                async move {
                    params.page = Some(page as i64);
                    Ok(this.$call_fn(&arg, params).await?.$field)
                }
            })
        }
    };
}

impl Amazon {
    page_stream_no_path! {
        /// Stream every product matching a search across all pages.
        search_products_stream => search_products, SearchProductsParams, results, SearchResult
    }
    page_stream_path! {
        /// Stream a product's reviews across all pages.
        get_reviews_stream => get_reviews, asin, GetReviewsParams, reviews, Review
    }
    page_stream_path! {
        /// Stream a seller's products across all pages.
        get_seller_products_stream => get_seller_products, seller_id, GetSellerProductsParams, results, SearchResult
    }
}
