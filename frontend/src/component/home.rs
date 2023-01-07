use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div class="container mx-auto px-4 h-screen flex items-center">
        <div class="py-4">
          <h1 class="mb-4 text-4xl font-extrabold leading-none tracking-tight md:text-5xl lg:text-6xl">
            {"dzień dobry szanowni państwo"}
          </h1>
          <Link<Route> classes="btn btn-primary" to={Route::Create}>
            {"Create new poll"}
          </Link<Route>>
        </div>
      </div>
    }
}
