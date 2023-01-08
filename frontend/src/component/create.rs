use yew::platform::spawn_local;
use yew::prelude::*;

use crate::codegen::helloworld::greeter_client::Greeter;
use crate::codegen::helloworld::HelloRequest;

static HOST: &str = "https://localhost:8081";

#[function_component(Create)]
pub fn create() -> Html {
    let onclick = Callback::from(move |_| {
        spawn_local(async {
            Greeter::new(HOST.to_string())
                .say_hello(HelloRequest {
                    name: "hi".to_string(),
                })
                .await
                .unwrap();
        })
    });
    html! {
        <div>
            <h1>{ "Create" }</h1>
            <button {onclick}>{ "Run request" }</button>
        </div>
    }
}
