use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
      <div class="container mx-auto px-4 h-screen flex items-center">
        <div class="py-4">
          <h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight md:text-5xl lg:text-6xl">
            { "Błąd 404" }
          </h1>
        </div>
      </div>
    }
}
