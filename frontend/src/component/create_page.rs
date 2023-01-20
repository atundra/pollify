use std::ops::Not;

use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, use_timeout, UseAsyncOptions};
use yew_router::prelude::use_navigator;

use crate::async_data::{AsyncData, ToAsyncData};
use crate::codegen::helloworld::greeter_client::Greeter;
use crate::codegen::helloworld::{HelloReply, HelloRequest};
use crate::codegen::poll_service::poll_service_client::PollService;
use crate::codegen::poll_service::{CreatePollRequest, PollKind, VoteOption};
use crate::component::create_poll_form::CreatePollForm;
use crate::router::Route;

use super::create_poll_form::FormData;

static HOST: &str = "http://localhost:50051";

#[function_component(Create)]
pub fn create() -> Html {
    let s = use_async(async move {
        Greeter::new(HOST.to_string())
            .say_hello(HelloRequest {
                name: "world".to_string(),
            })
            .await
            .map_err(|_| "Failed to run request")
    });

    let onclick = {
        let s = s.clone();

        Callback::from(move |_| {
            s.run();
        })
    };

    let async_data = s.to_async_data();
    let content = match async_data {
        AsyncData::Idle => "Idle",
        AsyncData::Loading => "Loading",
        AsyncData::Loaded(HelloReply { ref message }) => message,
        AsyncData::Failed(_) => "Failed",
    };

    let poll_service = use_memo(|_| PollService::new(HOST.to_string()), ());

    let poll_kinds = {
        let poll_service = poll_service.clone();
        use_async_with_options(
            async move {
                poll_service
                    .poll_kinds(())
                    .await
                    .map_err(|_| "Failed to load poll kinds".to_string())
                    .map(|response| response.kinds)
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    let kinds_async_data = poll_kinds.to_async_data();

    let error_state = use_state_eq(|| None::<String>);

    let hide_alert_timeout = {
        let error_state = error_state.clone();
        use_timeout(move || error_state.set(None), 5000)
    };

    let navigator = use_navigator().unwrap();

    let on_create_poll = {
        let error_state = error_state.clone();
        Callback::from(move |form_data: FormData| {
            let navigator = navigator.clone();
            let hide_alert_timeout = hide_alert_timeout.clone();
            let error_state = error_state.clone();
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
                    .map(|response| response.id);

                match response {
                    Ok(id) => navigator.push(&Route::PollPage { id }),
                    Err(text) => {
                        error_state.set(Some(text));
                        hide_alert_timeout.reset();
                    }
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "Create" }</h1>
            <button {onclick}>{ "Run request" }</button>
            <h2>{"Result"}</h2>
            <pre>{content}</pre>
            <CreatePollForm
                poll_kinds={kinds_async_data}
                on_create={on_create_poll}
            />
            if let Some(text) = (*error_state).clone() {
                <div class="toast toast-top">
                    <div class="alert alert-error">
                        <div>
                            <span>{text}</span>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
