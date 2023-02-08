use super::use_poll_service::use_poll_service;
use crate::async_data::{AsyncData, ToAsyncData};
use crate::codegen::poll_service::{GetPollBySlugRequest, GetPollBySlugResponse};
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

pub struct UsePollBySlugHandle<T, E> {
    inner: AsyncData<T, E>,
    run: Rc<dyn Fn()>,
}

impl<T, E> UsePollBySlugHandle<T, E> {
    pub fn run(&self) {
        (self.run)();
    }
}

impl<T, E> Deref for UsePollBySlugHandle<T, E> {
    type Target = AsyncData<T, E>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, E> Clone for UsePollBySlugHandle<T, E>
where
    T: Clone,
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            run: self.run.clone(),
        }
    }
}

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
pub fn use_poll_by_slug(slug: String) -> UsePollBySlugHandle<GetPollBySlugResponse, String> {
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

    let run = {
        let handle = handle.clone();
        Rc::new(move || {
            handle.run();
        })
    };

    UsePollBySlugHandle {
        inner: handle.to_async_data(),
        run,
    }
}
