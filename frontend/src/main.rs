mod app;
mod codegen;
mod component;
mod router;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
