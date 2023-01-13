use yew::prelude::*;

use super::CreatePollFormStep;

#[derive(Properties, PartialEq)]
pub struct StepsProps {
    pub value: CreatePollFormStep,
    pub onchange: Callback<CreatePollFormStep>,
}

#[function_component(Steps)]
pub fn steps(StepsProps { value, onchange }: &StepsProps) -> Html {
    let poll_ballot_active_class = match value {
        CreatePollFormStep::Ballot | CreatePollFormStep::Confirm => Some("step-primary"),
        _ => None,
    };

    let poll_finish_active_class = match value {
        CreatePollFormStep::Confirm => Some("step-primary"),
        _ => None,
    };

    let on_confirm_step = {
        let onchange = onchange.clone();
        Callback::from(move |_| {
            onchange.emit(CreatePollFormStep::Confirm);
        })
    };

    let on_poll_step = {
        let onchange = onchange.clone();
        Callback::from(move |_| {
            onchange.emit(CreatePollFormStep::Poll);
        })
    };

    let on_ballot_step = {
        let onchange = onchange.clone();
        Callback::from(move |_| {
            onchange.emit(CreatePollFormStep::Ballot);
        })
    };

    html! {
        <ul class="steps">
          <li class="step step-primary cursor-pointer" onclick={on_poll_step}>{"Poll"}</li>
          <li class={classes!("step", "cursor-pointer", poll_ballot_active_class)} onclick={on_ballot_step}>{"Ballot"}</li>
          <li class={classes!("step", "cursor-pointer", poll_finish_active_class)} onclick={on_confirm_step}>{"Finish"}</li>
        </ul>
    }
}
