use yew::prelude::*;

use super::CreatePollFormStep;

#[derive(Properties, PartialEq)]
pub struct StepsProps {
    pub value: CreatePollFormStep,
}

#[function_component(Steps)]
pub fn steps(StepsProps { value }: &StepsProps) -> Html {
    let poll_ballot_active_class = match value {
        CreatePollFormStep::Ballot | CreatePollFormStep::Summary => Some("step-primary"),
        _ => None,
    };

    let poll_finish_active_class = match value {
        CreatePollFormStep::Summary => Some("step-primary"),
        _ => None,
    };

    html! {
        <ul class="steps w-full">
          <li class="step step-primary">{"Poll"}</li>
          <li class={classes!("step", poll_ballot_active_class)}>{"Ballot"}</li>
          <li class={classes!("step", poll_finish_active_class)}>{"Finish"}</li>
        </ul>
    }
}
