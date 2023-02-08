use crate::{
    codegen::poll_service::{GetPollBySlugResponse, SubmitVoteRequest, VoteOption},
    hooks::use_poll_service::use_poll_service,
    poll_kind::poll_kind_id_to_label,
    toast::use_toast,
};
use js_sys::Date;
use prost_types::Timestamp;
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::use_local_storage;

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
pub struct PollFormProps {
    pub data: GetPollBySlugResponse,
    pub on_close: Callback<()>,
}

#[function_component(PollForm)]
pub fn poll_form(PollFormProps { data, on_close }: &PollFormProps) -> Html {
    let toast = use_toast().unwrap();

    let prev_vote_storage = use_local_storage::<String>(format!("ballot:{}", data.ballot_id));
    let selected_option = use_state_eq::<Option<String>, _>(|| (*prev_vote_storage).clone());

    let prev_vote_selected = (*prev_vote_storage)
        .clone()
        .zip((*selected_option).clone())
        .map(|(prev, current)| prev == current)
        .unwrap_or_default();

    let poll_service = use_poll_service();

    let rows = data
        .options
        .clone()
        .into_iter()
        .enumerate()
        .map(
            |(
                index,
                VoteOption {
                    title,
                    description,
                    id,
                },
            )| {
                let onchange = {
                    let id = id.clone();
                    let selected_option = selected_option.clone();
                    let finished = data.finished;
                    Callback::from(move |_| {
                      if finished {
                        return;
                      }

                      let id = id.clone();
                      let currently_checked = match &*selected_option {
                        Some(option) => *option == id,
                        None => false
                      };
                        if currently_checked {
                            selected_option.set(None);
                        } else {
                            selected_option.set(Some(id));
                        }
                    })
                };

                let checked = (*selected_option).clone().map(|selected| selected == id).unwrap_or(false);

                let description = description.unwrap_or_default();

                let checkbox_disabled = data.finished;

                html! {
                  <label key={id} class="flex gap-x-8 justify-between items-center">
                    <div class="text-4xl font-extrabold">{(index + 1).to_string()}</div>
                    <div class="grow space-y-2">
                      <h3 class="text-2xl font-bold">{title}</h3>
                      <div class="text-slate-500">{description}</div>
                    </div>
                    <div>
                      <input type="checkbox" class="checkbox" {checked} {onchange} disabled={checkbox_disabled} />
                    </div>
                  </label>
                }
            },
        )
        .enumerate()
        .flat_map(|(index, item)| {
            if index == 0 {
                vec![item]
            } else {
                vec![html! { <div class="divider"></div> }, item]
            }
        })
        .collect::<Html>();

    let submit_button_active = selected_option.is_some() && !prev_vote_selected;

    let submit_button_class = classes!(
        "btn",
        "btn-primary",
        "btn-wide",
        (!submit_button_active).then_some("btn-disabled"),
    );

    let submit_button_text = if data.finished {
        "Poll is closed"
    } else if submit_button_active {
        "Submit"
    } else if selected_option.is_none() {
        "Choose an option"
    } else {
        "Vote submited"
    };

    let on_submit = {
        let data = data.clone();
        Callback::from(move |_| {
            let ballot_id = data.ballot_id.clone();
            let toast = toast.clone();
            let poll_service = poll_service.clone();
            let prev_vote_storage = prev_vote_storage.clone();
            if let Some(option_id) = &*selected_option {
                let request = SubmitVoteRequest {
                    ballot_id,
                    option_id: option_id.clone(),
                    casted_at: Some(current_timestamp()),
                };

                let option_id = option_id.clone();

                spawn_local(async move {
                    let response = poll_service
                        .submit_vote(request)
                        .await
                        .map_err(|_err| "Failed to submit a vote");

                    prev_vote_storage.set(option_id);

                    match response {
                        Ok(_) => toast.success("Vote submited"),
                        Err(err) => toast.error(err),
                    }
                })
            }
        })
    };

    let on_close = {
        let on_close = on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let data = data.clone();

    html! {
      <div class="my-8 space-y-8 w-full">
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
