use yew::prelude::*;

use crate::component::input::Input;

use super::VoteOption;

#[derive(Properties, PartialEq)]
pub struct BallotStepProps {
    pub on_next: Callback<()>,
    pub on_prev: Callback<()>,
    pub on_add_option: Callback<()>,
    pub options: Vec<VoteOption>,
    pub on_options_change: Callback<Vec<VoteOption>>,
}

#[function_component(BallotStep)]
pub fn ballot_step(props: &BallotStepProps) -> Html {
    let submit_button_disabled =
        props.options.is_empty() || props.options.iter().all(|option| option.title.is_empty());
    let submit_button_disabled_class = submit_button_disabled.then_some("btn-disabled");
    let submit_button_classes =
        classes!("btn", "btn-primary", "px-8", submit_button_disabled_class);

    let on_next = {
        let on_next = props.on_next.clone();
        Callback::from(move |_| {
            if !submit_button_disabled {
                on_next.emit(());
            }
        })
    };

    let on_back = {
        let on_back = props.on_prev.clone();
        Callback::from(move |_| {
            on_back.emit(());
        })
    };

    let on_add_option = {
        let on_add_option = props.on_add_option.clone();
        Callback::from(move |_| {
            on_add_option.emit(());
        })
    };

    html! {
        <div class="space-y-4">
            {props.options.clone().into_iter().enumerate().map(|(index, VoteOption { title, description })| {
              let options = props.options.clone();
              let on_title_change: Callback<String> = {
                let on_options_change = props.on_options_change.clone();
                Callback::from(move |value| {
                  let mut options = options.clone();
                  let item = &mut options[index];
                  item.title = value;
                  on_options_change.emit(options);
                })
              };

              let options = props.options.clone();
              let on_description_change: Callback<String> = {
                let on_options_change = props.on_options_change.clone();
                Callback::from(move |value| {
                  let mut options = options.clone();
                  let item = &mut options[index];
                  item.description = value;
                  on_options_change.emit(options);
                })
              };

              let options = props.options.clone();
              let on_remove = {
                let on_options_change = props.on_options_change.clone();
                Callback::from(move |_| {
                  let mut options = options.clone();
                  options.remove(index);
                  on_options_change.emit(options);
                })
              };

              html! {
                  <div class="space-y-4 border border-neutral-600 rounded-box p-2">
                      <div class="form-control">
                        <div class="flex gap-2">
                            <Input
                              typ="text"
                              placeholder="Title"
                              class="input w-full input-bordered"
                              value={title}
                              onchange={on_title_change}
                            />
                            <button class="btn btn-outline btn-secondary" onclick={on_remove}>
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" class="w-6 h-6">
                                    <path fill="currentColor" d="M268 416h24a12 12 0 0 0 12-12V188a12 12 0 0 0-12-12h-24a12 12 0 0 0-12 12v216a12 12 0 0 0 12 12zM432 80h-82.41l-34-56.7A48 48 0 0 0 274.41 0H173.59a48 48 0 0 0-41.16 23.3L98.41 80H16A16 16 0 0 0 0 96v16a16 16 0 0 0 16 16h16v336a48 48 0 0 0 48 48h288a48 48 0 0 0 48-48V128h16a16 16 0 0 0 16-16V96a16 16 0 0 0-16-16zM171.84 50.91A6 6 0 0 1 177 48h94a6 6 0 0 1 5.15 2.91L293.61 80H154.39zM368 464H80V128h288zm-212-48h24a12 12 0 0 0 12-12V188a12 12 0 0 0-12-12h-24a12 12 0 0 0-12 12v216a12 12 0 0 0 12 12z"/>
                                </svg>
                            </button>
                        </div>
                      </div>
                      <div class="form-control">
                          <Input
                              typ="text"
                              placeholder="Description"
                              class="input w-full input-bordered"
                              value={description}
                              onchange={on_description_change}
                          />
                      </div>
                  </div>
              }
            }).collect::<Html>()}
            <button class="btn btn-outline btn-block" onclick={on_add_option}>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" class="w-4 h-4 mr-1">
                <path fill="currentColor" d="M416 208H272V64c0-17.67-14.33-32-32-32h-32c-17.67 0-32 14.33-32 32v144H32c-17.67 0-32 14.33-32 32v32c0 17.67 14.33 32 32 32h144v144c0 17.67 14.33 32 32 32h32c17.67 0 32-14.33 32-32V304h144c17.67 0 32-14.33 32-32v-32c0-17.67-14.33-32-32-32z"/>
              </svg>{"Add option"}</button>
            <div class="flex justify-between">
              <button class="btn btn-outline px-8" onclick={on_back}>{"Back"}</button>
              <button class={submit_button_classes} onclick={on_next}>{"Next"}</button>
            </div>
        </div>
    }
}
