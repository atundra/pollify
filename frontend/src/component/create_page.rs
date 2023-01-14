use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};

use crate::async_data::{AsyncData, ToAsyncData};
use crate::codegen::helloworld::greeter_client::Greeter;
use crate::codegen::helloworld::{HelloReply, HelloRequest};
use crate::codegen::poll_service::poll_service_client::PollService;
use crate::component::create_poll_form::CreatePollForm;

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

    let poll_kinds = use_async_with_options(
        async move {
            PollService::new(HOST.to_string())
                .poll_kinds(())
                .await
                .map_err(|_| "Failed to load poll kinds".to_string())
                .map(|response| response.kinds)
        },
        UseAsyncOptions::enable_auto(),
    );

    let kinds_async_data = poll_kinds.to_async_data();

    html! {
        <div>
            <h1>{ "Create" }</h1>
            <button {onclick}>{ "Run request" }</button>
            <h2>{"Result"}</h2>
            <pre>{content}</pre>
            <CreatePollForm poll_kinds={kinds_async_data} />
        </div>
    }
}
