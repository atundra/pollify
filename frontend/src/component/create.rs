use yew::platform::spawn_local;
use yew::prelude::*;

use crate::codegen::helloworld::greeter_client::Greeter;
use crate::codegen::helloworld::{HelloReply, HelloRequest};

static HOST: &str = "http://localhost:50051";

#[function_component(Create)]
pub fn create() -> Html {
    let state = use_state(|| "No response yet".to_string());

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            let state = state.clone();

            spawn_local(async move {
                let HelloReply { message } = Greeter::new(HOST.to_string())
                    .say_hello(HelloRequest {
                        name: "world".to_string(),
                    })
                    .await
                    .unwrap();

                state.set(message);
            })
        })
    };

    html! {
        <div>
            <h1>{ "Create" }</h1>
            <button {onclick}>{ "Run request" }</button>
            <h2>{"Result"}</h2>
            <pre>{&*state}</pre>
        </div>
    }
}
