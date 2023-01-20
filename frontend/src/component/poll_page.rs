use crate::hooks::use_poll_service::use_poll_service;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PollPageProps {
    pub slug: String,
}

#[function_component(PollPage)]
pub fn poll_page(PollPageProps { slug }: &PollPageProps) -> Html {
    let _poll_service = use_poll_service();

    html! {
      <div class="container mx-auto px-4 h-screen flex items-center">
        <div class="py-4">
          <h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight md:text-5xl lg:text-6xl">
            {"Poll "}{slug}{" page"}
          </h1>
        </div>
      </div>
    }
}
