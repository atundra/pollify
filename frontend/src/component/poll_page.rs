use crate::{
    async_data::AsyncData,
    codegen::poll_service::ClosePollRequest,
    component::poll_form::PollForm,
    hooks::{
        use_poll_by_slug::use_poll_by_slug, use_poll_service::use_poll_service,
        use_toast_on_async_data_error::use_toast_on_async_data_error,
    },
    toast::use_toast,
};
use yew::{platform::spawn_local, prelude::*};

#[derive(Properties, PartialEq, Clone)]
pub struct PollPageProps {
    pub slug: String,
}

#[function_component(PollPage)]
pub fn poll_page(PollPageProps { slug }: &PollPageProps) -> Html {
    let handle = use_poll_by_slug(slug.clone());
    let data = (*handle).clone();
    use_toast_on_async_data_error(data.clone());

    let toast = use_toast().unwrap();
    let poll_service = use_poll_service();

    let content = match data {
        AsyncData::Failed(_err) => {
            html! { <h1 class="text-2xl font-extrabold">{"Unable to load poll"}</h1> }
        }
        AsyncData::Idle => html! {},
        AsyncData::Loading => {
            html! { <div class="flex items-center justify-center w-full"><progress class="progress progress-accent w-96"></progress></div> }
        }
        AsyncData::Loaded(data) => {
            let ballot_id = data.clone().ballot_id;
            let on_close = {
                Callback::from(move |_| {
                    let handle = handle.clone();
                    let ballot_id = ballot_id.clone();
                    let request = ClosePollRequest { ballot_id };
                    let toast = toast.clone();
                    let poll_service = poll_service.clone();

                    spawn_local(async move {
                        let response = poll_service
                            .close_poll(request)
                            .await
                            .map_err(|_err| "Failed to close the poll");

                        match response {
                            Ok(_) => {
                                toast.success("Poll closed");
                                handle.run();
                            }
                            Err(err) => toast.error(err),
                        }
                    })
                })
            };

            html! {
                <PollForm {data} {on_close} />
            }
        }
    };

    html! {
      <div class="container mx-auto px-4 min-h-screen items-center flex max-w-3xl">
          {content}
      </div>
    }
}
