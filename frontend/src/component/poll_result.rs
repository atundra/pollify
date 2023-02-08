use crate::{
    codegen::poll_service::GetPollBySlugResponse, hooks::use_poll_result::use_poll_result,
    poll_kind::poll_kind_id_to_label,
};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PollResultProps {
    pub poll: GetPollBySlugResponse,
}

#[function_component(PollResult)]
pub fn poll_result(PollResultProps { poll }: &PollResultProps) -> Html {
    let poll = poll.clone();
    let _result_items = use_poll_result(poll.id);

    html! {
      <div class="my-8 space-y-8 w-full">
        <h1 class="mb-4 text-3xl font-bold leading-none tracking-tight md:text-5xl">{poll.title}</h1>
        if let Some(kind) = poll.kind.map(|kind| poll_kind_id_to_label(kind.id)) {
          <p>{kind}</p>
        }
        <div>
        //   {rows}
        </div>
      </div>
    }
}
