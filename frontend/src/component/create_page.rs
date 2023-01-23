use super::create_poll_form::FormData;
use crate::codegen::poll_service::{CreatePollRequest, NewVoteOption, PollKind};
use crate::component::create_poll_form::CreatePollForm;
use crate::hooks::use_poll_kinds::use_poll_kinds;
use crate::hooks::use_poll_service::use_poll_service;
use crate::hooks::use_toast_on_async_data_error::use_toast_on_async_data_error;
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

    use_toast_on_async_data_error(kinds_async_data.clone());

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
                            .map(|option| NewVoteOption {
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
