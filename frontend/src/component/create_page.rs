use super::create_poll_form::FormData;
use crate::async_data::AsyncData;
use crate::codegen::poll_service::{CreatePollRequest, PollKind, VoteOption};
use crate::component::create_poll_form::CreatePollForm;
use crate::hooks::use_poll_kinds::use_poll_kinds;
use crate::hooks::use_poll_service::use_poll_service;
use crate::router::Route;
use crate::toast::use_toast;
use std::ops::Not;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(Create)]
pub fn create() -> Html {
    let poll_service = use_poll_service();

    let kinds_async_data = use_poll_kinds();

    let toast = use_toast().unwrap();

    {
        let kinds_async_data = kinds_async_data.clone();
        use_effect_with_deps(
            move |data| {
                if let AsyncData::Failed(error) = data {
                    toast.error(error.to_string());
                }
            },
            kinds_async_data,
        );
    };

    let toast = use_toast().unwrap();

    let navigator = use_navigator().unwrap();

    let on_create_poll = {
        Callback::from(move |form_data: FormData| {
            let toast = toast.clone();
            let navigator = navigator.clone();
            let poll_service = poll_service.clone();
            spawn_local(async move {
                let response = poll_service
                    .create_poll(CreatePollRequest {
                        title: form_data.name,
                        kind: Some(PollKind {
                            id: form_data
                                .voting_system
                                .map(|id| id.parse::<i32>().unwrap())
                                .unwrap(),
                        }),
                        slug: form_data.slug.is_empty().not().then_some(form_data.slug),
                        options: form_data
                            .options
                            .iter()
                            .map(|option| VoteOption {
                                title: option.title.clone(),
                                description: option
                                    .description
                                    .is_empty()
                                    .not()
                                    .then_some(option.description.clone()),
                            })
                            .collect(),
                    })
                    .await
                    .map_err(|_| "Error: Failed to create a poll, please contact us".to_string())
                    .map(|response| response.slug);

                match response {
                    Ok(slug) => navigator.push(&Route::PollPage { slug }),
                    Err(text) => toast.error(text),
                }
            });
        })
    };

    html! {
        <div class="flex justify-center items-center h-full">
            <div class="py-8 w-full">
                <CreatePollForm
                    poll_kinds={kinds_async_data}
                    on_create={on_create_poll}
                />
            </div>
        </div>
    }
}
