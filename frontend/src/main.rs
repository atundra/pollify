mod app;
mod component;
mod router;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
