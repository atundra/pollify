use crate::{
    async_data::AsyncData,
    codegen::poll_service::{GetPollBySlugResponse, PollResultItem},
    hooks::use_poll_result::use_poll_result,
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
    let content: Html = data
        .iter()
        .map(|item| {
            let vote_option = item.vote_option.clone().unwrap();
            html! {
              <div>
                <div>{vote_option.title}{" - "}{item.votes_count}</div>
                if let Some(description) = vote_option.description {
                  <div>{description}</div>
                }
              </div>
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
