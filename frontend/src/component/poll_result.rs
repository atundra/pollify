use crate::{
    async_data::AsyncData,
    codegen::poll_service::{GetPollBySlugResponse, PollResultItem},
    hooks::{
        use_poll_result::use_poll_result,
        use_toast_on_async_data_error::use_toast_on_async_data_error,
    },
    poll_kind::poll_kind_id_to_label,
};
use chrono::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PollResultTableProps {
    pub data: Vec<PollResultItem>,
}

#[function_component(PollResultTable)]
pub fn poll_results_table(PollResultTableProps { data }: &PollResultTableProps) -> Html {
    let total_votes: i32 = data.iter().map(|item| item.votes_count).sum();
    let content: Html = data
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let progress_classes = classes!(
                "progress",
                "w-full",
                match index {
                    0 => "progress-primary",
                    _ => "progress-secondary",
                }
            );
            let ratio = item.votes_count as f32 / total_votes as f32;
            let vote_option = item.vote_option.clone().unwrap();

            // Pretty print percent value
            let percent = format!("{}", ((ratio * 10000.0).round() as i32) as f32 / 100.0);

            let votes_string = match item.votes_count {
                1 => " vote",
                _ => " votes",
            };

            let progress_value = (ratio * 100.0).to_string();

            html! {
              <div class="flex items-center gap-x-4">
                <div class="basis-1/2 space-y-2">
                  <div class="text-2xl font-bold">{vote_option.title}</div>
                  if let Some(description) = vote_option.description {
                    <div class="text-slate-500">{description}</div>
                  }
                </div>
                <div class="basis-1/2">
                  <div class="flex justify-between">
                    <div>{percent}{"%"}</div>
                    <div>{item.votes_count}{votes_string}</div>
                  </div>

                  <progress class={progress_classes} value={progress_value} max="100"></progress>
                </div>
              </div>
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
        .collect();

    html! {
      <div>
        {content}
      </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct PollResultProps {
    pub poll: GetPollBySlugResponse,
}

#[function_component(PollResult)]
pub fn poll_result(PollResultProps { poll }: &PollResultProps) -> Html {
    let poll = poll.clone();
    let result_items = use_poll_result(poll.id);
    use_toast_on_async_data_error(result_items.clone());

    let content = match result_items {
        AsyncData::Idle => html! { <div>{"Loading"}</div> },
        AsyncData::Loading => html! { <div>{"Loading"}</div> },
        AsyncData::Failed(err) => html! { <div>{"Error: "}{err}</div> },
        AsyncData::Loaded(data) => html! { <PollResultTable {data} /> },
    };

    let finished_at = poll
        .finished_at
        .map(|date| Utc.timestamp_opt(date.seconds, 0).unwrap())
        .map(|date| format!("{}", date.format("%e %b %Y at %H:%M")));

    html! {
      <div class="my-8 space-y-8 w-full">
        <h1 class="mb-4 text-3xl font-bold leading-none tracking-tight md:text-5xl">{poll.title}</h1>
        if let Some(kind) = poll.kind.map(|kind| poll_kind_id_to_label(kind.id)) {
          <p>{kind}</p>
        }
        if let Some(finished_at) = finished_at {
          <p>{"Finished "}{finished_at}</p>
        }
        <div>
          {content}
        </div>
      </div>
    }
}
