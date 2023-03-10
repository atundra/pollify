use yew::prelude::*;
use yew_router::prelude::*;

use crate::component::{create_page::Create, home::Home, not_found::NotFound, poll_page::PollPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/create")]
    Create,

    #[at("/p/:slug")]
    PollPage { slug: String },

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Create => html! { <Create /> },
        Route::NotFound => html! { <NotFound /> },
        Route::PollPage { slug } => html! { <PollPage {slug} /> },
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
