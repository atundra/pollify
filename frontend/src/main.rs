mod app;
mod async_data;
mod codegen;
mod component;
mod hooks;
mod poll_kind;
mod router;
mod toast;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
