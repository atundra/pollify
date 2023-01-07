use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Create)]
pub fn create() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
