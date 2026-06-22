mod app;
mod components;
mod models;
mod pages;
mod router;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
