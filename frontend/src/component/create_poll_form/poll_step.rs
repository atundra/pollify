use yew::prelude::*;

use crate::{async_data::AsyncData, codegen::poll_service::PollKind, component::input::Input};

#[derive(Properties, PartialEq)]
pub struct PollKindProps {
    pub id: i32,
}

#[function_component(PollKindOption)]
fn poll_kind(PollKindProps { id }: &PollKindProps) -> Html {
    let name = match *id {
        0 => Some("First Past the Post"),
        1 => Some("Single Transferable Vote"),
        2 => Some("Additional Member System"),
        _ => None,
    };

    html! {
        <option key={*id} value={id.to_string()}>{name}</option>
    }
}

#[derive(Properties, PartialEq)]
pub struct PollStepProps {
    pub poll_kinds: AsyncData<Vec<PollKind>, String>,
    pub name: String,
    pub on_name_change: Callback<String>,
    pub slug: String,
    pub on_slug_change: Callback<String>,
}

#[function_component(PollStep)]
pub fn poll_step(props: &PollStepProps) -> Html {
    let kinds_select = match &props.poll_kinds {
        AsyncData::Loaded(kinds) => html! {
            <select class="select w-full select-bordered">
                <option disabled=true selected=true>{"Voting system"}</option>
                {kinds.into_iter().map(|PollKind { id }| {
                    html! { <PollKindOption id={id} /> }
                }).collect::<Html>()}
            </select>
        },
        _ => html! {
            <select class="select w-full select-bordered" disabled={true}>
                <option disabled=true selected=true>{"Voting system"}</option>
            </select>
        },
    };

    let on_name_change = props.on_name_change.clone();
    let on_slug_change = props.on_slug_change.clone();

    html! {
      <div class="space-y-4 mt-4 max-w-md m-auto">
          <div class="form-control">
              <Input
                typ="text"
                placeholder="Poll name"
                class="input w-full input-bordered"
                value={props.name.clone()}
                onchange={on_name_change}
              />
              <label class="label">
                  <span class="label-text">{"Publicly available name"}</span>
              </label>
          </div>
          <div class="form-control">
              {kinds_select}
              <label class="label">
                  <span class="label-text">{"Publicly available name"}</span>
              </label>
          </div>
          <div class="form-control">
              <Input
                typ="text"
                placeholder="Slug (optional)"
                class="input w-full input-bordered"
                value={props.slug.clone()}
                onchange={on_slug_change}
              />
              <label class="label">
                  <span class="label-text">{"The poll would be available at: "}<code>{"pollify.com/p/slug"}</code></span>
              </label>
          </div>
          <button class="btn btn-primary btn-block">{"Create poll"}</button>
      </div>
    }
}
