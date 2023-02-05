use crate::{
    async_data::AsyncData,
    component::poll_form::PollForm,
    hooks::{
        use_poll_by_slug::use_poll_by_slug,
        use_toast_on_async_data_error::use_toast_on_async_data_error,
    },
};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PollPageProps {
    pub slug: String,
}

#[function_component(PollPage)]
pub fn poll_page(PollPageProps { slug }: &PollPageProps) -> Html {
    let data = use_poll_by_slug(slug.clone());
    use_toast_on_async_data_error(data.clone());

    let content = match data {
        AsyncData::Failed(_err) => {
            html! { <h1 class="text-2xl font-extrabold">{"Unable to load poll"}</h1> }
        }
        AsyncData::Idle => html! {},
        AsyncData::Loading => {
            html! { <div class="flex items-center justify-center w-full"><progress class="progress progress-accent w-96"></progress></div> }
        }
        AsyncData::Loaded(data) => html! { <PollForm {data} /> },
    };

    html! {
      <div class="container mx-auto px-4 min-h-screen items-center flex max-w-3xl">
          {content}
      </div>
    }
}
