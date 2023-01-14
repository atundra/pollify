mod poll_step;
mod steps;

use std::ops::Deref;

use yew::prelude::*;

use crate::{
    async_data::AsyncData, codegen::poll_service::PollKind,
    component::create_poll_form::poll_step::PollStep, component::create_poll_form::steps::Steps,
};

#[derive(PartialEq, Clone, Copy)]
pub enum CreatePollFormStep {
    Poll,
    Ballot,
    Confirm,
}

#[derive(Default, PartialEq, Clone)]
struct FormData {
    name: String,
    voting_system: i32,
    slug: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CreatePollFormProps {
    pub poll_kinds: AsyncData<Vec<PollKind>, String>,
}

#[function_component(CreatePollForm)]
pub fn create_poll_form(CreatePollFormProps { poll_kinds }: &CreatePollFormProps) -> Html {
    let step = use_state_eq(|| CreatePollFormStep::Poll);
    let form_data = use_state(FormData::default);

    let on_name_change = {
        let form_data = form_data.clone();

        Callback::from(move |value: String| {
            let mut new_form_data = form_data.deref().clone();
            new_form_data.name = value;
            form_data.set(new_form_data);
        })
    };

    let on_slug_change = {
        let form_data = form_data.clone();

        Callback::from(move |value: String| {
            let mut new_form_data = (*form_data).clone();
            new_form_data.slug = value;
            form_data.set(new_form_data);
        })
    };

    let current_step = match *step {
        CreatePollFormStep::Poll => {
            html! {
                <PollStep
                    poll_kinds={poll_kinds.clone()}
                    name={form_data.name.clone()}
                    {on_name_change}
                    slug={form_data.name.clone()}
                    {on_slug_change}
                />
            }
        }
        _ => html! { <div>{"234"}</div> },
    };

    let on_step_change: Callback<CreatePollFormStep> = {
        let step = step.clone();
        Callback::from(move |new_step| {
            step.set(new_step);
        })
    };

    html! {
        <div>
            <Steps value={*step} onchange={on_step_change} />
            {current_step}
        </div>
    }
}
