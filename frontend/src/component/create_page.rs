use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};

use crate::async_data::{AsyncData, ToAsyncData};
use crate::codegen::helloworld::greeter_client::Greeter;
use crate::codegen::helloworld::{HelloReply, HelloRequest};
use crate::codegen::poll_service::poll_service_client::PollService;
use crate::codegen::poll_service::{PollKind, PollKindsResponse};
use crate::component::create_poll_form::CreatePollForm;

static HOST: &str = "http://localhost:50051";

#[derive(Properties, PartialEq)]
pub struct PollKindProps {
    pub id: i32,
}

#[function_component(PollKindOption)]
fn poll_kind(PollKindProps { id }: &PollKindProps) -> Html {
    let name = match *id {
        0 => Some("First Past the Post"),
        1 => Some("Single Transferable Vote"),
        2 => Some("Additional Member System"),
        _ => None,
    };

    html! {
        <option key={*id} value={id.to_string()}>{name}</option>
    }
}

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

    let poll_kinds = use_async_with_options(
        async move {
            PollService::new(HOST.to_string())
                .poll_kinds(())
                .await
                .map_err(|_| "Failed to load poll kinds")
        },
        UseAsyncOptions::enable_auto(),
    );

    let kinds_async_data = poll_kinds.to_async_data();
    let kinds_select = match kinds_async_data {
        AsyncData::Loaded(PollKindsResponse { kinds }) => html! {
            <select class="select w-full select-bordered">
                <option disabled=true selected=true>{"Voting system"}</option>
                {kinds.into_iter().map(|PollKind { id }| {
                    html! { <PollKindOption id={id} /> }
                }).collect::<Html>()}
            </select>
        },
        _ => html! {
            <select class="select w-full select-bordered" disabled={true}>
                <option disabled=true selected=true>{"Voting system"}</option>
            </select>
        },
    };

    html! {
        <div>
            <h1>{ "Create" }</h1>
            <button {onclick}>{ "Run request" }</button>
            <h2>{"Result"}</h2>
            <pre>{content}</pre>
            <div class="space-y-4 mt-4 max-w-md m-auto">
                <div class="form-control">
                    <input type="text" placeholder="Poll name" class="input w-full input-bordered" />
                    <label class="label">
                        <span class="label-text">{"Publicly available name"}</span>
                    </label>
                </div>
                <div class="form-control">
                    {kinds_select}
                    <label class="label">
                        <span class="label-text">{"Publicly available name"}</span>
                    </label>
                </div>
                <div class="form-control">
                    <input type="text" placeholder="Slug (optional)" class="input w-full input-bordered" />
                    <label class="label">
                        <span class="label-text">{"The poll would be available at: "}<code>{"pollify.com/p/slug"}</code></span>
                    </label>
                </div>
                <button class="btn btn-primary btn-block">{"Create poll"}</button>
            </div>
            <CreatePollForm />
        </div>
    }
}
