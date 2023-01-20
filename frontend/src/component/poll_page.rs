use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PollPageProps {
    pub id: i32,
}

#[function_component(PollPage)]
pub fn poll_page(PollPageProps { id }: &PollPageProps) -> Html {
    html! {
      <div class="container mx-auto px-4 h-screen flex items-center">
        <div class="py-4">
          <h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight md:text-5xl lg:text-6xl">
            {"Poll "}{id.to_string()}{" page"}
          </h1>
        </div>
      </div>
    }
}
