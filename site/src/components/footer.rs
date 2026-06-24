use yew::prelude::*;
use yew_router::prelude::*;
use crate::models::{SiteConfig, SERVICES};
use crate::components::icons;
use crate::router::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="footer-container">
                <div class="footer-grid">
                    <div class="footer-brand">
                        <h3>{ "Milena Guedes" }</h3>
                        <p>{ "Massoterapia Integrativa" }</p>
                        <div class="footer-contact">
                            <a href={format!("https://wa.me/{}", SiteConfig::PHONE)}
                               target="_blank" rel="noopener noreferrer"
                               class="footer-whatsapp">
                                <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor">
                                    <path d={icons::WHATSAPP_SVG}/>
                                </svg>
                                { " (81) 9.9658-9013" }
                            </a>
                        </div>
                    </div>
                    <div class="footer-col">
                        <h4>{ "Serviços" }</h4>
                        <ul class="footer-service-list">
                            { SERVICES.iter().take(5).map(|s| {
                                html! {
                                    <li>
                                        <Link<Route> to={Route::Servicos}>
                                            { s.icon } { " " } { s.title }
                                        </Link<Route>>
                                    </li>
                                }
                            }).collect::<Html>() }
                        </ul>
                    </div>
                    <div class="footer-col">
                        <h4>{ "Links" }</h4>
                        <ul class="footer-nav-list">
                            <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Sobre}>{ "Sobre" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Servicos}>{ "Serviços" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Contato}>{ "Contato" }</Link<Route>></li>
                            <li><a href={SiteConfig::INSTAGRAM_URL} target="_blank" rel="noopener noreferrer">{ "Instagram" }</a></li>
                        </ul>
                    </div>
                </div>
                <div class="footer-copy">
                    <p>{ format!("© {} Milena Guedes — Massoterapia Integrativa. Todos os direitos reservados.", js_sys::Date::new_0().get_full_year()) }</p>
                </div>
            </div>
        </footer>
    }
}
