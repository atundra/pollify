use crate::router::Router;
use crate::toast::ToastProvider;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ToastProvider>
            <Router />
        </ToastProvider>
    }
}
