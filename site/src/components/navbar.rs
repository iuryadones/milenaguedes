use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let collapsed = use_state(|| true);
    let scrolled = use_state(|| false);
    let navigator = use_navigator().unwrap();

    {
        let scrolled = scrolled.clone();
        use_effect_with((), move |_| {
            let window = window().unwrap();
            let window2 = window.clone();
            let scrolled_clone = scrolled.clone();
            let closure = Closure::<dyn Fn()>::new(move || {
                let y = window.scroll_y().unwrap_or(0.0);
                scrolled_clone.set(y > 20.0);
            });
            let callback = closure.as_ref().unchecked_ref();
            let _ = window2.set_onscroll(Some(callback));
            move || {
                window2.set_onscroll(None);
                drop(closure);
            }
        });
    }

    let toggle_nav = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| collapsed.set(!*collapsed))
    };

    let route = use_route::<Route>().unwrap_or(Route::Home);

    let is_active = |r: &Route| -> Classes {
        if &route == r { Classes::from("active") } else { Classes::new() }
    };

    let make_nav_link = |target: Route, label: &'static str| -> Html {
        let target_clone = target.clone();
        let navigator = navigator.clone();
        let collapsed = collapsed.clone();
        let onclick = Callback::from(move |_| {
            collapsed.set(true);
            navigator.push(&target_clone);
        });
        html! {
            <a href={target.to_path()} onclick={onclick} class={is_active(&target)}>
                { label }
            </a>
        }
    };

    html! {
        <nav class={classes!("navbar", (*scrolled).then(|| "scrolled"))}>
            <div class="navbar-container">
                <a href="/" onclick={
                    let navigator = navigator.clone();
                    Callback::from(move |e: MouseEvent| { e.prevent_default(); navigator.push(&Route::Home); })
                } class="navbar-logo">
                    { "Milena Guedes" }
                </a>
                <button class="navbar-hamburger" onclick={toggle_nav} aria-label="Menu">
                    <span></span>
                    <span></span>
                    <span></span>
                </button>
                <ul class={classes!("navbar-links", (*collapsed).then(|| "collapsed"))}>
                    <li>{ make_nav_link(Route::Home, "Home") }</li>
                    <li>{ make_nav_link(Route::Sobre, "Sobre") }</li>
                    <li>{ make_nav_link(Route::Servicos, "Serviços") }</li>
                    <li>{ make_nav_link(Route::Contato, "Contato") }</li>
                    <li class="navbar-whatsapp-item">
                        <a href={format!("https://wa.me/{}", crate::models::SiteConfig::PHONE)}
                           target="_blank" rel="noopener noreferrer"
                           class="btn btn-primary btn-sm">
                            { "WhatsApp" }
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
