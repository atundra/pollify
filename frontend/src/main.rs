mod app;
mod async_data;
mod codegen;
mod component;
mod hooks;
mod router;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
