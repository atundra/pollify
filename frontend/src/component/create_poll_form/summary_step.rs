use yew::prelude::*;

use crate::component::create_poll_form::poll_step::poll_kind_id_to_label;

use super::{FormData, VoteOption};

// use crate::component::input::Input;

// use super::VoteOption;

#[derive(Properties, PartialEq)]
pub struct SummaryStepProps {
    pub on_submit: Callback<()>,
    pub on_back: Callback<()>,
    pub data: FormData,
}

#[function_component(SummaryStep)]
pub fn summary_step(props: &SummaryStepProps) -> Html {
    // let submit_button_disabled =
    //     props.options.is_empty() || props.options.iter().all(|option| option.title.is_empty());
    // let submit_button_disabled_class = submit_button_disabled.then_some("btn-disabled");
    // let submit_button_classes =
    //     classes!("btn", "btn-primary", "px-8", submit_button_disabled_class);

    // let on_next = {
    //     let on_next = props.on_next.clone();
    //     Callback::from(move |_| {
    //         if !submit_button_disabled {
    //             on_next.emit(());
    //         }
    //     })
    // };

    // let on_back = {
    //     let on_back = props.on_prev.clone();
    //     Callback::from(move |_| {
    //         on_back.emit(());
    //     })
    // };

    let table_rows = props
        .data
        .options
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, VoteOption { title, description })| {
            html! {
              <tr>
                <th>{(index + 1).to_string()}</th>
                <td>{title.to_string()}</td>
                <td>{description.to_string()}</td>
              </tr>
            }
        })
        .collect::<Html>();

    let voting_system = props
        .data
        .voting_system
        .clone()
        .map(|system| poll_kind_id_to_label(system.parse().unwrap()))
        .unwrap();

    let on_submit = {
        let on_submit = props.on_submit.clone();
        Callback::from(move |_| {
            on_submit.emit(());
        })
    };

    let on_back = {
        let on_back = props.on_back.clone();
        Callback::from(move |_| {
            on_back.emit(());
        })
    };

    html! {
      <div class="space-y-8">
        <h2 class="text-2xl font-bold text-center">{props.data.name.clone()}</h2>
        <p class="text-xl text-center">{voting_system}</p>
        if !props.data.slug.clone().is_empty() {
          <p class="text-center">
            {"Will be available at "}<code>{"pollify.com/p/"}{props.data.slug.clone()}</code>
          </p>
        }
        <div class="overflow-x-auto">
          <table class="table w-full">
            <thead>
              <tr>
                <th></th>
                <th>{"Title"}</th>
                <th>{"Description"}</th>
              </tr>
            </thead>
            <tbody>
              {table_rows}
            </tbody>
          </table>
        </div>
        <div class="flex justify-between">
          <button class="btn btn-outline px-8" onclick={on_back}>{"Back"}</button>
          <button class="btn btn-primary px-8" onclick={on_submit}>{"Confirm"}</button>
        </div>
      </div>
    }
}
