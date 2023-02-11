use super::use_poll_service::use_poll_service;
use crate::async_data::{AsyncData, ToAsyncData};
use crate::codegen::poll_service::{PollResultItem, PollResultRequest};
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[hook]
pub fn use_poll_result(poll_id: String) -> AsyncData<Vec<PollResultItem>, String> {
    let poll_service = use_poll_service();
    let handle = use_async_with_options(
        async move {
            poll_service
                .poll_result(PollResultRequest { poll_id })
                .await
                .map_err(|_| "Failed to load poll kinds".to_string())
                .map(|response| response.items)
        },
        UseAsyncOptions::enable_auto(),
    );
    handle.to_async_data()
}
