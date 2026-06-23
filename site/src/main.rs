mod app;
mod components;
mod models;
mod pages;
mod router;
mod seo;

use app::App;

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
