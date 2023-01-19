use yew::prelude::*;

use crate::{
    async_data::AsyncData,
    codegen::poll_service::PollKind,
    component::{
        input::Input,
        select::{Select, SelectItem},
    },
};

#[derive(Properties, PartialEq)]
pub struct PollKindProps {
    pub id: i32,
}

fn poll_kind_id_to_label(id: i32) -> Option<String> {
    match id {
        0 => Some("First Past the Post".to_string()),
        1 => Some("Single Transferable Vote".to_string()),
        2 => Some("Additional Member System".to_string()),
        _ => None,
    }
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
    pub voting_system: Option<String>,
    pub on_voting_system_change: Callback<String>,
    pub on_next: Callback<()>,
}

#[function_component(PollStep)]
pub fn poll_step(props: &PollStepProps) -> Html {
    let kinds_select = match &props.poll_kinds {
        AsyncData::Loaded(kinds) => {
            let items: Vec<SelectItem> = kinds
                .iter()
                .map(|poll_kind| -> SelectItem {
                    SelectItem {
                        value: poll_kind.id.to_string(),
                        label: poll_kind_id_to_label(poll_kind.id)
                            .unwrap_or_else(|| "Unsupported kind".to_string()),
                        disabled: false,
                    }
                })
                .collect();

            let voting_system = props.voting_system.clone();
            let on_voting_system_change = props.on_voting_system_change.clone();

            html! {
                <Select
                    class="select w-full select-bordered"
                    value={voting_system}
                    onchange={on_voting_system_change}
                    {items}
                    placeholder={Some("Voting system")}
                />
            }
        }
        _ => html! {
            <select class="select w-full select-bordered" disabled={true}>
                <option disabled=true selected=true>{"Voting system"}</option>
            </select>
        },
    };

    let on_name_change = props.on_name_change.clone();
    // let on_voting_system_change = props.on_voting_system_change.clone();
    let on_slug_change = props.on_slug_change.clone();

    let submit_button_disabled = props.name.is_empty() || props.voting_system.is_none();
    let submit_button_disabled_class = submit_button_disabled.then_some("btn-disabled");
    let submit_button_classes = classes!(
        "btn",
        "btn-primary",
        "btn-block",
        submit_button_disabled_class
    );

    let on_next = {
        let on_next = props.on_next.clone();
        Callback::from(move |_| {
            if !submit_button_disabled {
                on_next.emit(());
            }
        })
    };

    html! {
      <div class="space-y-4">
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
          <button class={submit_button_classes} onclick={on_next}>{"Next"}</button>
      </div>
    }
}
