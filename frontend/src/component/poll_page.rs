use crate::{
    async_data::AsyncData,
    codegen::poll_service::{SubmitVoteRequest, VoteOption},
    hooks::{
        use_poll_by_slug::use_poll_by_slug, use_poll_service::use_poll_service,
        use_toast_on_async_data_error::use_toast_on_async_data_error,
    },
    poll_kind::poll_kind_id_to_label,
    toast::use_toast,
};
use js_sys::Date;
use prost_types::Timestamp;
use yew::{platform::spawn_local, prelude::*};

fn current_timestamp() -> Timestamp {
    let now = Date::new_0();
    Timestamp::date_time_nanos(
        now.get_utc_full_year().into(),
        (now.get_utc_month() + 1).try_into().unwrap(),
        now.get_utc_date().try_into().unwrap(),
        now.get_utc_hours().try_into().unwrap(),
        now.get_utc_minutes().try_into().unwrap(),
        now.get_utc_seconds().try_into().unwrap(),
        now.get_utc_milliseconds() * 1000,
    )
    .unwrap()
}

#[derive(Properties, PartialEq, Clone)]
pub struct PollPageProps {
    pub slug: String,
}

#[function_component(PollPage)]
pub fn poll_page(PollPageProps { slug }: &PollPageProps) -> Html {
    let data = use_poll_by_slug(slug.clone());
    use_toast_on_async_data_error(data.clone());

    let selected_option = use_state_eq::<Option<i32>, _>(Default::default);

    let toast = use_toast().unwrap();

    let on_close = Callback::noop();

    let poll_service = use_poll_service();

    let content = match data {
        AsyncData::Failed(_err) => html! { <h1>{"Error"}</h1> },
        AsyncData::Idle => html! {},
        AsyncData::Loading => {
            html! { <div class="flex items-center justify-center w-full"><progress class="progress progress-accent w-96"></progress></div> }
        }
        AsyncData::Loaded(data) => {
            let rows = data
                .options
                .clone()
                .into_iter()
                .enumerate()
                .map(|(index, VoteOption { title, description, id })| {
                  let onchange = {
                    let selected_option = selected_option.clone();
                    Callback::from(move |_| {
                      let currently_checked = selected_option.map(|option| option == id).unwrap_or(false);
                      if currently_checked {
                        selected_option.set(None);
                      } else {
                        selected_option.set(Some(id));
                      }
                    })
                  };

                  let checked = selected_option.clone().map(|selected| selected == id).unwrap_or(false);

                  html! {
                    <label key={id} class="flex gap-x-8 justify-between items-center">
                      <div class="text-4xl font-extrabold">{(index + 1).to_string()}</div>
                      <div class="grow space-y-2">
                        <h3 class="text-2xl font-bold">{title.to_string()}</h3>
                        <div class="text-slate-500">{description.unwrap_or_default().to_string()}</div>
                      </div>
                      <div>
                        <input type="checkbox" class="checkbox" {checked} {onchange} />
                      </div>
                    </label>
                  }
                })
                .enumerate()
                .flat_map(|(index, item)| {
                    if index == 0 {
                        vec![item]
                    } else {
                        vec![html! { <div class="divider"></div> }, item]
                    }
                })
                .collect::<Html>();

            let submit_button_class = classes!(
                "btn",
                "btn-primary",
                "btn-wide",
                if selected_option.is_none() {
                    Some("btn-disabled")
                } else {
                    None
                }
            );
            let submit_button_text = if selected_option.is_some() {
                "Submit"
            } else {
                "Choose an option"
            };

            let on_submit = {
                Callback::from(move |_| {
                    let toast = toast.clone();
                    let poll_service = poll_service.clone();
                    if let Some(option_id) = *selected_option {
                        spawn_local(async move {
                            let request = SubmitVoteRequest {
                                ballot_id: data.ballot_id,
                                option_id,
                                casted_at: Some(current_timestamp()),
                            };

                            let response = poll_service
                                .submit_vote(request)
                                .await
                                .map_err(|_err| "Failed to submit a vote");

                            match response {
                                Ok(_) => toast.success("Vote submited"),
                                Err(err) => toast.error(err),
                            }
                        })
                    }
                })
            };

            html! {
              <div class="my-8 space-y-8">
                <h1 class="mb-4 text-3xl font-bold leading-none tracking-tight md:text-5xl">{data.title}</h1>
                if let Some(kind) = data.kind.map(|kind| poll_kind_id_to_label(kind.id)) {
                  <p>{kind}</p>
                }
                <div>
                  {rows}
                </div>
                <div class="flex justify-between">
                  <div>
                    if !data.finished {
                      <button class="btn btn-outline" onclick={on_close}>{"Close the poll"}</button>
                    }
                  </div>
                  <button class={submit_button_class} onclick={on_submit}>{submit_button_text}</button>
                </div>
              </div>
            }
        }
    };

    html! {
      <div class="container mx-auto px-4 h-screen flex max-w-3xl">
          {content}
      </div>
    }
}
