use crate::{hooks::use_poll_service::use_poll_service, router::Route, toast::use_toast};
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq, Clone)]
pub struct PollPageProps {
    pub slug: String,
}

#[function_component(PollPage)]
pub fn poll_page(PollPageProps { slug }: &PollPageProps) -> Html {
    let _poll_service = use_poll_service();

    let _toast = use_toast().unwrap();

    html! {
      <div class="container mx-auto px-4 h-screen flex items-center">
        <div class="py-4">
          <h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight md:text-5xl lg:text-6xl">
            {"Poll "}{slug}{" page"}
            <Link<Route> classes="btn btn-primary" to={Route::Create}>
              {"Create new poll"}
            </Link<Route>>
          </h1>
        </div>
      </div>
    }
}
