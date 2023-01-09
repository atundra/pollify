use yew::prelude::*;
use yew_hooks::use_async;

use crate::async_data::{AsyncData, ToAsyncData};
use crate::codegen::helloworld::greeter_client::Greeter;
use crate::codegen::helloworld::{HelloReply, HelloRequest};

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

    html! {
        <div>
            <h1>{ "Create" }</h1>
            <button {onclick}>{ "Run request" }</button>
            <h2>{"Result"}</h2>
            <pre>{content}</pre>
        </div>
    }
}
