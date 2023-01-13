mod steps;

use yew::prelude::*;

use crate::component::create_poll_form::steps::Steps;

#[derive(PartialEq, Clone, Copy)]
pub enum CreatePollFormStep {
    Poll,
    Ballot,
    Confirm,
}

#[function_component(CreatePollForm)]
pub fn create_poll_form() -> Html {
    let step = use_state_eq(|| CreatePollFormStep::Poll);

    let current_step = match *step {
        CreatePollFormStep::Poll => html! {
            <div>{"123"}</div>
        },
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
