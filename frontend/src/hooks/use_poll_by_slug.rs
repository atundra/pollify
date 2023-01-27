use super::use_poll_service::use_poll_service;
use crate::async_data::{AsyncData, ToAsyncData};
use crate::codegen::poll_service::{GetPollBySlugRequest, GetPollBySlugResponse};
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

/// It returns an `AsyncData` that will load the poll with the given slug
///
/// Arguments:
///
/// * `slug`: The slug of the poll to load.
///
/// Returns:
///
/// An AsyncData<GetPollBySlugResponse, String>
#[hook]
pub fn use_poll_by_slug(slug: String) -> AsyncData<GetPollBySlugResponse, String> {
    let poll_service = use_poll_service();
    let handle = use_async_with_options(
        async move {
            let slug = slug.clone();

            poll_service
                .get_poll_by_slug(GetPollBySlugRequest { slug: slug.clone() })
                .await
                .map_err(move |_err| format!("Failed to load poll {slug}"))
        },
        UseAsyncOptions::enable_auto(),
    );
    handle.to_async_data()
}
