mod ballot_step;
mod poll_step;
mod steps;
mod summary_step;

use std::ops::Deref;

use yew::prelude::*;

use crate::{
    async_data::AsyncData,
    codegen::poll_service::PollKind,
    component::create_poll_form::{ballot_step::BallotStep, poll_step::PollStep},
    component::create_poll_form::{steps::Steps, summary_step::SummaryStep},
};

#[derive(PartialEq, Clone, Copy, Default)]
pub enum CreatePollFormStep {
    #[default]
    Poll,
    Ballot,
    Summary,
}

#[derive(Default, PartialEq, Clone)]
pub struct VoteOption {
    pub title: String,
    pub description: String,
}

#[derive(PartialEq, Clone)]
pub struct FormData {
    pub name: String,
    pub voting_system: Option<String>,
    pub slug: String,
    pub options: Vec<VoteOption>,
}

impl Default for FormData {
    fn default() -> Self {
        Self {
            // name: Default::default(),
            name: "Vote for school president".to_string(),
            // voting_system: Default::default(),
            voting_system: Some("1".to_string()),
            slug: Default::default(),
            // slug: "yay-president".to_string(),
            // options: vec![VoteOption::default()],
            options: vec![
                VoteOption {
                    title: "Joe Mama".to_string(),
                    description: "You know this one".to_string(),
                },
                VoteOption {
                    title: "Joe Biden".to_string(),
                    description: "Make mmerica".to_string(),
                },
            ],
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CreatePollFormProps {
    pub poll_kinds: AsyncData<Vec<PollKind>, String>,
    pub on_create: Callback<FormData>,
}

#[function_component(CreatePollForm)]
pub fn create_poll_form(
    CreatePollFormProps {
        poll_kinds,
        on_create,
    }: &CreatePollFormProps,
) -> Html {
    let step = use_state_eq(CreatePollFormStep::default);
    let form_data = use_state(FormData::default);

    let on_name_change = {
        let form_data = form_data.clone();

        Callback::from(move |value: String| {
            let mut new_form_data = form_data.deref().clone();
            new_form_data.name = value;
            form_data.set(new_form_data);
        })
    };

    let on_voting_system_change: Callback<String> = {
        let form_data = form_data.clone();

        Callback::from(move |value| {
            let mut new_form_data = form_data.deref().clone();
            new_form_data.voting_system = Some(value);
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

    let on_go_to_poll: Callback<()> = {
        let step = step.clone();
        Callback::from(move |_| {
            step.set(CreatePollFormStep::Poll);
        })
    };

    let on_go_to_ballot: Callback<()> = {
        let step = step.clone();
        Callback::from(move |_| {
            step.set(CreatePollFormStep::Ballot);
        })
    };

    let on_go_to_summary = {
        let step = step.clone();
        Callback::from(move |_| {
            step.set(CreatePollFormStep::Summary);
        })
    };

    let on_add_option = {
        let form_data = form_data.clone();

        Callback::from(move |_| {
            let mut new_form_data = (*form_data).clone();
            let new_options = [new_form_data.options, vec![VoteOption::default()]].concat();
            new_form_data.options = new_options;
            form_data.set(new_form_data);
        })
    };

    let on_options_change = {
        let form_data = form_data.clone();

        Callback::from(move |options| {
            let mut new_form_data = (*form_data).clone();
            new_form_data.options = options;
            form_data.set(new_form_data);
        })
    };

    let summary_data = (*form_data).clone();

    let on_submit = {
        let form_data = (*form_data).clone();
        let on_create = on_create.clone();
        Callback::from(move |_| {
            on_create.emit(form_data.clone());
        })
    };

    html! {
        <div class="max-w-lg m-auto space-y-8">
            <Steps value={*step} />
            {match *step {
                CreatePollFormStep::Poll => html! {
                    <PollStep
                        poll_kinds={poll_kinds.clone()}
                        name={form_data.name.clone()}
                        {on_name_change}
                        slug={form_data.slug.clone()}
                        {on_slug_change}
                        voting_system={form_data.voting_system.clone()}
                        {on_voting_system_change}
                        on_next={on_go_to_ballot}
                    />
                },
                CreatePollFormStep::Ballot => html! {
                    <BallotStep
                        on_next={on_go_to_summary}
                        on_prev={on_go_to_poll}
                        on_add_option={on_add_option}
                        options={form_data.options.clone()}
                        on_options_change={on_options_change}
                    />
                },
                CreatePollFormStep::Summary => html! {
                    <SummaryStep
                        data={summary_data}
                        on_back={on_go_to_ballot}
                        on_submit={on_submit}
                    />
                },
            }}
        </div>
    }
}
